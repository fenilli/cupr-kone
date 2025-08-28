#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Entity(u64);

impl Entity {
    pub fn new(id: u32, generation: u32) -> Self {
        Self((generation as u64) << 32 | id as u64)
    }

    // Gets lower 32 bits
    pub fn id(self) -> u32 {
        self.0 as u32
    }
}

pub struct EntityManager {
    free_list: Vec<u32>,
    generations: Vec<u32>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            free_list: Vec::new(),
            generations: Vec::new(),
        }
    }

    pub fn create(&mut self) -> Entity {
        if let Some(id) = self.free_list.pop() {
            let generation = self.generations[id as usize];
            Entity::new(id, generation)
        } else {
            let id = self.generations.len() as u32;
            self.generations.push(0);
            Entity::new(id, 0)
        }
    }

    pub fn destroy(&mut self, entity: Entity) {
        let id = entity.id();
        self.generations[id as usize] += 1;
        self.free_list.push(id);
    }
}
