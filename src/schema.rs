table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        phonenumber -> Varchar,
        email_verified -> Bool,
        created_at -> Timestamptz,
        senha -> Varchar,
    }
}
