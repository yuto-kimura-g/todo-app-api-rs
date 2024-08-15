// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        due_date -> Nullable<Datetime>,
        is_done -> Bool,
    }
}
