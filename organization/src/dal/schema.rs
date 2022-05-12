table! {
    groups (id) {
        id -> Uuid,
        organization_id -> Uuid,
        name -> Varchar,
    }
}

table! {
    organizations (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

table! {
    users_groups (user_id, group_id) {
        user_id -> Uuid,
        group_id -> Uuid,
    }
}

table! {
    users_organizations (user_id, organization_id) {
        user_id -> Uuid,
        organization_id -> Uuid,
    }
}

joinable!(groups -> organizations (organization_id));
joinable!(users_groups -> groups (group_id));
joinable!(users_groups -> users (user_id));
joinable!(users_organizations -> organizations (organization_id));
joinable!(users_organizations -> users (user_id));

allow_tables_to_appear_in_same_query!(
    groups,
    organizations,
    users,
    users_groups,
    users_organizations,
);
