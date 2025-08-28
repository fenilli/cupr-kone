use std::cell::{Ref, RefMut};

use super::{
    entity::{Entity, EntityManager},
    storage::{ComponentStorage, SparseSet},
};

pub struct World {
    entity_manager: EntityManager,
    component_storage: ComponentStorage,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            component_storage: ComponentStorage::new(),
        }
    }

    pub fn spawn(&mut self) -> Entity {
        self.entity_manager.create()
    }

    pub fn despawn(&mut self, entity: Entity) {
        self.entity_manager.destroy(entity);
        self.component_storage.remove(entity);
    }

    pub fn insert<T: 'static>(&mut self, entity: Entity, component: T) {
        self.component_storage.insert(entity, component);
    }

    pub fn query<T: 'static>(&'_ self) -> Option<Ref<'_, SparseSet<T>>> {
        self.component_storage.iter::<T>()
    }

    pub fn query_mut<T: 'static>(&'_ self) -> Option<RefMut<'_, SparseSet<T>>> {
        self.component_storage.iter_mut::<T>()
    }
}
