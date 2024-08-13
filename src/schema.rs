// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        #[max_length = 256]
        id -> Varchar,
        title -> Text,
        description -> Nullable<Text>,
        due_date -> Nullable<Date>,
        is_done -> Bool,
    }
}
