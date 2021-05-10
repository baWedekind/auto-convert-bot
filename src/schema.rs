table! {
    chatsettings (chat_id) {
        chat_id -> Int8,
        whole_message_always -> Bool,
    }
}

table! {
    definednames (chat_id, short_hand) {
        name_id -> Uuid,
        chat_id -> Int8,
        short_hand -> Varchar,
    }
}

table! {
    dictionary (name_id_source) {
        name_id_source -> Uuid,
        name_id_target -> Uuid,
        conversion_rate -> Float8,
    }
}

table! {
    longhands (name_id, long_hand) {
        name_id -> Uuid,
        long_hand -> Varchar,
    }
}

joinable!(definednames -> chatsettings (chat_id));

allow_tables_to_appear_in_same_query!(
    chatsettings,
    definednames,
    dictionary,
    longhands,
);
