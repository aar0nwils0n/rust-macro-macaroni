table! {
    movies (id) {
        id -> Int4,
        title -> Varchar,
    }
}

table! {
    showings (id) {
        id -> Int4,
        time -> Varchar,
        movieId -> Int4,
    }
}
