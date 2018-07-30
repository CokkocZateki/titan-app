use super::db::{UnksoMainForums};
use super::models;
use rocket_contrib::Json;
use super::schema::wcf1_user;
use super::schema::wcf_user_group;
use diesel::RunQueryDsl;
use diesel::query_dsl::filter_dsl::FindDsl;

struct SearchModels {
    pub name: String,

}

joinable!(wcf1_user -> wcf_user_group (user_online_group_id));

/*
#[get("/")]
pub fn search(unkso_main: UnksoMainForums) -> Json<models::WcfUser> {
    let users = wcf1_user::table.inner_join
}
*/

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello from Project Titan!"
}

#[get("/<user_id>")]
pub fn test_database_conn(user_id: i32, unkso_main: UnksoMainForums) -> Json<models::WcfUser> {
    let user = wcf1_user::table.find(user_id).first(&*unkso_main).unwrap();

    Json(user)
}