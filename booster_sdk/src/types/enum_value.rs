//! Forward-compatible wire representation of IDL enums.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// An IDL enum field as received on the wire.
///
/// IDL enums are 32-bit values in CDR. Decoding them straight into a Rust
/// enum would make a value added by newer robot firmware fail deserialization
/// and drop the whole sample, so received messages keep the raw value and
/// decode on access:
///
/// - [`get`](Self::get) returns the typed value, or `None` if it is unknown;
/// - [`raw`](Self::raw) returns the integer as sent.
///
/// Serializes exactly like the C++ enum (a plain `u32`).
pub struct EnumValue<E> {
    raw: u32,
    _marker: PhantomData<fn() -> E>,
}

impl<E> EnumValue<E> {
    /// Wrap a raw wire value.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// The raw wire value.
    #[must_use]
    pub const fn raw(self) -> u32 {
        self.raw
    }
}

impl<E: TryFrom<i32>> EnumValue<E> {
    /// The typed value, or `None` if this SDK does not know it.
    #[must_use]
    pub fn get(self) -> Option<E> {
        i32::try_from(self.raw)
            .ok()
            .and_then(|value| E::try_from(value).ok())
    }
}

impl<E: Into<i32>> From<E> for EnumValue<E> {
    fn from(value: E) -> Self {
        // IDL enums are unsigned 32-bit on the wire; reinterpret the bits.
        Self::from_raw(u32::from_ne_bytes(value.into().to_ne_bytes()))
    }
}

impl<E> Clone for EnumValue<E> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<E> Copy for EnumValue<E> {}

impl<E> Default for EnumValue<E> {
    /// The first IDL enumerator (value 0).
    fn default() -> Self {
        Self::from_raw(0)
    }
}

impl<E> PartialEq for EnumValue<E> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<E> Eq for EnumValue<E> {}

impl<E> Hash for EnumValue<E> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

impl<E: TryFrom<i32> + fmt::Debug> fmt::Debug for EnumValue<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.get() {
            Some(value) => value.fmt(f),
            None => write!(f, "Unknown({})", self.raw),
        }
    }
}

impl<E> Serialize for EnumValue<E> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u32(self.raw)
    }
}

impl<'de, E> Deserialize<'de> for EnumValue<E> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        u32::deserialize(deserializer).map(Self::from_raw)
    }
}

impl<E: TryFrom<i32> + PartialEq> PartialEq<E> for EnumValue<E> {
    fn eq(&self, other: &E) -> bool {
        self.get().is_some_and(|value| value == *other)
    }
}

#[cfg(test)]
mod tests {
    use super::EnumValue;
    use crate::types::FallDownStateType;

    #[test]
    fn known_and_unknown_values() {
        let known = EnumValue::<FallDownStateType>::from(FallDownStateType::HasFallen);
        assert_eq!(known.get(), Some(FallDownStateType::HasFallen));
        assert_eq!(known, FallDownStateType::HasFallen);

        let unknown = EnumValue::<FallDownStateType>::from_raw(1234);
        assert_eq!(unknown.get(), None);
        assert_eq!(unknown.raw(), 1234);
        assert_eq!(format!("{unknown:?}"), "Unknown(1234)");
    }

    #[test]
    fn unknown_value_round_trips_through_json() {
        let value: EnumValue<FallDownStateType> = serde_json::from_str("77").unwrap();
        assert_eq!(value.raw(), 77);
        assert_eq!(serde_json::to_string(&value).unwrap(), "77");
    }
}
