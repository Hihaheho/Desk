mod drag_state;
pub mod event_handler;
pub mod shape;
pub mod widget;
use std::ops::Add;

use bevy_math::Vec2;

pub use drag_state::*;

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

pub struct Follow<T> {
    pub target: T,
    pub parameters: FollowParams,
}

pub struct FollowParams {
    pub position_offset: Vec2,
    pub ignore_area_size: f32,
    pub velocity_coefficient: f32,
    pub velocity_power: f32,
    pub velocity_max: f32,
    pub velocity_offset: f32,
}

impl Default for FollowParams {
    fn default() -> Self {
        Self {
            position_offset: Default::default(),
            ignore_area_size: 0.0,
            velocity_coefficient: 1.0,
            velocity_power: 1.0,
            velocity_max: 1000.0,
            velocity_offset: 0.0,
        }
    }
}

impl FollowParams {
    pub fn follow_vector(&self, me: &Vec2, target: &Vec2) -> Velocity {
        let vec = Vec2::new(target.x - me.x, target.y - me.y) - self.position_offset;
        let length = vec.length();
        if length <= self.ignore_area_size {
            return Vec2::ZERO.into();
        }
        let velocity =
            (length - self.velocity_offset).powf(self.velocity_power) * self.velocity_coefficient;
        let vec = vec.normalize() * velocity.min(self.velocity_max);
        vec.into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default() {
        let params = FollowParams::default();
        assert_nearly_eq(
            params
                .follow_vector(&Vec2::new(0.3, 2.), &Vec2::new(1.0, 0.5))
                .0,
            Vec2::new(0.7, -1.5),
        );
    }

    #[test]
    fn offset() {
        let params = FollowParams {
            position_offset: Vec2::new(1.0, 2.0),
            ..Default::default()
        };
        assert_nearly_eq(
            params
                .follow_vector(&Vec2::new(0.4, 2.), &Vec2::new(2.0, 3.5))
                .0,
            Vec2::new(0.6, -0.5),
        );
    }

    #[test]
    fn power() {
        let params = FollowParams {
            velocity_power: 1.0,
            ..Default::default()
        };
        assert_nearly_eq(
            params
                .follow_vector(&Vec2::new(0.0, 0.0), &Vec2::new(3.0, 4.0))
                .0,
            Vec2::new(3.0, 4.0),
        );

        let params = FollowParams {
            velocity_power: 2.0,
            ..Default::default()
        };
        assert_nearly_eq(
            params
                .follow_vector(&Vec2::new(0.0, 0.0), &Vec2::new(3.0, 4.0))
                .0,
            Vec2::new(15.0, 20.0),
        );
    }

    #[test]
    fn ignore_velocity() {
        let params = FollowParams {
            ignore_area_size: 5.0,
            ..Default::default()
        };
        assert_nearly_eq(
            params
                .follow_vector(&Vec2::new(0.0, 0.0), &Vec2::new(3.0, 4.0))
                .0,
            Vec2::new(0.0, 0.0),
        );
        assert_nearly_eq(
            params
                .follow_vector(&Vec2::new(0.0, 0.0), &Vec2::new(3.0, 4.1))
                .0,
            Vec2::new(3.0, 4.1),
        );
    }

    #[test]
    fn max_velocity() {
        let params = FollowParams {
            velocity_max: 5.0,
            ..Default::default()
        };
        assert_nearly_eq(
            params
                .follow_vector(&Vec2::new(0.0, 0.0), &Vec2::new(3.0, 4.0))
                .0,
            Vec2::new(3.0, 4.0),
        );
        assert_nearly_eq(
            params
                .follow_vector(&Vec2::new(0.0, 0.0), &Vec2::new(3.03, 4.04))
                .0,
            Vec2::new(3.0, 4.0),
        );
    }

    #[test]
    fn velocty_offset() {
        let params = FollowParams {
            velocity_offset: 1.0,
            ..Default::default()
        };
        assert_nearly_eq(
            params
                .follow_vector(&Vec2::new(0.0, 0.0), &Vec2::new(3.0, 4.0))
                .0,
            Vec2::new(3.0 * 4.0 / 5.0, 4.0 * 4.0 / 5.0),
        );
    }

    #[test]
    fn velocty_coefficient() {
        let params = FollowParams {
            velocity_coefficient: 2.0,
            ..Default::default()
        };
        assert_nearly_eq(
            params
                .follow_vector(&Vec2::new(0.0, 0.0), &Vec2::new(1.0, 1.0))
                .0,
            Vec2::new(2.0, 2.0),
        );
    }

    fn assert_nearly_eq(vec1: Vec2, vec2: Vec2) {
        let diff = vec1 - vec2;
        if diff.x.abs() > 0.0001 || diff.y.abs() > 0.0001 {
            assert_eq!(vec1, vec2);
        }
    }

    #[test]
    #[should_panic]
    fn assert_nearly_eq_fail() {
        assert_nearly_eq(Vec2::new(1.0, 1.0), Vec2::new(1.0, 1.0 + 0.001));
    }

    #[test]
    #[should_panic]
    fn assert_nearly_eq_fail2() {
        assert_nearly_eq(Vec2::new(1.0, 1.0), Vec2::new(1.0 + 0.001, 1.0));
    }
}
