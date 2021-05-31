pub enum Widget {
    Window {
        title: String,
        ast: Option<Box<Widget>>,
        computed: Option<Box<Widget>>,
    },
}
