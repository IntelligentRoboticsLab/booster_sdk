//! High-level client APIs for the Booster Robotics SDK.

pub mod ai_client;
pub mod commands;
pub mod light_control_client;
pub mod loco_client;
pub mod vision_client;
pub mod x5_camera_client;

pub use ai_client::*;
pub use commands::*;
pub use light_control_client::*;
pub use loco_client::*;
pub use vision_client::*;
pub use x5_camera_client::*;

/// Declare an i32-backed enum with serde, `From<i32>`, and `TryFrom<i32>`.
///
/// Default form makes the enum `pub`:
/// `api_id_enum! { Name { A = 1, B = 2 } }`
///
/// You can also pass attributes and visibility:
/// `api_id_enum! { #[non_exhaustive] pub(crate) Name { A = 1 } }`
#[macro_export]
macro_rules! api_id_enum {
    (
        $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident = $value:literal
            ),+ $(,)?
        }
    ) => {
        $crate::api_id_enum! {
            @impl
            pub $name {
                $(
                    $(#[$variant_meta])*
                    $variant = $value
                ),+
            }
        }
    };
    (
        $(#[$meta:meta])*
        $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident = $value:literal
            ),+ $(,)?
        }
    ) => {
        $crate::api_id_enum! {
            @impl
            $(#[$meta])*
            pub $name {
                $(
                    $(#[$variant_meta])*
                    $variant = $value
                ),+
            }
        }
    };
    (
        $(#[$meta:meta])*
        $vis:vis $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident = $value:literal
            ),+ $(,)?
        }
    ) => {
        $crate::api_id_enum! {
            @impl
            $(#[$meta])*
            $vis $name {
                $(
                    $(#[$variant_meta])*
                    $variant = $value
                ),+
            }
        }
    };
    (
        @impl
        $(#[$meta:meta])*
        $vis:vis $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident = $value:literal
            ),+ $(,)?
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
        #[serde(into = "i32", try_from = "i32")]
        #[repr(i32)]
        $(#[$meta])*
        $vis enum $name {
            $(
                $(#[$variant_meta])*
                $variant = $value,
            )+
        }

        impl From<$name> for i32 {
            fn from(value: $name) -> Self {
                value as i32
            }
        }

        impl TryFrom<i32> for $name {
            type Error = &'static str;

            fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
                match value {
                    $(
                        $value => Ok(Self::$variant),
                    )+
                    _ => Err("invalid value"),
                }
            }
        }
    };
}
