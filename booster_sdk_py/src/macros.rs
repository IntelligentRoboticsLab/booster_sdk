//! Declarative helpers shared by the binding modules.

/// Bind a Rust integer-backed enum as a Python class with one class attribute
/// per variant (the convention used throughout these bindings).
///
/// `$repr` is the enum's integer representation (`i32` for `api_id_enum!`
/// enums, `u32` for the device-catalog wire enums).
macro_rules! py_int_enum {
    (
        $py_name:ident, $public_name:literal, $inner:ty, $repr:ty,
        { $($attr:ident => $variant:ident),+ $(,)? }
    ) => {
        #[pyo3::pyclass(module = "booster_sdk_bindings", name = $public_name, eq, hash, frozen)]
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $py_name(pub(crate) $inner);

        #[pyo3::pymethods]
        impl $py_name {
            $(
                #[classattr]
                const $attr: Self = Self(<$inner>::$variant);
            )+

            /// Convert a raw wire value; raises `ValueError` for unknown values.
            #[staticmethod]
            fn from_int(value: $repr) -> pyo3::PyResult<Self> {
                <$inner>::try_from(value).map(Self).map_err(|_| {
                    pyo3::exceptions::PyValueError::new_err(format!(
                        concat!("invalid ", $public_name, " value: {}"),
                        value
                    ))
                })
            }

            #[getter]
            fn name(&self) -> &'static str {
                match self.0 {
                    $(<$inner>::$variant => stringify!($attr),)+
                }
            }

            fn __int__(&self) -> $repr {
                <$repr>::from(self.0)
            }

            fn __index__(&self) -> $repr {
                <$repr>::from(self.0)
            }

            fn __repr__(&self) -> String {
                format!(concat!($public_name, ".{}"), self.name())
            }
        }

        impl From<$py_name> for $inner {
            fn from(value: $py_name) -> Self {
                value.0
            }
        }

        impl From<$inner> for $py_name {
            fn from(value: $inner) -> Self {
                Self(value)
            }
        }
    };
}

/// Bind a flat parameter type as a mutable Python class: one
/// `#[pyo3(get, set)]` field per Rust field, a keyword constructor with the
/// given defaults, `to_json()` / `from_json()`, and `to_rust()` /
/// `from_rust()` conversions (`.into()` per field).
macro_rules! py_param_class {
    (
        $py_name:ident, $public_name:literal, $inner:ty,
        { $($field:ident : $fty:ty = $default:expr),+ $(,)? }
        $(, { $($extra:tt)* })?
    ) => {
        #[pyo3::pyclass(module = "booster_sdk_bindings", name = $public_name, get_all, set_all)]
        #[derive(Clone)]
        pub struct $py_name {
            $(pub(crate) $field: $fty,)+
        }

        #[allow(dead_code, clippy::useless_conversion, clippy::clone_on_copy)]
        impl $py_name {
            pub(crate) fn to_rust(&self) -> $inner {
                <$inner>::from(self.clone())
            }

            pub(crate) fn from_rust(value: $inner) -> Self {
                Self {
                    $($field: value.$field.into(),)+
                }
            }
        }

        #[allow(clippy::useless_conversion, clippy::clone_on_copy)]
        impl From<$py_name> for $inner {
            fn from(value: $py_name) -> Self {
                Self {
                    $($field: value.$field.into(),)+
                }
            }
        }

        impl From<$inner> for $py_name {
            fn from(value: $inner) -> Self {
                Self::from_rust(value)
            }
        }

        impl Default for $py_name {
            fn default() -> Self {
                Self { $($field: $default),+ }
            }
        }

        #[pyo3::pymethods]
        impl $py_name {
            #[new]
            #[pyo3(signature = ($($field = $default),+))]
            #[allow(clippy::too_many_arguments)]
            fn new($($field: $fty),+) -> Self {
                Self { $($field),+ }
            }

            /// Serialize to the JSON body sent to the service.
            fn to_json(&self) -> pyo3::PyResult<String> {
                serde_json::to_string(&self.to_rust())
                    .map_err(|err| pyo3::exceptions::PyValueError::new_err(err.to_string()))
            }

            /// Parse from a JSON body.
            #[staticmethod]
            fn from_json(json: &str) -> pyo3::PyResult<Self> {
                serde_json::from_str::<$inner>(json)
                    .map(Self::from_rust)
                    .map_err(|err| pyo3::exceptions::PyValueError::new_err(err.to_string()))
            }

            $($($extra)*)?

            fn __repr__(&self) -> String {
                format!("{:?}", self.to_rust())
            }
        }
    };
}

/// Bind a plain Rust data type read-only: a newtype class with one getter
/// per listed field (`<$ret>::from(field.clone())`) plus optional extra
/// methods.
macro_rules! py_readonly_class {
    (
        $py_name:ident, $public_name:literal, $inner:ty,
        { $($field:ident : $ret:ty),* $(,)? }
        $(, { $($extra:tt)* })?
    ) => {
        #[pyo3::pyclass(module = "booster_sdk_bindings", name = $public_name)]
        #[derive(Clone)]
        pub struct $py_name(pub(crate) $inner);

        #[pyo3::pymethods]
        #[allow(clippy::clone_on_copy, clippy::useless_conversion)]
        impl $py_name {
            $(
                #[getter]
                fn $field(&self) -> $ret {
                    <$ret>::from(self.0.$field.clone())
                }
            )*

            $($($extra)*)?

            fn __repr__(&self) -> String {
                format!("{:?}", self.0)
            }
        }

        impl From<$inner> for $py_name {
            fn from(value: $inner) -> Self {
                Self(value)
            }
        }
    };
}

pub(crate) use py_int_enum;
pub(crate) use py_param_class;
pub(crate) use py_readonly_class;
