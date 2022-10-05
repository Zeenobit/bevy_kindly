## Bevy Kindly 💖

This is a minimalistic implementation of entity kinds for Bevy game engine. In summary, it allows the user to define, construct, and query entities of different kinds. Each kind of entity comes with an expected set of components, and a specialized command queue which may be extended with kind-specific system commands.

This means that instead of writing this... 😵‍💫
```rust
#[derive(Component)]
struct Inventory {
  items: Vec<Entity>,
  owner: Entity,
  buckets: HashMap<Entity, Vec<Entity>>,
}
```

You can write this... 😌
```rust
#[derive(Component)]
struct Inventory {
  items: Vec<Item>,
  owner: Owner,
  buckets: HashMap<Bucket, Vec<Item>>,
}
```

Where `Item`, `Owner`, and `Bucket` can be defined as unique entity kinds.

### Usage

To define an entity kind, this boilerplate is currently needed:
```rust
struct Owner(Entity);

impl EntityKind for Owner {
  // Components inserted by default into every Owner:
  type DefaultBundle = (Inventory); // Inserted automatically
  type Bundle = (Name); // Must be provided as a parameter during insertion

  unsafe fn from_entity_unchecked(entity: Entity) -> Self {
    Self(entity)
  }

  fn entity(&self) -> Entity {
    self.0
  }
}
```
Ideally, this should be wrapped into a macro of some kind. I'm still working on that.

Entities can be spawned with a kind in 3 separate ways, all of which are identical in underlying implementation.
They can either be spawned using `spawn_with_kind<T>`:
```rust
commands.spawn_with_kind::<Owner>(...);
```
Or using `insert_kind<T>` if the entity is already spawned, or if the entity may have multiple kinds:
```rust
commands.entity(entity).insert_kind::<Owner>(...);
```
Or by just inserting a `KindBundle<T>` directly:
```rust
commands.entity(entity).insert(KindBundle::<Owner>::new(...));
```

Any system can filter queries using `WithKind<T>` and `EntityWithKind<T>` world queries.
`EntityWithKind<T>` is designed to function like an `Entity`, but with a kind.
`WithKind<T>` can be used as a query filter when the actual entity is not needed.

For example:
```rust
fn update_inventories(query: Query<(EntityWithKind<Owner>, &Inventory)>) {
  for (owner, inventory) in &query {
    let owner: Owner = owner.get();
    ...
    let entity: Entity = owner.entity();
    ...
  }
}
```

Additionally, any entity kind can have special commands that may only be invoked on entities of that kind.
This is done by extending `EntityKindCommands<T>`:

```rust
trait InsertInventoryItem {
  fn insert_inventory_item(self, item: &Item);
}

impl InsertInventoryItem for &mut EntityKindCommands<'_, '_, '_, Owner> {
  fn insert_inventory_item(self, item: &Item) {
    self.commands().add(move |world: &mut World| {
      ...
    });
  }
}
```

These commands can then be invoked on any entity with kind:
```rust
commands.spawn_with_kind::<Owner>(("Bob".into(),)).insert_inventory_item(...);
```
Or:
```rust
let owner: Owner = ...;
commands.with_kind(&owner).insert_inventory_item(...);
```

Any `EntityRef` may also be "casted" safely into a kind using `try_with_kind`:
```rust
let owner: Option<Owner> = world.entity(entity).try_with_kind::<Owner>();
```

This is the original issue which was the motivation behind this crate:
https://github.com/bevyengine/bevy/issues/1634

### Limitations

- There is no safety against direct removal of entity kind components.
- If an entity has multiple kinds, any intersection of the expected components can cause unwanted overrides.

### TODO

- [ ] Macro for entity kind boilerplate
- [ ] `Bundle` and `DefaultBundle` do not need to be defined if `#![feature(associated_type_defaults)]` is stabilized
