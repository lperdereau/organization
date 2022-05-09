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
    users_organizations (user_id, organization_id) {
        user_id -> Uuid,
        organization_id -> Uuid,
    }
}

joinable!(groups -> organizations (organization_id));
joinable!(users_organizations -> organizations (organization_id));
joinable!(users_organizations -> users (user_id));

allow_tables_to_appear_in_same_query!(
    groups,
    organizations,
    users,
    users_organizations,
);
