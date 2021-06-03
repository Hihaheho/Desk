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
