use bevy::{app::{App, FixedUpdate, Plugin}, ecs::resource::Resource, time::{Fixed, Time}};

pub struct GameTimePlugin;

impl Plugin for GameTimePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(CurrentTimePoint(TimePoint { hours: 8, minutes: 0, seconds: 0, day: 1 }))
        .add_systems(FixedUpdate, advance_game_time);
    }
}

#[derive(Debug, Clone, Copy, PartialEq )]
pub struct TimePoint {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub day: u32
}

#[derive(Debug, Clone, Copy, PartialEq,Resource )]
pub struct CurrentTimePoint(pub TimePoint);


fn advance_game_time(
    time: bevy::prelude::Res<Time<Fixed>>,
    mut current_time: bevy::prelude::ResMut<CurrentTimePoint>,
) {
    let delta_seconds = time.delta_secs();
    let mut total_ingame_seconds = (delta_seconds * 60.0) as u32;

    let mut tp = current_time.0;

    tp.seconds += total_ingame_seconds % 60;
    if tp.seconds >= 60 {
        tp.seconds -= 60;
        tp.minutes += 1;
    }

    total_ingame_seconds /= 60;
    tp.minutes += total_ingame_seconds % 60;
    if tp.minutes >= 60 {
        tp.minutes -= 60;
        tp.hours += 1;
    }

    total_ingame_seconds /= 60;
    tp.hours += total_ingame_seconds % 24;
    if tp.hours >= 24 {
        tp.hours -= 24;
        tp.day += 1;
    }

    current_time.0 = tp;
}
