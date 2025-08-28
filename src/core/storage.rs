use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

use super::entity::Entity;

trait ComponentVec {
    fn remove(&mut self, entity: Entity);
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_any(&self) -> &dyn Any;
}

pub struct SparseSet<T> {
    entities: Vec<Option<usize>>,
    dense: Vec<Entity>,
    data: Vec<T>,
}

impl<T> SparseSet<T> {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            dense: Vec::new(),
            data: Vec::new(),
        }
    }

    fn index_of(&self, entity: Entity) -> Option<usize> {
        self.entities.get(entity.id() as usize).and_then(|&i| i)
    }

    pub fn insert(&mut self, entity: Entity, component: T) {
        let id = entity.id() as usize;
        if id >= self.entities.len() {
            self.entities.resize(id + 1, None);
        }

        self.entities[id] = Some(self.dense.len());
        self.dense.push(entity);
        self.data.push(component);
    }

    pub fn len(&self) -> usize {
        self.dense.len()
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.index_of(entity).map(|i| &self.data[i])
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.index_of(entity).map(|i| &mut self.data[i])
    }

    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.dense.iter().cloned().zip(self.data.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        self.dense.clone().into_iter().zip(self.data.iter_mut())
    }
}

impl<T: 'static> ComponentVec for SparseSet<T> {
    fn remove(&mut self, entity: Entity) {
        if let Some(Some(i)) = self.entities.get(entity.id() as usize) {
            let i = *i;
            let last = self.dense.pop().unwrap();
            let last_data = self.data.pop().unwrap();
            if i < self.dense.len() {
                self.dense[i] = last;
                self.data[i] = last_data;
                self.entities[last.id() as usize] = Some(i);
            }
            self.entities[entity.id() as usize] = None;
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ComponentStorage {
    storages: HashMap<TypeId, RefCell<Box<dyn ComponentVec>>>,
}

impl ComponentStorage {
    pub fn new() -> Self {
        Self {
            storages: HashMap::new(),
        }
    }

    pub fn insert<T: 'static>(&mut self, entity: Entity, component: T) {
        let entry = self
            .storages
            .entry(TypeId::of::<T>())
            .or_insert_with(|| RefCell::new(Box::new(SparseSet::<T>::new())));

        entry
            .borrow_mut()
            .as_any_mut()
            .downcast_mut::<SparseSet<T>>()
            .unwrap()
            .insert(entity, component);
    }

    pub fn remove(&mut self, entity: Entity) {
        for storage in self.storages.values() {
            storage.borrow_mut().remove(entity);
        }
    }

    pub fn get<T: 'static>(&'_ self, entity: Entity) -> Option<Ref<'_, T>> {
        let cell = self.storages.get(&TypeId::of::<T>())?;
        let borrow = cell.borrow();

        Some(Ref::map(borrow, |b| {
            b.as_any()
                .downcast_ref::<SparseSet<T>>()
                .unwrap()
                .get(entity)
                .unwrap()
        }))
    }

    pub fn get_mut<T: 'static>(&'_ self, entity: Entity) -> Option<RefMut<'_, T>> {
        let cell = self.storages.get(&TypeId::of::<T>())?;
        let borrow = cell.borrow_mut();

        Some(RefMut::map(borrow, |b| {
            b.as_any_mut()
                .downcast_mut::<SparseSet<T>>()
                .unwrap()
                .get_mut(entity)
                .unwrap()
        }))
    }

    pub fn iter<T: 'static>(&'_ self) -> Option<Ref<'_, SparseSet<T>>> {
        let cell = self.storages.get(&TypeId::of::<T>())?;
        let borrow = cell.borrow();
        Ref::filter_map(borrow, |b| b.as_any().downcast_ref::<SparseSet<T>>()).ok()
    }

    pub fn iter_mut<T: 'static>(&'_ self) -> Option<RefMut<'_, SparseSet<T>>> {
        let cell = self.storages.get(&TypeId::of::<T>())?;
        let borrow = cell.borrow_mut();
        RefMut::filter_map(borrow, |b| b.as_any_mut().downcast_mut::<SparseSet<T>>()).ok()
    }
}
