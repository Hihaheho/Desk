use bevy::prelude::*;
use physics::{DragState, Follow, FollowParams};
use terminal::Cursor;

pub fn toggle_follow_for_drag_state(
    mut commands: Commands,
    cursors: Query<(Entity, &Transform), With<Cursor>>,
    drags: Query<(Entity, &Transform, &DragState), Changed<DragState>>,
) {
    let (cursor, cursor_vec) = if let Ok((entity, transform)) = cursors.get_single() {
        (entity, transform.translation.truncate())
    } else {
        return;
    };
    for (entity, transform, drag_state) in drags.iter() {
        match drag_state {
            DragState::Dragging => {
                let follow: Follow<Entity> = Follow {
                    target: cursor,
                    parameters: FollowParams {
                        position_offset: cursor_vec - transform.translation.truncate(),
                        ignore_area_size: 5.0,
                        velocity_coefficient: 10.0,
                        velocity_power: 1.2,
                        velocity_max: 2000.0,
                        ..Default::default()
                    },
                };
                commands.entity(entity).insert(follow);
            }
            DragState::NotDragging => {
                commands.entity(entity).remove::<Follow<Entity>>();
            }
        }
    }
}
