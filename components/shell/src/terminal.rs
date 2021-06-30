use bevy_math::Vec2;
use physics::widget::{
    component::{sugar as c, Component},
    Widget,
};

#[non_exhaustive]
pub enum Command {
    Move {
        direction: Direction,
        distance: Distance,
    },
    FollowMe,
    Place {},
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub enum Distance {
    Pixel(f32),
}

pub struct Terminal {}
pub fn render_terminal(terminal: &Terminal, position: Vec2) -> Option<Widget> {
    Some(Widget {
        id: "desk shell".into(),
        position: position,
        shape: None,
        component: c::vertical_array(vec![
            c::label("I'm your friend."),
            c::horizontal_array(vec![c::label(">"), c::input_string("command", "")]),
        ]),
    })
}
