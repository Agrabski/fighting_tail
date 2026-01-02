use bevy::{
    app::{Plugin, Update},
    ecs::{
        entity::Entity,
        event::EventReader,
        system::{Commands, Query, Res},
    },
    log::debug,
    time::{Fixed, Time},
};

use crate::{
    map::{HEX_RADIUS_IN_METERS, HexGrid},
    movement::{
        GamePosition, Kph, MoveUnitMessage, MovementConfig, MovingTowards, PROGRESS_COMPLETE,
        PROGRESS_ZERO, Path,
    },
};

pub struct PathFindingPlugin;

impl Plugin for PathFindingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (calculate_path, move_unit_along_path));
    }
}

fn calculate_path(
    mut events: EventReader<MoveUnitMessage>,
    query: Query<(&GamePosition, &MovementConfig)>,
    grid: Res<HexGrid>,
    mut commands: Commands,
) {
    for event in events.read() {
        if let Ok((current_pos, config)) = query.get(event.unit) {
            debug!(
                target: "pathfinding",
                "Calculating path for unit {:?} from {:?} to {:?}",
                event.unit,
                current_pos.hex, event.destination
            );
            let path = grid.find_path(current_pos.hex, event.destination, config);
            if let Some(first) = path.first().copied() {
                commands
                    .entity(event.unit)
                    .insert((Path { waypoints: path }, MovingTowards::new(first)));
            }
        }
    }
}

const TEMPORARY_MOVE_SPEED: Kph = 10.0;

fn move_unit_along_path(
    mut query: Query<(Entity, &mut MovingTowards, &mut Path, &mut GamePosition)>,
    time: Res<Time<Fixed>>,
    mut commands: Commands,
) {
    for (entity, mut moving, mut path, mut position) in query.iter_mut() {
        moving.progress +=
            (TEMPORARY_MOVE_SPEED / HEX_RADIUS_IN_METERS * 1000.0 * time.delta_secs() * 60.0
                / 3600.0)
                * PROGRESS_COMPLETE;
        debug!(target: "movement", "Entity {:?} progressed to {:?}%", entity, moving.progress / PROGRESS_COMPLETE * 100.0);
        if moving.progress >= PROGRESS_COMPLETE {
            debug!(target: "movement", "Entity {:?} reached hex {:?}", entity, moving.destination);
            let overflow: f32 = moving.progress - PROGRESS_COMPLETE;
            moving.progress = PROGRESS_ZERO;
            position.hex = moving.destination;
            if let Some(next) = path
                .waypoints
                .iter()
                .skip_while(|&&h| h != moving.destination)
                .nth(1)
                .copied()
            {
                moving.destination = next;
                moving.progress += overflow;
                debug!(target: "movement", "Entity {:?} moving towards hex {:?}", entity, moving.destination);
            } else {
                debug!(target: "movement", "Entity {:?} reached final destination hex {:?}", entity, moving.destination);
                commands.entity(entity).remove::<(MovingTowards, Path)>();
                path.waypoints.clear();
            }
        }
    }
}
