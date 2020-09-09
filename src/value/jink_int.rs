//! Represents an integer in Jinko. All integers are signed 64 bytes

pub struct JinkInt(i64);

impl From<i64> for JinkInt {
    fn from(i: i64) -> Self {
        JinkInt(i)
    }
}
