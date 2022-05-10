use std::collections::HashMap;

use bevy::prelude::*;
use physics::{Follow, Velocity};

pub fn follow(
    mut query_set: QuerySet<(
        Query<&Follow<Entity>>,
        Query<&Transform>,
        Query<(&mut Velocity, &Transform, &Follow<Entity>)>,
    )>,
) {
    let mut entities = Vec::new();
    for follow in query_set.q0().iter() {
        entities.push(follow.target);
    }
    let mut vecs = HashMap::new();
    for entity in entities {
        if let Ok(transform) = query_set.q1().get(entity) {
            vecs.insert(entity, transform.translation.truncate());
        }
    }
    for (mut velocity, transform, follow) in query_set.q2_mut().iter_mut() {
        if let Some(target) = vecs.get(&follow.target) {
            let vec = transform.translation.truncate();
            *velocity = &*velocity + follow.parameters.follow_vector(&vec, target);
        }
    }
}
