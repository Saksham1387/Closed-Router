// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "provider_type"))]
    pub struct ProviderType;
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
    request_logs (id) {
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
        #[max_length = 36]
        id -> Varchar,
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
        #[max_length = 255]
        username -> Nullable<Varchar>,
        password_hash -> Nullable<Text>,
    }
}

diesel::joinable!(provider_api_keys -> providers (provider_id));
diesel::joinable!(provider_api_keys -> users (user_id));
diesel::joinable!(request_logs -> providers (provider_id));
diesel::joinable!(request_logs -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(provider_api_keys, providers, request_logs, users,);
