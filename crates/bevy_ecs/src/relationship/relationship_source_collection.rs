use crate::entity::{hash_set::EntityHashSet, Entity};
use alloc::vec::Vec;
use smallvec::SmallVec;

/// The internal [`Entity`] collection used by a [`RelationshipTarget`](crate::relationship::RelationshipTarget) component.
/// This is not intended to be modified directly by users, as it could invalidate the correctness of relationships.
pub trait RelationshipSourceCollection {
    /// The type of iterator returned by the `iter` method.
    ///
    /// This is an associated type (rather than using a method that returns an opaque return-position impl trait)
    /// to ensure that all methods and traits (like [`DoubleEndedIterator`]) of the underlying collection's iterator
    /// are available to the user when implemented without unduly restricting the possible collections.
    ///
    /// The [`SourceIter`](super::SourceIter) type alias can be helpful to reduce confusion when working with this associated type.
    type SourceIter<'a>: Iterator<Item = Entity>
    where
        Self: 'a;

    /// Creates a new empty instance.
    fn new() -> Self;

    /// Returns an instance with the given pre-allocated entity `capacity`.
    ///
    /// Some collections will ignore the provided `capacity` and return a default instance.
    fn with_capacity(capacity: usize) -> Self;

    /// Reserves capacity for at least `additional` more entities to be inserted.
    ///
    /// Not all collections support this operation, in which case it is a no-op.
    fn reserve(&mut self, additional: usize);

    /// Adds the given `entity` to the collection.
    ///
    /// Returns whether the entity was added to the collection.
    /// Mainly useful when dealing with collections that don't allow
    /// multiple instances of the same entity ([`EntityHashSet`]).
    fn add(&mut self, entity: Entity) -> bool;

    /// Removes the given `entity` from the collection.
    ///
    /// Returns whether the collection actually contained
    /// the entity.
    fn remove(&mut self, entity: Entity) -> bool;

    /// Iterates all entities in the collection.
    fn iter(&self) -> Self::SourceIter<'_>;

    /// Returns the current length of the collection.
    fn len(&self) -> usize;

    /// Clears the collection.
    fn clear(&mut self);

    /// Attempts to save memory by shrinking the capacity to fit the current length.
    ///
    /// This operation is a no-op for collections that do not support it.
    fn shrink_to_fit(&mut self);

    /// Returns true if the collection contains no entities.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Add multiple entities to collection at once.
    ///
    /// May be faster than repeatedly calling [`Self::add`].
    fn extend_from_iter(&mut self, entities: impl IntoIterator<Item = Entity>) {
        // The method name shouldn't conflict with `Extend::extend` as it's in the rust prelude and
        // would always conflict with it.
        for entity in entities {
            self.add(entity);
        }
    }
}

impl RelationshipSourceCollection for Vec<Entity> {
    type SourceIter<'a> = core::iter::Copied<core::slice::Iter<'a, Entity>>;

    fn new() -> Self {
        Vec::new()
    }

    fn reserve(&mut self, additional: usize) {
        Vec::reserve(self, additional);
    }

    fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity)
    }

    fn add(&mut self, entity: Entity) -> bool {
        Vec::push(self, entity);

        true
    }

    fn remove(&mut self, entity: Entity) -> bool {
        if let Some(index) = <[Entity]>::iter(self).position(|e| *e == entity) {
            Vec::remove(self, index);

            return true;
        }

        false
    }

    fn iter(&self) -> Self::SourceIter<'_> {
        <[Entity]>::iter(self).copied()
    }

    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn shrink_to_fit(&mut self) {
        Vec::shrink_to_fit(self);
    }

    fn extend_from_iter(&mut self, entities: impl IntoIterator<Item = Entity>) {
        self.extend(entities);
    }
}

impl RelationshipSourceCollection for EntityHashSet {
    type SourceIter<'a> = core::iter::Copied<crate::entity::hash_set::Iter<'a>>;

    fn new() -> Self {
        EntityHashSet::new()
    }

    fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }

    fn with_capacity(capacity: usize) -> Self {
        EntityHashSet::with_capacity(capacity)
    }

    fn add(&mut self, entity: Entity) -> bool {
        self.insert(entity)
    }

    fn remove(&mut self, entity: Entity) -> bool {
        // We need to call the remove method on the underlying hash set,
        // which takes its argument by reference
        self.0.remove(&entity)
    }

    fn iter(&self) -> Self::SourceIter<'_> {
        self.iter().copied()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn clear(&mut self) {
        self.0.clear();
    }

    fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit();
    }

    fn extend_from_iter(&mut self, entities: impl IntoIterator<Item = Entity>) {
        self.extend(entities);
    }
}

impl<const N: usize> RelationshipSourceCollection for SmallVec<[Entity; N]> {
    type SourceIter<'a> = core::iter::Copied<core::slice::Iter<'a, Entity>>;

    fn new() -> Self {
        SmallVec::new()
    }

    fn reserve(&mut self, additional: usize) {
        SmallVec::reserve(self, additional);
    }

