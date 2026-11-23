//! Serde helper for fixed-size arrays longer than 32 elements.
//!
//! Serde only implements `Serialize`/`Deserialize` for arrays up to 32
//! elements. IDL fixed arrays (e.g. `double[36]` covariance matrices) are
//! encoded in CDR as a plain sequence of elements without a length prefix,
//! which is exactly how serde tuples are encoded, so we (de)serialize them as
//! tuples.

use std::{fmt, marker::PhantomData};

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error, SeqAccess, Visitor},
    ser::SerializeTuple,
};

pub fn serialize<S, T, const N: usize>(value: &[T; N], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    let mut tuple = serializer.serialize_tuple(N)?;
    for element in value {
        tuple.serialize_element(element)?;
    }
    tuple.end()
}

pub fn deserialize<'de, D, T, const N: usize>(deserializer: D) -> Result<[T; N], D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default + Copy,
{
    struct ArrayVisitor<T, const N: usize>(PhantomData<T>);

    impl<'de, T, const N: usize> Visitor<'de> for ArrayVisitor<T, N>
    where
        T: Deserialize<'de> + Default + Copy,
    {
        type Value = [T; N];

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "an array of length {N}")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut out = [T::default(); N];
            for (index, slot) in out.iter_mut().enumerate() {
                *slot = seq
                    .next_element()?
                    .ok_or_else(|| A::Error::invalid_length(index, &self))?;
            }
            Ok(out)
        }
    }

    deserializer.deserialize_tuple(N, ArrayVisitor::<T, N>(PhantomData))
}
