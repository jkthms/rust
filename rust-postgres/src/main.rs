use postgres::{Client, NoTls, Error};
use std::collections::HashMap;

struct Author {
    _id: i32,
    name: String,
    country: String,
}

fn main() -> Result<(), Error> {
    // Establish a connection to the local database
    let mut client = Client::connect("postgres://youruser:yourpassword@localhost:5432/library", NoTls)?;

    // Create new tables in the local database
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS author (
            id          SERIAL PRIMARY KEY,
            name        VARCHAR NOT NULL,
            country     VARCHAR NOT NULL
        )")?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS book (
            id          SERIAL PRIMARY KEY,
            title       VARCHAR NOT NULL,
            author_id   INTEGER NOT NULL REFERENCES author(id)
        )")?;

    // Insert new data into the tables
    let mut authors = HashMap::new();

    authors.insert(String::from("J.K. Rowling"), String::from("United Kingdom"));
    authors.insert(String::from("J.R.R. Tolkien"), String::from("United Kingdom"));
    authors.insert(String::from("George R.R. Martin"), String::from("United States"));
    authors.insert(String::from("Aldous Huxley"), String::from("United Kingdom"));

    // Loop through the HashMap and insert into the author table
    for (key, name) in &authors{
        let author = Author {
            _id: 0,
            name: key.to_string(),
            country: name.to_string(),
        };

        client.execute(
            "INSERT INTO author (name, country) VALUES ($1, $2)",
            &[&author.name, &author.country],
        )?;
    }

    // Query the author table and print the results to the console
    for row in client.query("SELECT id, name, country FROM author", &[])? {
        let author = Author {
            _id: row.get(0),
            name: row.get(1),
            country: row.get(2),
        };

        println!("{}: {}", author.name, author.country);
    }

    Ok(())
}
