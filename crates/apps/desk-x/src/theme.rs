use bevy::prelude::*;
use desk_theme::Theme;

pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Theme>()
            .add_startup_system(add_theme)
            .add_system(theme_changed);
    }
}

fn add_theme(mut commands: Commands) {
    commands.spawn(ron::from_str::<Theme>(include_str!("../../../../configs/theme.ron")).unwrap());
}

fn theme_changed(
    mut clear_coler: ResMut<ClearColor>,
    mut theme: Query<&mut Theme, Changed<Theme>>,
) {
    if let Ok(theme) = theme.get_single() {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let theme_ron = ron::ser::to_string_pretty(theme, Default::default()).unwrap();
            std::fs::write("configs/theme.ron", theme_ron).unwrap();
        }
    }
}
