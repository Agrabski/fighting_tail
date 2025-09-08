use bevy::{
    app::{App, AppExit, Plugin, Startup},
    color::Color,
    ecs::{
        event::EventWriter,
        name::Name,
        observer::Trigger,
        system::{Commands, Res},
    },
    picking::events::{Click, Pointer},
    sprite::{Anchor, Sprite},
    window::SystemCursorIcon,
};
use bevy_lunex::{OnHoverSetCursor, Rl, UiColor, UiFetchFromCamera, UiLayout, UiLayoutRoot};

use crate::user_interface::theme::Theme;

pub struct UnitListPlugin;

impl Plugin for UnitListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_unit_list);
    }
}

fn setup_unit_list(
    mut commands: Commands,
    asset_server: Res<bevy::asset::AssetServer>,
    theme: Res<Theme>,
) {
    commands
        .spawn((
            Name::new("Unit List UI"),
            UiLayoutRoot::new_2d(),
            // Make the UI synchronized with camera viewport size
            UiFetchFromCamera::<0>,
        ))
        .with_children(|ui| {
            ui.spawn((
                // You can name the entity
                Name::new("My Rectangle"),
                // Specify the position and size of the button
                UiLayout::window()
                    .anchor(Anchor::Center) // Put the origin at the center
                    .pos(Rl((50.0, 50.0))) // Set the position to 50%
                    .size((200.0, 50.0)) // Set the size to [200.0, 50.0]
                    .pack(),
                // Color the sprite with red color
                UiColor::from(Color::srgb(1.0, 0.0, 0.0)),
                // Attach sprite to the node
                Sprite::from_image(asset_server.load("hex.png")),
                // When hovered, it will request the cursor icon to be changed
                OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                // Interactivity is done through observers, you can query anything here
            ))
            .observe(
                |_: Trigger<Pointer<Click>>, mut exit: EventWriter<AppExit>| {
                    // Close the app on click
                    exit.write(AppExit::Success);
                },
            );
        });
}
