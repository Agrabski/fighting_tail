use bevy::{color::Color, ecs::resource::Resource, reflect::Reflect};

pub struct ThemePlugin;
impl bevy::app::Plugin for ThemePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Theme {
            primary: Color::srgba(1., 98. / 255., 81. / 255., 1.0),
            secondary: Color::srgba(172. / 255., 64. / 255., 63. / 255., 1.0),
            accent: Color::linear_rgba(252. / 255., 226. / 255., 8. / 255., 1.0),
            background: Color::srgba(8. / 255., 226. / 255., 252. / 255., 1.0),
            text: Color::WHITE,
        });
    }
}

#[derive(Clone, Debug, Reflect, Resource)]
pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub background: Color,
    pub text: Color,
}
