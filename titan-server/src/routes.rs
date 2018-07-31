use super::db::{UnksoMainForums, TitanPrimary};
use super::models;
use rocket_contrib::Json;
use super::schema::{wcf1_user, wcf_user_group, titan_branches};
use diesel::RunQueryDsl;
use diesel::query_dsl::filter_dsl::FindDsl;
use diesel::prelude::*;

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello from Project Titan!"
}

joinable!(wcf1_user -> wcf_user_group (user_online_group_id));

#[get("/")]
pub fn search_users(unkso_main: UnksoMainForums, unkso_titan: TitanPrimary) -> Json<models::TitanBranch> {
    use super::schema::titan_branches::dsl::*;
    let branch = titan_branches
        .filter(titan_branches::dsl::is_enabled.eq(true))
        .load::<models::TitanBranch>(&unkso_titan)
        .unwrap();

    Json(branch[0])
}

#[get("/<user_id>")]
pub fn find_user(user_id: i32, unkso_main: UnksoMainForums) -> Json<models::WcfUser> {
    let user = wcf1_user::table.find(user_id).first(&*unkso_main).unwrap();

    Json(user)
}