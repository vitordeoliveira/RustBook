enum BlogState {
    Draft,
    PendingReview,
    Published,
}

pub struct Post {
    state: BlogState,
    content: String,
}

trait State {
    fn request_review(self: Box<Self>) -> BlogState;
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: BlogState::Draft,
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        use BlogState::*;
        match self.state {
            Draft => "",
            PendingReview => "",
            Published => &self.content,
        }
    }

    pub fn request_review(&mut self) {
        use BlogState::*;
        self.state = match self.state {
            Draft => PendingReview,
            PendingReview => Published,
            Published => Published,
        }
    }
}
