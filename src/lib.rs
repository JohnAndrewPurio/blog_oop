pub enum ReviewStatus {
    Approved(Post),
    PendingReview(PendingReviewPost),
}

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
    approve_count: u8,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approve_count: 0,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(mut self) -> ReviewStatus {
        self.approve_count += 1;

        if self.approve_count < 2 {
            ReviewStatus::PendingReview(self)
        } else {
            ReviewStatus::Approved(Post {
                content: self.content,
            })
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_rejected() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");

        let post = post.request_review();
        let post = post.reject();

        assert!(matches!(post, DraftPost { .. }));
    }

    #[test]
    fn post_published() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");

        let post = post.request_review();
        let post = match post.approve() {
            ReviewStatus::PendingReview(p) => p,
            ReviewStatus::Approved(_) => panic!("Post requires at least 2 approve_count"),
        };

        let post = match post.approve() {
            ReviewStatus::PendingReview(_) => panic!("Post should be approved after 2 or more approve_count"),
            ReviewStatus::Approved(p) => p
        };

        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
