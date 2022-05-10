use std::collections::HashMap;

use bevy::prelude::*;
use physics::{Follow, Velocity};

pub fn follow(
    follows: Query<&Follow<Entity>>,
    transforms: Query<&Transform>,
    mut velocities: Query<(&mut Velocity, &Transform, &Follow<Entity>)>,
) {
    let mut entities = Vec::new();
    for follow in follows.iter() {
        entities.push(follow.target);
    }
    let mut vecs = HashMap::new();
    for entity in entities {
        if let Ok(transform) = transforms.get(entity) {
            vecs.insert(entity, transform.translation.truncate());
        }
    }
    for (mut velocity, transform, follow) in velocities.iter_mut() {
        if let Some(target) = vecs.get(&follow.target) {
            let vec = transform.translation.truncate();
            *velocity = &*velocity + follow.parameters.follow_vector(&vec, target);
        }
    }
}
