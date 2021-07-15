pub mod shape;
pub mod widget;
use bevy_math::Vec2;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Velocity(pub Vec2);

impl From<Vec2> for Velocity {
    fn from(velocity: Vec2) -> Self {
        Velocity(velocity)
    }
}
