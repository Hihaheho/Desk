use bevy::{prelude::*, render::camera::Camera};
use terminal::Cursor;

#[derive(Bundle)]
struct CursorBundle {
    cursor: Cursor,
}

pub(crate) fn add_cursor(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle((Cursor, Transform::default()));
}
pub(crate) fn move_cursor(
    windows: Res<Windows>,
    mut query_set: QuerySet<(
        Query<&Transform, With<Camera>>,
        Query<&mut Transform, With<Cursor>>,
    )>,
) {
    if let Some((window, position)) = windows
        .get_primary()
        .and_then(|window| window.cursor_position().map(|pos| (window, pos)))
    {
        let camera = {
            if let Ok(camera) = query_set.q0().single() {
                *camera
            } else {
                return;
            }
        };
        if let Ok(mut cursor) = query_set.q1_mut().single_mut() {
            let position = translate_position(position, window, &camera);
            cursor.translation.x = position.x;
            cursor.translation.y = position.y;
        }
    }
}

fn translate_position(pos: Vec2, wnd: &Window, camera: &Transform) -> Vec2 {
    // get the size of the window
    let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

    // the default orthographic projection is in pixels from the center;
    // just undo the translation
    let p = pos - size / 2.0;

    // apply the camera transform
    (camera.compute_matrix() * p.extend(0.0).extend(1.0))
        .truncate()
        .truncate()
}
