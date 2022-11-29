use std::ops::BitAnd;

#[must_use]
pub fn has_flag<T: BitAnd + Copy>(bitfield: T, flag: T) -> bool
where
    <T as BitAnd>::Output: PartialEq<T>,
{
    bitfield & flag == flag
}
