table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        phonenumber -> Varchar,
        verified -> Bool,
        created_at -> Timestamp,
        senha -> Varchar,
    }
}
