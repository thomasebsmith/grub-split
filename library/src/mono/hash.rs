pub trait Hash {
    #[must_use]
    fn hash(&self) -> u32;
}

impl Hash for str {
    fn hash(&self) -> u32 {
        let mut hash: u32 = 0;

        if self.is_empty() {
            return hash;
        }

        for byte in &self.as_bytes()[1..] {
            hash = hash
                .wrapping_shl(5)
                .wrapping_sub(hash.wrapping_add((*byte).into()));
        }

        hash = hash.wrapping_shl(5).wrapping_sub(hash);

        hash
    }
}

impl Hash for String {
    fn hash(&self) -> u32 {
        self.as_str().hash()
    }
}
