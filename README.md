## Bevy 💖 Kindly

This crate is a minimalistic implementation of [Kinded Entities](https://github.com/bevyengine/bevy/issues/1634) for Bevy game engine.

In summary, it allows the user to define, construct, and query entities of different kinds. Each kind of entity is defined with an expected set of components, and a specialized command queue which may be extended with commands for specific kinds of entities.

This means that instead of writing this... 😵‍💫
```rust
#[derive(Component)]
struct Friends(Vec<Entity>);

#[derive(Component)]
struct Inventory {
  items: Vec<Entity>,
  buckets: HashMap<Entity, Vec<Entity>>,
}
```

You can write this... 😌
```rust
#[derive(Component)]
struct Friends(Vec<Person>);

#[derive(Component)]
struct Inventory {
  items: Vec<Item>,
  buckets: HashMap<Bucket, Vec<Item>>,
}
```

Where `Person`, `Item`, and `Bucket` can be defined as unique entity kinds.

The end result is increased readability because it's much easier to distinguish references to different kinds of entities. It also allows the user to make safer assumptions about the existence of some expected components on a specific kind of entity.

### Integration

Add to `Cargo.toml` (replace * with your desired version):
```
[dependencies]
bevy_kindly = "*"
```

### Usage

To define an entity kind, this boilerplate is currently needed:
```rust
struct Person(Entity);

impl EntityKind for Person {
    // Components inserted into every `Person` by default:
    type DefaultBundle = (Friends,);

    // Components that must be provided to spawn a `Person`:
    type Bundle = (Name, Age);

    // Boilerplate
    unsafe fn from_entity_unchecked(entity: Entity) -> Self {
        Self(entity)
    }

    // Boilerplate
    fn entity(&self) -> Entity {
        self.0
    }
}
```
Ideally, this should be wrapped into a macro of some kind. I'm still working on that.

Entities can be spawned with a kind in 3 separate ways, all of which are identical in underlying implementation.
They can either be spawned using `spawn_with_kind<T>`:
```rust
commands.spawn_with_kind::<Person>(("Alice".into(), Age(25)));
```
Or using `insert_kind<T>` if the entity is already spawned, or if the entity may have multiple kinds:
```rust
commands.entity(entity).insert_kind::<Person>(("Alice".into(), Age(25)));
```
Or by just inserting a `KindBundle<T>` directly:
```rust
commands.entity(entity).insert(KindBundle::<Owner>::new(("Alice".into(), Age(25))));
```
Notice how you must provide the required components in order to mark the entity as the given kind.

Any system can filter queries using `WithKind<T>` and `EntityWithKind<T>` world queries.
`EntityWithKind<T>` is designed to function like an `Entity`, but with a kind.
`WithKind<T>` can be used as a query filter when the actual entity is not needed.

For example:
```rust
fn do_something_with_people_and_their_friends(query: Query<(EntityWithKind<Person>, &Friends)>) {
  for (person, friends) in &query {
    let person: Person = person.get();
    ...
    let entity: Entity = person.entity();
    ...
  }
}
```

Additionally, any entity kind can have special commands that may only be invoked on entities of that kind.
This is done by extending `EntityKindCommands<T>`:

```rust
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
```

These commands can then be invoked on any entity with kind:
```rust
let alice = commands.spawn_with_kind::<Person>(("Alice".into(), Age(25))).get();
commands.spawn_with_kind::<Person>(("Bob".into(), Age(30))).add_friend(alice);
```
Or:
```rust
let alice = commands.spawn_with_kind::<Person>(("Alice".into(), Age(25))).get();
let bob = commands.spawn_with_kind::<Person>(("Bob".into(), Age(30))).get();
commands.with_kind(&alice).add_friend(bob);
```

Any `EntityRef` may also be "casted" safely into a kind using `try_with_kind`:
```rust
let person: Option<Person> = world.entity(entity).try_with_kind::<Person>();
```

### Cost

The main cost is implementing the `EntityKind` trait in your project as needed.

Beyond that, the only runtime cost is the addition of a private component with some `PhantomData<T>` that is added to mark the kind of an entity.
There is no need for any systems or type registration at runtime.

### Limitations

- There is no safety against direct removal of entity kind components.
- If an entity has multiple kinds, any intersection of the expected components can cause unwanted overrides.

### TODO

- [ ] Macro for entity kind boilerplate
- [ ] `Bundle` and `DefaultBundle` do not need to be defined if `#![feature(associated_type_defaults)]` is stabilized
- [ ] More documentation/examples/tests
