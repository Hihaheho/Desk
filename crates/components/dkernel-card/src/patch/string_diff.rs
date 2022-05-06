
pub enum Operation {
    Delete,
    Insert,
    Equal,
}

pub struct StringDiff {
    pub operation: Operation,
    pub text: String,
}

pub struct StringPatch {
    pub diffs: Vec<StringDiff>,
    pub start1: i32,
    pub start2: i32,
    pub length1: i32,
    pub length2: i32,
}
