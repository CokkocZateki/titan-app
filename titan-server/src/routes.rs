use super::db::{UnksoMainForums, TitanPrimary};
use super::models;
use rocket_contrib::Json;/*
use diesel::RunQueryDsl;
use diesel::query_dsl::filter_dsl::FindDsl;*/
use diesel::prelude::*;

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello from Project Titan!"
}

// joinable!(wcf1_user -> wcf_user_group (user_online_group_id));

#[get("/")]
pub fn search_users(unkso_main: UnksoMainForums, unkso_titan: TitanPrimary) -> Json<Vec<models::TitanBranch>> {
    use schema::titan_branches::dsl::*;

    let branches = titan_branches
        .filter(is_enabled.eq(true))
        .load::<models::TitanBranch>(&*unkso_titan)
        .unwrap();

    Json(branch)
}

#[get("/<user_id>")]
pub fn find_user(user_id: i32, unkso_main: UnksoMainForums) -> Json<models::WcfUser> {
    use schema::wcf1_user::dsl::*;
    let user = wcf1_user.find(user_id).first(&*unkso_main).unwrap();

    Json(user)
}