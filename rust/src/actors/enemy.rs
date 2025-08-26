use bevy::platform::collections::HashSet;
use godot::classes::{AnimatedSprite2D, RayCast2D};

use crate::prelude::*;

use super::*;

#[derive(GodotClass, BevyBundle)]
#[class(init, base = Node2D)]
#[bevy_bundle((Enemy),(Direction), (Speed: speed))]
pub struct EnemyNode {
    #[export]
    speed: f32,
    base: Base<Node2D>,
}

#[derive(Component, Default)]
struct Enemy;

#[derive(Resource, Default)]
struct REnemyIndex {
    to_change_dir: HashSet<Entity>,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<REnemyIndex>();
        app.add_systems(
            PhysicsUpdate,
            (detect_walls, change_dir, apply_direction)
                .chain()
                .run_if(in_state(GameState::InGame)),
        );
    }
}

#[main_thread_system]
fn detect_walls(
    mut enemies: Query<
        (&mut GodotNodeHandle, Entity),
        (With<Node2DMarker>, With<Enemy>),
    >,
    mut enemy_index: ResMut<REnemyIndex>,
) {
    let root_name = "Enemy";
    let _ = enemies
        .iter_mut()
        .try_for_each(|(mut handle, e)| {
            let root = handle.get::<Node2D>();
            let mut path = "WallCasts";
            let wallcasts = root
                .try_get_node_as::<Node2D>(path)
                .ok_or(NodeError::not_found(root_name, path))?;

            path = "GroundCasts";
            let groundcasts = root
                .try_get_node_as::<Node2D>(path)
                .ok_or(NodeError::not_found(root_name, path))?;

            path = ".";
            let wall_collision = wallcasts
                .get_children()
                .iter_shared()
                .map(|f| {
                    let raycast = f
                        .try_get_node_as::<RayCast2D>(path)
                        .ok_or(NodeError::not_found(root_name, path))?;
                    Aok(raycast)
                })
                .collect::<anyhow::Result<Vec<Gd<RayCast2D>>>>()?
                .iter()
                .any(|f| f.is_colliding());

            let no_ground = groundcasts
                .get_children()
                .iter_shared()
                .map(|f| {
                    let raycast = f
                        .try_get_node_as::<RayCast2D>(path)
                        .ok_or(NodeError::not_found(root_name, path))?;
                    Aok(raycast)
                })
                .collect::<anyhow::Result<Vec<Gd<RayCast2D>>>>()?
                .iter()
                .any(|f| !f.is_colliding());

            if wall_collision || no_ground {
                enemy_index.to_change_dir.insert(e);
            }
            Aok(())
        })
        .map_err(|e| warn!("{e}"));
}

fn change_dir(
    mut enemies: Query<&mut Direction, With<Enemy>>,
    mut enemy_index: ResMut<REnemyIndex>,
) {
    enemy_index.to_change_dir.iter().for_each(|entity| {
        let Ok(mut dir) = enemies.get_mut(*entity) else {
            return;
        };
        dir.change();
    });
    // reset index after processing all entities
    enemy_index.to_change_dir = HashSet::default();
}

#[main_thread_system]
fn apply_direction(
    mut enemies: Query<
        (&mut GodotNodeHandle, &Direction, &Speed),
        (With<Enemy>, With<Node2DMarker>),
    >,
    delta: Res<PhysicsDelta>,
) {
    let root_name = "Enemy";
    let _ = enemies
        .iter_mut()
        .try_for_each(|(mut handle, dir, speed)| {
            let mut node = handle.get::<Node2D>();
            let path = "EnemySprite";
            let mut sprite = node
                .try_get_node_as::<AnimatedSprite2D>(path)
                .ok_or(NodeError::not_found(root_name, path))?;

            let mut position = node.get_position();
            position.x += dir.0 * speed.0 * delta.delta_seconds;
            sprite.set_flip_h(dir.0 == -1.);
            node.set_position(position);

            Aok(())
        })
        .map_err(|e| warn!("{e}"));
}
