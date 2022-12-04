use std::ops::Range;

pub struct TextualDiagnostics {
    pub title: String,
    pub reports: Vec<Report>,
}

pub struct Report {
    pub text: String,
    pub span: Range<usize>,
}
