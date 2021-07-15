use physics::widget::component::{sugar as c, Component};
use terminal::terminal::Terminal;

pub fn render_terminal(_terminal: &Terminal) -> Component {
    c::vertical_array(vec![
        c::label("I'm your friend."),
        c::horizontal_array(vec![c::label(">"), c::input_string("command", "")]),
    ])
}
