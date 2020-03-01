#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

extern crate dotenv;

use std::collections::HashMap;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use rocket_contrib::json::{Json};
use serde_json;
// use rocket::outcome::*;
table! {
    movies (id) {
        id -> Integer,
        title -> Varchar,
    }
}

table! {
    showings (id) {
        id -> Integer,
        time -> Varchar,
        movie_id -> Integer,
    }
}

joinable!(showings -> movies (id));
allow_tables_to_appear_in_same_query!(showings, movies);

#[derive(PartialEq, Identifiable, Queryable, Serialize)]
#[table_name="movies"]
pub struct Movie {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable, Deserialize, Clone)]
#[table_name="movies"]
pub struct NewMovie {
    pub title: String
}

#[derive(Queryable, Serialize, Associations, PartialEq, Identifiable)]
#[belongs_to(Movie)]
#[table_name="showings"]
pub struct Showing {
    pub id: i32,
    pub movie_id : i32,
    pub time : String 
}

#[derive(Queryable, Serialize, Associations, PartialEq, Identifiable)]
#[belongs_to(Movie)]
#[table_name="showings"]
pub struct JoinShowing {
    pub id: Option<i32>,
    pub movie_id : Option<i32>,
    pub time : Option<String> 
}

#[derive(Insertable, Deserialize, Clone)]
#[table_name="showings"]
pub struct NewShowing {
    pub movie_id : i32,
    pub time : String
}
#[derive(Serialize, Clone)]
struct MovieWithShowing {
    id: i32,
    title: String,
    showings : Vec<String>
}



pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


#[post("/movie", format = "application/json", data = "<new_movie>")]
fn post_movie(new_movie: Json<NewMovie>) -> Json<String> {
    let conn = establish_connection();

    let row = diesel::insert_into(movies::table)
        .values(&new_movie.clone())
        .execute(&conn);

    Json(serde_json::to_string(&Movie{
        id: 1,
        title: new_movie.title.clone()
    }).unwrap())
}

#[post("/showing", format = "application/json", data = "<new_showing>")]
fn post_showing(new_showing: Json<NewShowing>) -> Json<String> {
    let conn = establish_connection();
    let row = diesel::insert_into(showings::table)
        .values(new_showing.clone())
        .execute(&conn);

    Json(serde_json::to_string(&Showing{
        id: 2,
        movie_id: new_showing.movie_id.clone(),
        time: new_showing.time.clone(),
    }).unwrap())
}

#[get("/movie/<id>")]
fn get_movie(id : i32) -> Json<String> {
    let conn = establish_connection();
    let rows: Vec<(Movie, JoinShowing)> = movies::table
        .left_outer_join(showings::table)
        .select((
            (movies::id, movies::title),
            (showings::id.nullable(), showings::movie_id.nullable(), showings::time.nullable())
        ))
        .filter(movies::id.eq(id))
        .load(&conn)
        .expect("error");

    let movies : Vec<MovieWithShowing> = rows.into_iter()
        .fold(HashMap::new(), |sum : HashMap<i32, MovieWithShowing>, (movie, showing) | {
            let movie = sum
            .clone()
            .get(&movie.id)
            .map(|x| x.clone())
            .map(|row| {
                let row_clone = row.clone();
                let new_movie = MovieWithShowing {
                    showings : showing.time
                        .map(|showing| {
                            row_clone.showings.clone().push(showing);
                            row_clone.showings
                        })
                        .unwrap_or(row.showings),
                    title: row.title,
                    id: row.id,
                };
                new_movie
            })
            .unwrap_or(MovieWithShowing { id: movie.id, title: movie.title, showings: Vec::new() });
            let mut new_sum = sum.clone();
            new_sum.insert(movie.id, movie);
            new_sum
        })
        .values()
        .map(|x| x.clone())
        .collect();

    Json(serde_json::to_string(&movies).unwrap())
}


fn main() {
    rocket::ignite()
        .mount("/", routes![post_movie])
        .launch();
}