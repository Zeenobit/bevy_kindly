use bevy_kindly::*;

use bevy_ecs::prelude::*;

///
/// An [`EntityKind`] which can be inserted into a [`Container`].
///
#[derive(EntityKind)]
struct Containable(Entity);

///
/// Collection of [`Containable`] entities currently stored inside a [`Container`].
///
#[derive(Component, Default)]
struct Items(Vec<Containable>);

///
/// Maximum number of items [`Container`] that may be stored in a [`Container`].
///
#[derive(Component)]
struct Capacity(usize);

///
/// An [`EntityKind`] which can store [`Containable`] entities.
///
#[derive(EntityKind)]
#[defaults(Items)]
#[components(Capacity)]
struct Container(Entity);

///
/// Extension trait to insert a [`Containable`] entity into a [`Container`].
///
trait InsertIntoContainer {
    fn insert_into_container(self, container: &Container) -> Self;
}

impl InsertIntoContainer for &mut EntityKindCommands<'_, '_, '_, Containable> {
    fn insert_into_container(self, &Container(entity): &Container) -> Self {
        let item = self.get();
        self.commands().add(move |world: &mut World| {
            let &Capacity(capacity) = world
                .get::<Capacity>(entity)
                .expect("container must have capacity");
            let Items(items) = world
                .get_mut::<Items>(entity)
                .expect("container must have items")
                .into_inner();
            if items.len() < capacity {
                items.push(item);
            }
        });
        self
    }
}

#[test]
fn it_works() {
    use bevy_kindly::utils::Execute;

    // Create a new world
    let mut world = World::new();

    // Spawn a container
    let container: Container = world
        .execute(|_, mut commands| commands.spawn_with_kind::<Container>((Capacity(5),)).get());

    // Ensure all containers have capacity and items
    assert!(world.entity(container.entity()).contains::<Capacity>());
    assert!(world.entity(container.entity()).contains::<Items>());

    world.execute(|_, mut commands| {
        // Spawn an item, insert it into the container
        commands
            .spawn_with_kind::<Containable>(())
            .insert_into_container(&container);
    });

    // Ensure item was inserted
    assert_eq!(world.get::<Items>(container.entity()).unwrap().0.len(), 1);
}
