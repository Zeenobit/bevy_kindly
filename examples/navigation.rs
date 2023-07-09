//! This example demonstrates how systems can use [`EntityKind`] to make strong guarantees
//! about queries.
//! 
//! Each [`EntityKind`] can be queried by systems in order to filter entities at runtime.

use bevy::prelude::*;
use bevy_kindly::*;

/// A navigation `Agent` is a kind of entity.
/// Each navigation agent must have some `Speed`, and `Clearance`.
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

/// Extension trait used to set a navigation destination on an agent.
trait NavigateTo {
    fn navigate_to(self, position: Position);
}

/// Only agents can have a navigation request.
impl NavigateTo for &mut EntityKindCommands<'_, '_, '_, Agent> {
    fn navigate_to(self, position: Position) {
        self.insert(Destination(position));
    }
}

/// System which spawns an agent and adds a navigation request
fn spawn_agent(mut commands: Commands) {
    let mut agent = commands
        .spawn_with_kind::<Agent>((Speed(10.0), Clearance(2)));
    info!("{:?} spawned", agent.get());

    // Only agents have access to this command
    agent.navigate_to(Position(Vec2::new(5.0, 10.0)));
}

/// This system updates navigation for agents.
/// Because this system only operates on entities of kind `Agent`, there is a
/// strong guarantee[1] that all query entities contain `Speed`, `Clearance`, and `Position`.
/// 
/// It is not possible for an `Agent` entity to spawn without any of those components.
/// Therefore, there is no chance of this query "skipping" some entities because
/// they are missing one or more of the required components and/or bundles.
///
/// [1] This guarantee is broken if any of these components are removed manually after
/// the entity has spawned.
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
        .add_systems(Startup, spawn_agent)
        .add_systems(Update, update_navigation)
        .run();
}
