table! {
    wcf1_user (userID) {
        userID -> Integer,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        accessToken -> Varchar,
        languageID -> Integer,
        registrationDate -> Integer,
        styleID -> Integer,
        banned -> Bool,
        banReason -> Nullable<Mediumtext>,
        banExpires -> Integer,
        activationCode -> Integer,
        lastLostPasswordRequestTime -> Integer,
        lostPasswordKey -> Varchar,
        lastUsernameChange -> Integer,
        // newEmail -> Varchar,
        // oldUsername -> Varchar,
        // quitStarted -> Integer,
        // reactivationCode -> Integer,
        // registrationIpAddress -> Varchar,
        // avatarID -> Nullable<Integer>,
        // disableAvatar -> Bool,
        // disableAvatarReason -> Nullable<Text>,
        // disableAvatarExpires -> Integer,
        // enableGravatar -> Bool,
        // gravatarFileExtension -> Varchar,
        // signature -> Nullable<Text>,
        // signatureEnableBBCodes -> Bool,
        // signatureEnableHtml -> Bool,
        // signatureEnableSmilies -> Bool,
        // disableSignature -> Bool,
        // disableSignatureReason -> Nullable<Text>,
        // disableSignatureExpires -> Integer,
        // lastActivityTime -> Integer,
        // profileHits -> Integer,
        // rankID -> Nullable<Integer>,
        // userTitle -> Varchar,

        #[sql_name = "userOnlineGroupID"]
        user_online_group_id -> Nullable<Integer>,
        // activityPoints -> Integer,
        // notificationMailToken -> Varchar,
        // authData -> Varchar,
        // likesReceived -> Integer,
        // socialNetworkPrivacySettings -> Nullable<Text>,
        // wbbPosts -> Integer,
    }
}

table! {
    wcf_user_group (group_id) {
        group_id -> Integer,
        group_name -> Varchar,
    }
}

table! {
    wcf_rank (rank_id) {
        rank_id -> Integer,
        branch_id -> Integer,
        name -> Varchar,
        paygrade -> Varchar,
        image -> Varchar,
    }
}

table! {
    wcf_branch (branch_id) {
        branch_id -> Integer,
        name -> Varchar,
        image -> Varchar,
        rank_unavailable_image -> Varchar,
        is_disabled -> Bool,
    }
}

table! {
    titan_branches (id) {
        id -> Integer,
        name -> Varchar,
        wcf_user_group_id -> Integer,
        is_enabled -> Bool,
    }
}