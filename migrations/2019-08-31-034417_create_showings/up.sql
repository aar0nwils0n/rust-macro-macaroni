create table Showings (
    id integer not null primary key autoincrement,
    movie_id integer not null,
    time varchar(255) not null,
    FOREIGN KEY(movie_id) REFERENCES Movies(id)
)