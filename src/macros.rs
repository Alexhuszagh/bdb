/**
 *  Macros
 *  ------
 *
 *  Shared macros for the BDB library.
 *
 *  :copyright: (c) 2014-2018 The Rust Project Developers.
 *  :copyright: (c) 2018 Alex Huszagh.
 *  :license: MIT, see LICENSE.md for more details.
 */

// MACROS
// ------

/**
 *  \brief Macro to register an enumeration to and from a number with serde.
 *
 *  Causes derived traits like `Debug` and `std::fmt` to correctly use
 *  the enumeration name, however, allows serde to serialize the enum
 *  to and from the underlying enum.
 *
 *  ```text, no_run
 *  \example
 *      enum_number!(Enumeration {
 *          A = 1,
 *          B = 2,
 *          C = 3,
 *      });
 *
 *      assert_eq!(serde_json::to_string(&Enumeration::A).unwrap(), "1");
 *      assert_eq!(serde_json::to_string(&Enumeration::B).unwrap(), "2");
 *      assert_eq!(serde_json::from_str("1").unwrap(), Enumeration::A);
 */
#[macro_export]
macro_rules! enum_number {
    ($name:ident { $($variant:ident = $value:expr, )* }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
        pub enum $name {
            $($variant = $value,)*
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                // Serialize the enum as a u64.
                serializer.serialize_u64(*self as u64)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("positive integer")
                    }

                    fn visit_u64<E>(self, value: u64) -> Result<$name, E>
                    where
                        E: ::serde::de::Error,
                    {
                        // Rust does not come with a simple way of converting a
                        // number to an enum, so use a big `match`.
                        match value {
                            $( $value => Ok($name::$variant), )*
                            _ => Err(E::custom(
                                format!("unknown {} value: {}",
                                stringify!($name), value))),
                        }
                    }
                }

                // Deserialize the enum from a u64.
                deserializer.deserialize_u64(Visitor)
            }
        }
    }
}