    fn with_capacity(capacity: usize) -> Self {
        SmallVec::with_capacity(capacity)
    }

    fn add(&mut self, entity: Entity) -> bool {
        SmallVec::push(self, entity);

        true
    }

    fn remove(&mut self, entity: Entity) -> bool {
        if let Some(index) = <[Entity]>::iter(self).position(|e| *e == entity) {
            SmallVec::remove(self, index);

            return true;
        }

        false
    }

    fn iter(&self) -> Self::SourceIter<'_> {
        <[Entity]>::iter(self).copied()
    }

    fn len(&self) -> usize {
        SmallVec::len(self)
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn shrink_to_fit(&mut self) {
        SmallVec::shrink_to_fit(self);
    }

    fn extend_from_iter(&mut self, entities: impl IntoIterator<Item = Entity>) {
        self.extend(entities);
    }
}

impl RelationshipSourceCollection for Entity {
    type SourceIter<'a> = core::iter::Once<Entity>;

    fn new() -> Self {
        Entity::PLACEHOLDER
    }

    fn reserve(&mut self, _: usize) {}

    fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }

    fn add(&mut self, entity: Entity) -> bool {
        *self = entity;

        true
    }

    fn remove(&mut self, entity: Entity) -> bool {
        if *self == entity {
            *self = Entity::PLACEHOLDER;

            return true;
        }

        false
    }

    fn iter(&self) -> Self::SourceIter<'_> {
        core::iter::once(*self)
    }

    fn len(&self) -> usize {
        if *self == Entity::PLACEHOLDER {
            return 0;
        }
        1
    }

    fn clear(&mut self) {
        *self = Entity::PLACEHOLDER;
    }

    fn shrink_to_fit(&mut self) {}

    fn extend_from_iter(&mut self, entities: impl IntoIterator<Item = Entity>) {
        if let Some(entity) = entities.into_iter().last() {
            *self = entity;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{Component, World};
    use crate::relationship::RelationshipTarget;

    #[test]
    fn vec_relationship_source_collection() {
        #[derive(Component)]
        #[relationship(relationship_target = RelTarget)]
        struct Rel(Entity);

        #[derive(Component)]
        #[relationship_target(relationship = Rel, linked_spawn)]
        struct RelTarget(Vec<Entity>);

        let mut world = World::new();
        let a = world.spawn_empty().id();
        let b = world.spawn_empty().id();

        world.entity_mut(a).insert(Rel(b));

        let rel_target = world.get::<RelTarget>(b).unwrap();
        let collection = rel_target.collection();
        assert_eq!(collection, &alloc::vec!(a));
    }

    #[test]
    fn smallvec_relationship_source_collection() {
        #[derive(Component)]
        #[relationship(relationship_target = RelTarget)]
        struct Rel(Entity);

        #[derive(Component)]
        #[relationship_target(relationship = Rel, linked_spawn)]
        struct RelTarget(SmallVec<[Entity; 4]>);

        let mut world = World::new();
        let a = world.spawn_empty().id();
        let b = world.spawn_empty().id();

        world.entity_mut(a).insert(Rel(b));

        let rel_target = world.get::<RelTarget>(b).unwrap();
        let collection = rel_target.collection();
        assert_eq!(collection, &SmallVec::from_buf([a]));
    }

    #[test]
    fn entity_relationship_source_collection() {
        #[derive(Component)]
        #[relationship(relationship_target = RelTarget)]
        struct Rel(Entity);

        #[derive(Component)]
        #[relationship_target(relationship = Rel)]
        struct RelTarget(Entity);

        let mut world = World::new();
        let a = world.spawn_empty().id();
        let b = world.spawn_empty().id();

        world.entity_mut(a).insert(Rel(b));

        let rel_target = world.get::<RelTarget>(b).unwrap();
        let collection = rel_target.collection();
        assert_eq!(collection, &a);
    }

    #[test]
    fn one_to_one_relationships() {
        #[derive(Component)]
        #[relationship(relationship_target = Below)]
        struct Above(Entity);

        #[derive(Component)]
        #[relationship_target(relationship = Above)]
        struct Below(Entity);

        let mut world = World::new();
        let a = world.spawn_empty().id();
        let b = world.spawn_empty().id();

        world.entity_mut(a).insert(Above(b));
        assert_eq!(a, world.get::<Below>(b).unwrap().0);

        // Verify removing target removes relationship
        world.entity_mut(b).remove::<Below>();
        assert!(world.get::<Above>(a).is_none());

        // Verify removing relationship removes target
        world.entity_mut(a).insert(Above(b));
        world.entity_mut(a).remove::<Above>();
        assert!(world.get::<Below>(b).is_none());

        // Actually - a is above c now! Verify relationship was updated correctly
        let c = world.spawn_empty().id();
        world.entity_mut(a).insert(Above(c));
        assert!(world.get::<Below>(b).is_none());
        assert_eq!(a, world.get::<Below>(c).unwrap().0);
    }
}
