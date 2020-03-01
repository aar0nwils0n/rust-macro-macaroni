
#[derive(Queryable)]
pub struct Movie {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable)]
#[table_name="movies"]
pub struct NewMovie {
    pub title: String
}

pub struct Showing {
    pub id: i32,
    pub movieId : i32,
    pub time : String 
}

#[table_name="showings"]
pub struct NewShowing {
    pub movieId : i32,
    pub time : String
}
