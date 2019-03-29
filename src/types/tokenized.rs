use super::{Context, Event, Project};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Tokenized<T> {
    pub payload: T,
    pub token: String,
}

graphql_object!(Tokenized<Project>: Context as "AuthenticatedProject" |&self| {
    field token() -> &str {
        &self.token
    }

    field project() -> &Project {
        &self.payload
    }
});

graphql_object!(Tokenized<Event>: Context as "AuthenticatedEvent" |&self| {
    field token() -> &str {
        &self.token
    }

    field event() -> &Event {
        &self.payload
    }
});
