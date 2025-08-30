use super::entity::{Entity, MAX_N_ENTITIES};

pub struct EntityManager {
    generations: Vec<u32>,
    free_vec: Vec<u32>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            generations: Vec::with_capacity(MAX_N_ENTITIES),
            free_vec: Vec::with_capacity(MAX_N_ENTITIES),
        }
    }

    pub fn allocate(&mut self) -> Entity {
        if self.generations.len() >= MAX_N_ENTITIES && self.free_vec.is_empty() {
            panic!(
                "EntityManager: exceeded MAX_N_ENTITIES of {}",
                MAX_N_ENTITIES
            );
        }

        let (index, generation) = if let Some(index) = self.free_vec.pop() {
            (index, self.generations[index as usize])
        } else {
            let index = self.generations.len();
            let generation: u32 = 0;
            self.generations.push(0);
            (index as u32, generation)
        };

        Entity::new(index, generation)
    }

    pub fn deallocate(&mut self, entity: Entity) -> bool {
        let index = entity.index() as usize;

        if index >= self.generations.len() {
            return false;
        };

        self.generations[index] = self.generations[index].wrapping_add(1);
        self.free_vec.push(index as u32);

        true
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        let index = entity.index() as usize;

        if index >= self.generations.len() {
            return false;
        };

        entity.generation() == self.generations[index]
    }
}
