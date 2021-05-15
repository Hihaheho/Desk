use bevy::prelude::*;
use systems::card::create_card;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_card_system.system());
    }
}

fn create_card_system(mut commands: Commands) {
    commands
        .spawn()
        .insert(create_card(Vec2::new(200.0, 200.0)));

    commands
        .spawn()
        .insert(create_card(Vec2::new(400.0, 200.0)));
}
