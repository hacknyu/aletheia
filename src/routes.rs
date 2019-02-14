pub mod projects {
    use crate::models::Project;
    use crate::types::{DbConn, InsertableProject, Result};
    use rocket::{get, post};
    use rocket_contrib::json::Json;

    #[get("/")]
    pub fn index(conn: DbConn) -> Result<Json<Vec<Project>>> {
        Ok(Json(crate::controllers::projects_controller::all(&conn)?))
    }

    #[post("/", format = "application/json", data = "<project>")]
    pub fn create(conn: DbConn, project: Json<InsertableProject>) -> Result<Json<Project>> {
        let project = project.into_inner();
        Ok(Json(crate::controllers::projects_controller::insert(
            project, &conn,
        )?))
    }
}

pub mod users {
    use crate::models::User;
    use crate::types::{DbConn, InsertableUser, Result};
    use rocket::{get, post};
    use rocket_contrib::json::Json;

    #[get("/")]
    pub fn index(conn: DbConn) -> Result<Json<Vec<User>>> {
        Ok(Json(crate::controllers::users_controller::all(&conn)?))
    }

    #[post("/", format = "application/json", data = "<user>")]
    pub fn create(conn: DbConn, user: Json<InsertableUser>) -> Result<Json<User>> {
        let user = user.into_inner();
        Ok(Json(crate::controllers::users_controller::insert(
            user, &conn,
        )?))
    }
}
