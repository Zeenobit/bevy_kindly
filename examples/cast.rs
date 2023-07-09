//! This example demonstrates how entities with multiple kinds can be converted into each other.
//!
//! When you have an entity with multiple kinds, each system may need to reference the entity as one
//! its "inner kinds". Therefore, it becomes useful to enable conversion between different entity kinds
//! where needed.
//!
//! NOTE: It is recommended to understand the `multiple.rs` example before going through this one.

use bevy::prelude::*;
use bevy_kindly::*;

/// Same as `Person` from `person.rs` example, except it uses a named `PersonBundle` to define
/// the components of a `Person`.
/// This is needed in order to assign multiple kinds to the person.
#[derive(EntityKind, Debug)]
#[default_components(Friends)]
#[bundle(PersonBundle)]
struct Person(Entity);

/// Enable conversion from `Person` to `Agent` (similar to an "upcast").
impl From<Person> for Agent {
    fn from(Person(entity): Person) -> Self {
        // SAFE: Every `Person` is also an `Agent`
        unsafe { Agent::from_entity_unchecked(entity) }
    }
}

#[derive(Bundle)]
struct PersonBundle {
    name: Name,
    age: Age,
    agent: KindBundle<Agent>,
}

#[derive(Component, Default)]
struct Friends(Vec<Person>);

#[derive(Component, Clone)]
struct Age(u32);

// `Agent`, same as `navigation.rs`
#[derive(Debug, EntityKind)]
#[default_components(Position)]
#[components(Speed, Clearance)]
struct Agent(Entity);

#[derive(Component)]
struct Speed(f64);

#[derive(Component)]
struct Clearance(usize);

#[derive(Component, Default, Debug)]
struct Position(Vec2);

#[derive(Component)]
struct Destination(Position);

trait NavigateTo {
    fn navigate_to(self, position: Position);
}

/// Only agents can have a navigation request.
impl NavigateTo for &mut EntityKindCommands<'_, '_, '_, Agent> {
    fn navigate_to(self, position: Position) {
        self.insert(Destination(position));
    }
}

fn spawn_person(mut commands: Commands) {
    // Spawn `Person`
    let person: Person = commands.spawn_with_kind::<Person>(PersonBundle {
        name: "Alice".into(),
        age: Age(25),
        agent: KindBundle::new((Speed(10.0), Clearance(2))),
    }).get();
    info!("{:?} spawned", person);

    // Cast the person into an `Agent`
    let agent = person.into();

    commands.with_kind(&agent).navigate_to(Position(Vec2::new(5.0, 10.0)));
}

// Navigation logic is the same as `navigation.rs`
// Notice how navigation doesn not need to filter for `Person`.
fn update_navigation(
    mut query: Query<(
        EntityWithKind<Agent>,
        &Speed,
        &Clearance,
        &Destination,
        &mut Position,
    )>,
) {
    for (agent, _speed, _clearance, destination, _position) in &mut query {
        info!("TODO: {:?} is navigating to {:?} ...", agent, destination.0);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_person)
        .add_systems(Update, update_navigation)
        .run();
}
