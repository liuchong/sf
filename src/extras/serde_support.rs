use crate::id::Id;
use core::fmt;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
impl Serialize for Id {
    fn serialize<S: Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_string())
        } else {
            serializer.serialize_bytes(&self.as_bytes())
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Id {
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            struct IdStringVisitor;

            impl<'vi> de::Visitor<'vi> for IdStringVisitor {
                type Value = Id;

                fn expecting(
                    &self,
                    formatter: &mut fmt::Formatter,
                ) -> fmt::Result {
                    write!(formatter, "a sf id string")
                }

                fn visit_str<E: de::Error>(self, value: &str) -> Result<Id, E> {
                    value.parse::<Id>().map_err(E::custom)
                }

                fn visit_bytes<E: de::Error>(
                    self,
                    value: &[u8],
                ) -> Result<Id, E> {
                    Id::from_slice(value).map_err(E::custom)
                }
            }

            deserializer.deserialize_str(IdStringVisitor)
        } else {
            struct IdBytesVisitor;

            impl<'vi> de::Visitor<'vi> for IdBytesVisitor {
                type Value = Id;

                fn expecting(
                    &self,
                    formatter: &mut fmt::Formatter,
                ) -> fmt::Result {
                    write!(formatter, "bytes")
                }

                fn visit_bytes<E: de::Error>(
                    self,
                    value: &[u8],
                ) -> Result<Id, E> {
                    Id::from_slice(value).map_err(E::custom)
                }
            }

            deserializer.deserialize_bytes(IdBytesVisitor)
        }
    }
}
