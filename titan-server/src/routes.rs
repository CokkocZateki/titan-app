use super::db::{UnksoMainForums, TitanPrimary};
use super::models;
use rocket_contrib::Json;
use diesel::prelude::*;
use super::schema::*;

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello from Project Titan!"
}

joinable!(wcf1_user_to_group -> wcf1_user (user_id));
joinable!(wcf1_user_to_group -> wcf1_user_group (group_id));
allow_tables_to_appear_in_same_query!(wcf1_user_to_group, wcf1_user, wcf1_user_group);
// allow_tables_to_appear_in_same_query!(wcf1_user_to_group, wcf1_user_group);

#[get("/")]
pub fn search_users(unkso_main: UnksoMainForums, unkso_titan: TitanPrimary) -> Json<Vec<models::WcfUser>> {
     use schema::titan_branches::dsl::*;
     use schema::wcf1_user_to_group::dsl::*;

    let branches = titan_branches
        .filter(is_enabled.eq(true))
        .load::<models::TitanBranch>(&*unkso_titan)
        .unwrap();

    let users = wcf1_user_to_group
        .inner_join(wcf1_user::table)
        .inner_join(wcf1_user_group::table)
        .select(wcf1_user::all_columns)
        .filter(group_id.eq(40))
        .load::<models::WcfUser>(&*unkso_main)
        .unwrap();

    /*
    let debug = diesel::debug_query::<diesel::mysql::Mysql, _>(&users);
    let a = format!("{:?}", debug);
    println!("{:?}", a);
    */

    Json(users)
}

#[get("/<user_id>")]
pub fn find_user(user_id: i32, unkso_main: UnksoMainForums) -> Json<models::WcfUser> {
    use schema::wcf1_user::dsl::*;
    let user = wcf1_user.find(user_id).first(&*unkso_main).unwrap();

    Json(user)
}