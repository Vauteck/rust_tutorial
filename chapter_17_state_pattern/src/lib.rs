
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    approved_once: bool,
    approved_twice: bool
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
            approved_once: false,
            approved_twice: false,
        }
    }
}

impl Post {
    pub fn add_text(&mut self, text: &str) {
        if self.state.as_ref().unwrap().can_change_content() {
            self.content.push_str(text);
        }
    }
}

impl Post {
    /*
    pub fn content(&self) -> &str {
        ""
    }
    */
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
}

impl Post {
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }

    pub fn approved(&mut self) {
        if !self.approved_once {
            self.approved_once = true;
            return;
        }

        if !self.approved_twice {
            self.approved_twice = true;
        }

        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn can_change_content(&self) -> bool {
        false
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct Draft {}

impl State for Draft {
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn can_change_content(&self) -> bool {
        true
    }
}
