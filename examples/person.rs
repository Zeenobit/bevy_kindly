use bevy::prelude::*;
use bevy_kindly::*;

// A `Person` is a kind of entity.
// Deriving `Clone`, `Copy`, `PartialEq`, and `Eq` is not required, but it's often convenient.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Person(Entity);

// Define the entity kind:
impl EntityKind for Person {
    // Components inserted into every `Person` by default:
    type DefaultBundle = (Friends,);

    // Components that must be provided to spawn a `Person`:
    type Bundle = (Name, Age);

    // Boilerplate :( (WIP)
    unsafe fn from_entity_unchecked(entity: Entity) -> Self {
        Self(entity)
    }

    // Boilerplate :( (WIP)
    fn entity(&self) -> Entity {
        self.0
    }
}

#[derive(Component, Default)]
struct Friends(Vec<Person>);

#[derive(Component, Clone)]
struct Age(u32);

// Some commands only `Person` entities can invoke:
trait PersonCommands {
    // Only people can be friends with each other
    fn add_friend(self, friend: Person);
}

impl PersonCommands for &mut EntityKindCommands<'_, '_, '_, Person> {
    fn add_friend(self, friend: Person) {
        let person = self.get();
        self.commands().add(move |world: &mut World| {
            // These unwraps are safe(er), because every `Person` entity has a `Friends` component
            world.get_mut::<Friends>(person.entity()).unwrap().0.push(friend);
            world.get_mut::<Friends>(friend.entity()).unwrap().0.push(person);
        });
    }
}

fn main() {
    use bevy_kindly::utils::Execute;

    let mut world = World::new();

    // Spawn Alice
    let alice: Person = world.execute(|_, mut commands| {
        // Name and Age must be provided. Friends is inserted automatically.
        commands.spawn_with_kind::<Person>(("Alice".into(), Age(25))).get()
    });

    // Spawn Bob
    let bob: Person = world.execute(|_, mut commands| {
        commands.spawn_with_kind::<Person>(("Bob".into(), Age(30))).get()
    });

    // Make Alice friends with Bob
    world.execute(|_, mut commands| {
        commands.with_kind(&alice).add_friend(bob);
    });

    // Ensure Alice is friends with Bob
    assert!(world.get::<Friends>(alice.entity()).unwrap().0.contains(&bob));
    // Ensure Bob is friends with Alice
    assert!(world.get::<Friends>(bob.entity()).unwrap().0.contains(&alice));
}
