use std::fmt::{Debug, Formatter, Result as FormatResult};
use std::marker::PhantomData;

use bevy_ecs::prelude::*;
use bevy_ecs::query::WorldQuery;
use bevy_ecs::system::EntityCommands;
use bevy_ecs::world::EntityRef;

pub use macros::EntityKind;

///
/// Some kind of an [`Entity`] with an expected set of components.
///
pub trait EntityKind: 'static + Send + Sync {
    ///
    /// A [`Bundle`] of components created and inserted by default into all entities with this [`EntityKind`].
    ///
    type DefaultBundle: Bundle + Default;

    ///
    /// A [`Bundle`] of components inserted into all entities with this [`EntityKind`].
    ///
    type Bundle: Bundle;

    ///
    /// Creates a new [`Entity`] with this [`EntityKind`].
    ///
    /// # Safety
    ///
    /// This function assumes `entity` has all the components associated with this [`EntityKind`].
    ///
    unsafe fn from_entity_unchecked(entity: Entity) -> Self;

    ///
    /// Returns this [`EntityKind`] as a generic [`Entity`].
    ///
    fn entity(&self) -> Entity;
}

///
/// A [`Bundle`] inserted into all entities of given [`EntityKind`].
///
#[derive(Bundle)]
pub struct KindBundle<T: EntityKind> {
    kind: Kind<T>,
    #[bundle]
    default_bundle: T::DefaultBundle,
    #[bundle]
    bundle: T::Bundle,
}

impl<T: EntityKind> KindBundle<T> {
    ///
    /// Creates a new [`KindBundle`] with given [`EntityKind::Bundle`].
    ///
    pub fn new(bundle: T::Bundle) -> Self {
        Self {
            kind: Kind::default(),
            default_bundle: T::DefaultBundle::default(),
            bundle,
        }
    }
}

impl<T: EntityKind> Clone for KindBundle<T>
where
    T::Bundle: Clone,
{
    fn clone(&self) -> Self {
        Self {
            kind: Kind::default(),
            default_bundle: T::DefaultBundle::default(),
            bundle: self.bundle.clone(),
        }
    }
}

impl<T: EntityKind> Default for KindBundle<T>
where
    T::Bundle: Default,
{
    fn default() -> Self {
        Self::new(T::Bundle::default())
    }
}

///
/// A [`WorldQuery`] filter for entities with some given [`EntityKind`].
///
#[derive(WorldQuery)]
pub struct WithKind<T: EntityKind> {
    with_kind: With<Kind<T>>,
}

///
/// A [`WorldQuery`] used to query entities with some given [`EntityKind`].
///
#[derive(WorldQuery)]
pub struct EntityWithKind<T: EntityKind> {
    entity: Entity,
    with_kind: WithKind<T>,
}

impl<T: EntityKind> EntityWithKindItem<'_, T> {
    ///
    /// Returns this [`EntityWithKindItem`] as a generic [`Entity`].
    ///
    pub fn entity(&self) -> Entity {
        self.entity
    }

    ///
    /// Returns the [`EntityKind`] from query.
    ///
    pub fn get(&self) -> T {
        // SAFE: `EntityWithKind` ensures entity has correct kind
        unsafe { T::from_entity_unchecked(self.entity) }
    }
}

impl<T: EntityKind> PartialEq<T> for EntityWithKindItem<'_, T> {
    fn eq(&self, other: &T) -> bool {
        self.entity() == other.entity()
    }
}

impl<T: EntityKind + Debug> Debug for EntityWithKindItem<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "{:?}", self.get())
    }
}

///
/// A wrapper for [`EntityCommands`] to execute commands on entities with a specific [`EntityKind`].
///
pub struct EntityKindCommands<'w, 's, 'a, T: EntityKind>(
    EntityCommands<'w, 's, 'a>,
    PhantomData<T>,
);

impl<'w, 's, 'a, T: EntityKind> EntityKindCommands<'w, 's, 'a, T> {
    ///
    /// Creates a new [`EntityKindCommands`] with some generic [`EntityCommands`].
    ///
    /// # Safety
    ///
    /// This function assumes `entity` is associated with the correct [`EntityKind`].
    ///
    pub unsafe fn from_entity_unchecked(entity: EntityCommands<'w, 's, 'a>) -> Self {
        Self(entity, PhantomData)
    }

    ///
    /// Returns the associated [`Entity`].
    ///
    pub fn entity(&self) -> Entity {
        self.0.id()
    }

