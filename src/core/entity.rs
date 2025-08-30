use std::fmt::Display;

pub const MAX_N_ENTITIES: usize = 1_000_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Entity(u64);

impl Entity {
    pub fn new(index: u32, generation: u32) -> Self {
        Self((generation as u64) << 32 | index as u64)
    }

    // Gets lower 32 bits
    pub fn index(self) -> u32 {
        self.0 as u32
    }

    // Gets lower 32 bits
    pub fn generation(self) -> u32 {
        (self.0 >> 32) as u32
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "E({}:{})", self.index(), self.generation())
    }
}
