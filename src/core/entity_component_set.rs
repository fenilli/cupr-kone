use std::{
    any::TypeId,
    fmt::{Debug, Display},
    u32,
};

use super::entity::{Entity, MAX_N_ENTITIES};

pub struct EntityComponentSet<T> {
    sparse_map: Vec<u32>,
    entities: Vec<u32>,
    components: Vec<T>,
}

impl<T: 'static> EntityComponentSet<T> {
    pub fn new() -> Self {
        Self {
            sparse_map: vec![u32::MAX; MAX_N_ENTITIES],
            entities: Vec::new(),
            components: Vec::new(),
        }
    }

    pub fn insert(&mut self, entity: Entity, component: T) {
        let index = entity.index() as usize;

        if index >= MAX_N_ENTITIES {
            panic!(
                "EntityComponentSet<{:?}>: exceeded MAX_N_ENTITIES of {}",
                TypeId::of::<T>(),
                MAX_N_ENTITIES
            );
        }

        // Overwrite component
        if let Some(&dense_index) = self.sparse_map.get(index)
            && dense_index != u32::MAX
        {
            self.components[dense_index as usize] = component;
            return;
        }

        let dense_index = self.entities.len() as u32;
        self.entities.push(index as u32);
        self.components.push(component);
        self.sparse_map[index] = dense_index;
    }

    pub fn remove(&mut self, entity: Entity) {
        let index = entity.index() as usize;

        if index >= self.sparse_map.len() {
            return;
        }

        let dense_index = self.sparse_map[index] as usize;

        if dense_index as u32 == u32::MAX {
            return;
        }

        let last_dense_index = self.entities.len() - 1;

        if dense_index != last_dense_index {
            self.components.swap_remove(dense_index);
            let moved_entity = self.entities.swap_remove(dense_index) as usize;

            self.sparse_map[moved_entity] = dense_index as u32;
        } else {
            self.components.pop();
            self.entities.pop();
        }

        self.sparse_map[index] = u32::MAX;
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        let dense_index = match self.contains(entity) {
            None => return None,
            Some(index) => index,
        };

        self.components.get(dense_index)
    }

    fn contains(&self, entity: Entity) -> Option<usize> {
        let index = entity.index() as usize;

        if index < self.sparse_map.len() {
            let dense_index = self.sparse_map[index];
            if dense_index != u32::MAX {
                return Some(dense_index as usize);
            }
        }

        None
    }
}

impl<T: 'static + Debug> Display for EntityComponentSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "EntityComponentSet<{}>:", std::any::type_name::<T>())?;
        writeln!(f, "  Dense entities: {:?}", self.entities)?;
        writeln!(f, "  Components:")?;
        for (i, &entity_index) in self.entities.iter().enumerate() {
            let component = &self.components[i];
            writeln!(f, "    Entity {} -> {:?}", entity_index, component)?;
        }
        Ok(())
    }
}
