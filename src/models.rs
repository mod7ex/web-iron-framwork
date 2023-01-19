use uuid::Uuid;
use chrono::{DateTime, offset::Utc};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Post {
    title: String,
    body: String,
    author: String,
    timestamp: DateTime<Utc>,
    uuid: Uuid
}

impl Post {
    pub fn new(
        title: &str,
        body: &str,
        author: &str,
        timestamp: DateTime<Utc>,
        uuid: Uuid
    ) -> Post {
        Post {
            title: title.to_string(),
            body: body.to_string(),
            author: author.to_string(),
            timestamp,
            uuid,
        }
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }
}