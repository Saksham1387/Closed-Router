// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "provider_type"))]
    pub struct ProviderType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "website_status"))]
    pub struct WebsiteStatus;
}

diesel::table! {
    provider_api_keys (id) {
        id -> Int4,
        #[max_length = 255]
        user_id -> Varchar,
        provider_id -> Int4,
        api_key_encrypted -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ProviderType;

    providers (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        provider_type -> ProviderType,
        #[max_length = 255]
        api_endpoint -> Varchar,
        is_active -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    region (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    request_logs (id) {
        id -> Int4,
        #[max_length = 255]
        user_id -> Varchar,
        provider_id -> Int4,
        #[max_length = 100]
        model -> Varchar,
        prompt_tokens -> Nullable<Int4>,
        completion_tokens -> Nullable<Int4>,
        total_tokens -> Nullable<Int4>,
        status_code -> Int4,
        error_message -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user (id) {
        id -> Text,
        username -> Text,
        password -> Text,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 64]
        api_key -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 255]
        id -> Varchar,
    }
}

diesel::table! {
    website (id) {
        id -> Text,
        url -> Text,
        time_added -> Timestamp,
        user_id -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::WebsiteStatus;

    website_tick (id) {
        id -> Text,
        response_time_ms -> Int4,
        status -> WebsiteStatus,
        region_id -> Text,
        website_id -> Text,
        createdAt -> Timestamp,
    }
}

diesel::joinable!(provider_api_keys -> providers (provider_id));
diesel::joinable!(provider_api_keys -> users (user_id));
diesel::joinable!(request_logs -> providers (provider_id));
diesel::joinable!(request_logs -> users (user_id));
diesel::joinable!(website -> user (user_id));
diesel::joinable!(website_tick -> region (region_id));
diesel::joinable!(website_tick -> website (website_id));

diesel::allow_tables_to_appear_in_same_query!(
    provider_api_keys,
    providers,
    region,
    request_logs,
    user,
    users,
    website,
    website_tick,
);
