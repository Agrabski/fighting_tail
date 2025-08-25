// A complete, self-contained Bevy system for pseudo-turn-based unit movement on a grid.
// This example now organizes the logic into a dedicated plugin for better code structure.
// This code is compatible with Bevy 0.16.1.
mod camera;
mod map;
mod game_actions;
mod movement;
mod units;
use bevy::prelude::*;

use crate::map::{HexGridPlugin, HexPosition};



/// Component to mark a unit.
#[derive(Component)]
pub struct Unit;

/// Component to hold the unit's calculated path.
/// The path is a vector of positions representing the steps to take.
#[derive(Component, Debug)]
pub struct Path {
    pub positions: Vec<HexPosition>,
}

/// The unit's movement speed in hexes per minute.
#[derive(Component)]
pub struct MovementSpeed(f32);

/// Event to trigger a unit's movement to a new destination.
#[derive(Event)]
pub struct MoveUnitEvent {
    pub unit: Entity,
    pub destination: HexPosition,
}

/// The state of the game, managing player input and unit movement.
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum GameState {
    /// Player is issuing commands.
    #[default]
    Planning,
    /// Units are executing their orders.
    Execution,
}

/// A resource to track the total elapsed in-game time.
#[derive(Resource)]
pub struct InGameTime {
    pub time_elapsed: f32,
}

/// A resource to track the timer for each turn.
#[derive(Resource)]
pub struct TurnTimer {
    pub timer: Timer,
}


// ---
// Bevy Systems
// ---

/// System to handle `MoveUnitEvent` and calculate the path.
/// This system only runs during the planning phase.
fn handle_move_unit_event(
    mut events: EventReader<MoveUnitEvent>,
    mut commands: Commands,
    unit_query: Query<&Position, With<Unit>>,
) {
    for event in events.read() {
        if let Ok(start_pos) = unit_query.get(event.unit) {
            if let Some(path) = find_path(*start_pos, event.destination) {
                // If a path is found, add the `Path` component to the unit.
                commands.entity(event.unit).insert(Path { positions: path });
                println!(
                    "Path found for unit {:?}: {:?}",
                    event.unit, event.destination
                );
            } else {
                println!(
                    "No path found for unit {:?} to destination {:?}",
                    event.unit, event.destination
                );
            }
        }
    }
}

/// Core movement system. Moves units along their path based on their speed and the turn duration.
/// This system only runs during the execution phase.
fn move_units(
    mut commands: Commands,
    mut unit_query: Query<(Entity, &mut Position, &mut Path, &MovementSpeed)>,
) {
    // The amount of movement a unit can perform in a single 15-minute turn.
    const TURN_DURATION_MINUTES: f32 = 15.0;

    for (entity, mut position, mut path, speed) in unit_query.iter_mut() {
        // Calculate the maximum number of hexes the unit can travel this turn.
        let hexes_to_move = (speed.0 * TURN_DURATION_MINUTES) as usize;

        if path.positions.is_empty() {
            // Path is complete, remove the Path component.
            commands.entity(entity).remove::<Path>();
            println!("Unit {:?} reached its destination.", entity);
            continue;
        }

        // Move the unit a number of steps equal to its movement allowance for this turn.
        let steps_this_turn = std::cmp::min(hexes_to_move, path.positions.len());

        if steps_this_turn > 0 {
            // Update the unit's position to the last hex it can reach this turn.
            *position = path.positions[steps_this_turn - 1];
            // Remove the positions the unit has just traversed.
            path.positions.drain(0..steps_this_turn);

            println!("Unit {:?} moved to {:?}", entity, *position);
        } else {
            // Unit can't move this turn, maybe due to low speed.
            println!("Unit {:?} did not move this turn.", entity);
        }
    }
}

/// System to manage the turn timer and state transitions.
fn turn_timer_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut turn_timer: ResMut<TurnTimer>,
    mut in_game_time: ResMut<InGameTime>,
    time: Res<Time>,
) {
    turn_timer.timer.tick(time.delta());

    if turn_timer.timer.just_finished() {
        // Turn is over, switch to planning phase.
        next_state.set(GameState::Planning);
        // Add the turn duration to the total in-game time.
        in_game_time.time_elapsed += turn_timer.timer.duration().as_secs_f32();
        println!("Turn finished. Switching to Planning phase.");
    }
}

/// System to begin the execution phase when the user presses a key (e.g., 'Return').
fn begin_execution_phase(
    mut next_state: ResMut<NextState<GameState>>,
    mut turn_timer: ResMut<TurnTimer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::Execution);
        turn_timer.timer.reset(); // Reset the timer to begin the new turn
        println!("Turn started. Switching to Execution phase.");
    }
}

/// System to display the current game state and in-game time.
fn display_game_state(
    current_state: Res<State<GameState>>,
    in_game_time: Res<InGameTime>,
    turn_timer: Res<TurnTimer>,
) {
    println!("Current State: {:?}", current_state.get());
    println!(
        "In-Game Time: {:.2} minutes",
        in_game_time.time_elapsed / 60.0
    );
    println!(
        "Time remaining in turn: {:.2} seconds",
        turn_timer.timer.remaining_secs()
    );
}

/// A Bevy plugin that handles all movement-related logic.
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add resources
            .insert_resource(InGameTime { time_elapsed: 0.0 })
            // A turn is 15 minutes of in-game time
            .insert_resource(TurnTimer {
                timer: Timer::from_seconds(15.0 * 60.0, TimerMode::Once),
            })
            // Add events
            .add_event::<MoveUnitEvent>()
            // Add systems to manage the game state and turn
            .add_systems(
                Update,
                (
                    handle_move_unit_event.run_if(in_state(GameState::Planning)),
                    move_units.run_if(in_state(GameState::Execution)),
                    turn_timer_system.run_if(in_state(GameState::Execution)),
                    begin_execution_phase.run_if(in_state(GameState::Planning)),
                    display_game_state,
                ),
            );
    }
}

/// Setup system to spawn a unit and trigger its first movement.
fn setup(mut commands: Commands, mut move_event: EventWriter<MoveUnitEvent>) {
    // Spawn a unit at (0, 0)
    let unit = commands
        .spawn((
            Unit,
            Position { x: 0, y: 0 },
            // Speed in hexes per minute
            MovementSpeed(2.0),
             Sprite {
                    color: Color::Hwba(Hwba { hue: 1.0, whiteness: 1.0, blackness: 1.0, alpha: 100.0 }),
                    custom_size: Some(Vec2::splat(10.0)),
                    ..Default::default()
                }
        ))
        .id();

    // Send a movement event to the unit to move to (10, 5)
    move_event.write(MoveUnitEvent {
        unit,
        destination: Position { x: 10, y: 5 },
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the new MovementPlugin
        .add_plugins(MovementPlugin)
        .add_plugins(HexGridPlugin)
        .add_plugins(camera::CameraPlugin)
        // Add setup system
        .add_systems(Startup, setup)
        .insert_state(GameState::Planning)
        .run();
}
