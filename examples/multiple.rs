use bevy::prelude::*;
use bevy_kindly::*;

/// NOTE: Refer to `person.rs` and `navigation.rs` before going through this example.

/// Same as `Person` from `person.rs` example, except it uses a named `PersonBundle` to define
/// the components of a `Person`.
/// This is needed in order to assign multiple kinds to the person.
#[derive(EntityKind, Clone, Copy, PartialEq, Eq)]
#[default_components(Friends)]
#[bundle(PersonBundle)]
struct Person(Entity);

#[derive(Bundle)]
struct PersonBundle {
    // `Name` and `Age`, same as `person.rs`
    name: Name,
    age: Age,
    // `Person` must also be of kind `Agent`
    #[bundle]
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

fn main() {
    let mut world = World::default();

    // Spawn a Person through direct kind bundle insertion
    let entity = world
        .spawn()
        .insert_bundle(KindBundle::<Person>::new(PersonBundle {
            name: "Alice".into(),
            age: Age(25),
            agent: KindBundle::<Agent>::new((Speed(10.0), Clearance(2))),
        })).id();

    // Ensure the entity is both a Person and an Agent
    assert!(world.entity(entity).try_with_kind::<Person>().is_some());
    assert!(world.entity(entity).try_with_kind::<Agent>().is_some());

    // Ensure all required components exist on the person
    assert!(world.entity(entity).contains::<Friends>());
    assert!(world.entity(entity).contains::<Name>());
    assert!(world.entity(entity).contains::<Age>());
    assert!(world.entity(entity).contains::<Position>());
    assert!(world.entity(entity).contains::<Speed>());
    assert!(world.entity(entity).contains::<Clearance>());
}
