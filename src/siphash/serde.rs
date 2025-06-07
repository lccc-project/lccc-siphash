use serde::Deserialize;

use crate::{RawSipHasher, SipHashState};

impl serde::Serialize for SipHashState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_newtype_struct("SipHashState", &self.inspect_state())
    }
}

impl<const C: usize, const D: usize> serde::Serialize for RawSipHasher<C, D> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_newtype_struct("RawSipHasher", self.state())
    }
}

#[cfg(feature = "rng")]
impl<const C: usize, const D: usize> serde::Serialize for crate::rng::SiphashRng<C, D> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_newtype_struct("SiphashRng", self.raw())
    }
}

macro_rules! impl_newtype_deser {
    ($(::$path:ident)* $base:ident <$($gen:ident),*> as $ty:ty => $method:ident) => {
        impl <'de, $(const $gen: usize),*> serde::Deserialize<'de> for $($path ::)* $base <$($gen),*> {
            fn deserialize<__D>(deserializer: __D) -> Result<Self, __D::Error> where __D: serde::Deserializer<'de> {
                struct __Visitor<$(const $gen: usize),*>;

                impl<'de, $(const $gen: usize),*> serde::de::Visitor<'de> for __Visitor<$($gen),*> {
                    type Value = $($path ::)* $base <$($gen),*>;
                    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                        formatter.write_str(core::concat!("a ", core::stringify!($base)))
                    }

                    fn visit_newtype_struct<__D>(self, deserializer: __D) -> Result<Self::Value, __D::Error> where __D: serde::Deserializer<'de> {
                        let val = <$ty>::deserialize(deserializer)?;

                        Ok(<$($path ::)* $base::<$($gen),*>>::$method(val))
                    }
                }

                deserializer.deserialize_newtype_struct(core::stringify!($base), __Visitor::<$($gen),*>)
            }
        }
    };
}

impl_newtype_deser!(SipHashState <> as [u64; 4] => from_state);
impl_newtype_deser!(RawSipHasher <C,D> as SipHashState => from_state);
#[cfg(feature = "rng")]
impl_newtype_deser!(::crate::rng SiphashRng <C,D> as RawSipHasher<C,D> => from_raw);