    ///
    /// Returns the associated [`EntityKind`].
    ///
    pub fn get(&self) -> T {
        // SAFE: `EntityKindCommands<T>` is always associated with an entity of matching kind
        unsafe { T::from_entity_unchecked(self.entity()) }
    }

    ///
    /// Returns the underlying [`Commands`].
    ///
    pub fn commands(&mut self) -> &mut Commands<'w, 's> {
        self.0.commands()
    }

    ///
    /// Returns the underying [`EntityCommands`].
    ///
    pub fn as_entity(&mut self) -> &mut EntityCommands<'w, 's, 'a> {
        &mut self.0
    }

    pub fn insert(&mut self, component: impl Component) -> &mut Self {
        self.0.insert(component);
        self
    }

    pub fn remove<S: Component>(&mut self) -> &mut Self {
        self.0.remove::<S>();
        self
    }
}

///
/// Extension trait used to insert a new [`EntityKind`] into any [`Entity`] using some [`EntityCommands`].
///
pub trait InsertKind<'w, 's, 'a> {
    ///
    /// Inserts a new [`EntityKind`] into the associated [`Entity`] and returns an [`EntityKindCommands`] for it.
    ///
    fn insert_kind<T: EntityKind>(self, bundle: T::Bundle) -> EntityKindCommands<'w, 's, 'a, T>;
}

impl<'w, 's, 'a> InsertKind<'w, 's, 'a> for EntityCommands<'w, 's, 'a> {
    fn insert_kind<T: EntityKind>(
        mut self,
        bundle: T::Bundle,
    ) -> EntityKindCommands<'w, 's, 'a, T> {
        self.insert_bundle(KindBundle::<T>::new(bundle));
        // SAFE: `KindBundle` was just inserted
        unsafe { EntityKindCommands::from_entity_unchecked(self) }
    }
}

///
/// Extension trait which provides [`EntityKind`] support for [`Commands`].
///
pub trait KindCommands<'w, 's, 'a> {
    ///
    /// Spawns a new [`Entity`] with given [`EntityKind`] and returns an [`EntityKindCommands`] for it.
    ///
    fn spawn_with_kind<T: EntityKind>(self, bundle: T::Bundle)
        -> EntityKindCommands<'w, 's, 'a, T>;

    ///
    /// Returns a new [`EntityKindCommands`] for some [`EntityKind`].
    ///
    fn with_kind<T: EntityKind>(self, kind: &T) -> EntityKindCommands<'w, 's, 'a, T>;
}

impl<'w, 's, 'a> KindCommands<'w, 's, 'a> for &'a mut Commands<'w, 's> {
    fn spawn_with_kind<T: EntityKind>(
        self,
        bundle: T::Bundle,
    ) -> EntityKindCommands<'w, 's, 'a, T> {
        self.spawn().insert_kind(bundle)
    }

    fn with_kind<T: EntityKind>(self, kind: &T) -> EntityKindCommands<'w, 's, 'a, T> {
        // SAFE: `kind` may only reference an entity with correct kind
        unsafe { EntityKindCommands::from_entity_unchecked(self.entity(kind.entity())) }
    }
}

///
/// Extension trait used to safely cast an [`Entity`] into an [`EntityKind`].
///
pub trait TryWithKind {
    ///
    /// Checks if this [`Entity`] has the given [`EntityKind`] and returns it.
    ///
    fn try_with_kind<T: EntityKind>(self) -> Option<T>;
}

impl TryWithKind for &EntityRef<'_> {
    fn try_with_kind<T: EntityKind>(self) -> Option<T> {
        self.contains::<Kind<T>>()
            // SAFE: Entity kind was just checked
            .then(|| unsafe { T::from_entity_unchecked(self.id()) })
    }
}

///
/// A [`Component`] which marks an [`Entity`] as having a given [`EntityKind`].
///
#[derive(Component)]
struct Kind<T: EntityKind>(PhantomData<T>);

impl<T: EntityKind> Default for Kind<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

pub mod utils {
    use bevy_ecs::prelude::*;
    use bevy_ecs::system::CommandQueue;

    ///
    /// Extension trait to execute [`Commands`] on a [`World`].
    ///
    pub trait Execute {
        fn execute<F: FnOnce(&World, Commands) -> R, R>(self, f: F) -> R;
    }

    impl Execute for &mut World {
        fn execute<F: FnOnce(&World, Commands) -> R, R>(self, f: F) -> R {
            let mut queue = CommandQueue::default();
            let commands = Commands::new(&mut queue, self);
            let result = f(self, commands);
            queue.apply(self);
            result
        }
    }
}
