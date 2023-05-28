
pub enum Status {
    Done,
    Doing,
    Lapsed,
    Future,
}

pub struct ChromeBookmark {
    text: String,
    link: String,
}

pub struct PDFReference {
    path: Vec<String>,
    topic_tags: Vec<String>,
    status: Status,
    project_tags: Vec<String>,
}
