mod drag_state;
pub mod event_handler;
mod follow;
pub mod shape;
pub mod widget;
use std::ops::Add;

use bevy_math::Vec2;

pub use drag_state::*;
pub use follow::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Velocity(pub Vec2);

impl From<Vec2> for Velocity {
    fn from(velocity: Vec2) -> Self {
        Velocity(velocity)
    }
}

impl Add<Vec2> for &Velocity {
    type Output = Velocity;

    fn add(self, rhs: Vec2) -> Self::Output {
        (self.0 + rhs).into()
    }
}

impl Add<Velocity> for &Velocity {
    type Output = Velocity;

    fn add(self, rhs: Velocity) -> Self::Output {
        (self.0 + rhs.0).into()
    }
}
