use serde::Serialize;
use std::ops::Deref;

#[derive(Serialize, Debug, Clone, Copy)]
pub struct NullableU32(Option<u32>);

impl TryFrom<Option<i64>> for NullableU32 {
    type Error = std::num::TryFromIntError;

    fn try_from(value: Option<i64>) -> Result<Self, Self::Error> {
        Ok(match value {
            Some(v) => NullableU32(Some(v.try_into()?)),
            None => NullableU32(None),
        })
    }
}

impl From<Option<u32>> for NullableU32 {
    fn from(value: Option<u32>) -> Self {
        Self(value)
    }
}

impl Deref for NullableU32 {
    type Target = Option<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl NullableU32 {
    pub fn as_option(&self) -> Option<u32> {
        self.0
    }
}
