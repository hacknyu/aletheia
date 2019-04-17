use super::RequestContext;
use crate::types::{Event, Project, User};
use juniper::FieldResult;

pub struct QueryRoot {}

graphql_object!(QueryRoot: RequestContext as "Query" |&self| {
    description: "The root query object of the schema"

    field users(
        &executor
    ) -> FieldResult<Vec<User>> {
        let database = &executor.context().conn;
        Ok(crate::resolvers::user::all(&database)?)
    }

    field projects(
        &executor
    ) -> FieldResult<Vec<Project>> {
        let database = &executor.context().conn;
        Ok(crate::resolvers::project::all(&database)?)
    }

    field projectBySlugAndEvent(
        &executor,
        slug: String,
        event_slug: String,
    ) -> FieldResult<Project> {
        let database = &executor.context().conn;
        Ok(crate::resolvers::project::get_by_slug_and_event(&slug, &event_slug, database)?)
    }

    field eventBySlug(
        &executor,
        slug: String
    ) -> FieldResult<Event> {
        let database = &executor.context().conn;
        Ok(crate::resolvers::event::get_by_slug(&slug, database)?)
    }

    field events(&executor) -> FieldResult<Vec<Event>> {
        let database = &executor.context().conn;
        Ok(crate::resolvers::event::all(&database)?)
    }

});
