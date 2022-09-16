pub trait Hash {
    #[must_use]
    fn hash(&self) -> u32;
}
