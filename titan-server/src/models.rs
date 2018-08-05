use super::schema::{titan_branches};

#[derive(Serialize, Deserialize, Queryable)]
pub struct WcfUser {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub access_token: String,
    pub language_id: i32,
    pub registration_date: i32,
    pub style_id: i32,
    pub banned: bool,
    pub ban_reason: Option<String>,
    pub ban_expires: i32,
    pub activation_code: i32,
    pub last_lost_password_request_time: i32,
    pub lost_password_key: String,
    pub last_username_change: i32,
    // pub newEmail: String,
    // pub oldUsername: String,
    // pub quitStarted: i32,
    // pub reactivationCode: i32,
    // pub registrationIpAddress: String,
    // pub avatarID: Option<i32>,
    // pub disableAvatar: bool,
    // pub disableAvatarReason: Option<String>,
    // pub disableAvatarExpires: i32,
    // pub enableGravatar: bool,
    // pub gravatarFileExtension: String,
    // pub signature: Option<String>,
    // pub signatureEnableBBCodes: bool,
    // pub signatureEnableHtml: bool,
    // pub signatureEnableSmilies: bool,
    // pub disableSignature: bool,
    // pub disableSignatureReason: Option<String>,
    // pub disableSignatureExpires: i32,
    // pub lastActivityTime: i32,
    // pub profileHits: i32,
    // pub rankID: Option<i32>,
    // pub userTitle: String,
    pub user_online_group_id: Option<i32>,
    // pub activityPoints: i32,
    // pub notificationMailToken: String,
    // pub authData: String,
    // pub likesReceived: i32,
    // pub socialNetworkPrivacySettings: Option<String>,
    // pub wbbPosts: i32
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct WcfUserGroup {
    pub group_id: i32,
    pub group_name: String
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct WcfUserToGroup {
    pub group_id: i32,
    pub user_id: i32
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct WcfUserActivityEvent {
    pub event_id: i32,
    pub user_id: i32,
    pub time: i32
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct WcfRank {
    pub rank_id: i32,
    pub branch_id: i32,
    pub paygrade: String,
    pub name: String,
    pub image: String
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct WcfBranch {
    pub branch_id: i32,
    pub name: String,
    pub image: String,
    pub rank_unavailable_image: String,
    pub is_disabled: bool
}

#[derive(Identifiable, Serialize, Deserialize, Queryable)]
#[table_name="titan_branches"]
pub struct TitanBranch {
    pub id: i32,
    pub name: String,
    pub wcf_user_group_id: i32,
    pub is_enabled: bool
}