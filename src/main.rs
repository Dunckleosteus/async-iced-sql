// this program will take a csv file as input and add it to a a database as a table
use sqlx::{Connection, PgPool};
use std::error::Error;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://postgres:266399@localhost:5432/postgres";
    let conn = match sqlx::postgres::PgPool::connect(url).await {
        Ok(value) => {
            println!("Connected to database");
            value
        }
        Err(_) => {
            panic!("Failed to connect to database");
        }
    };
    create_database(&conn).await?;
    fill_database(&conn).await?;
    Ok(())
}
async fn create_database(conn: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    // this function created creates all the tables in the database
    let query = match fs::read_to_string("sql/build.sql") {
        Ok(v) => v,
        Err(_) => panic!("Did not find the build.sql file used to create database"),
    };
    let queries = query.split(";").filter(|x| *x != "");
    for q in queries {
        // splitting each query by ';' and executing them one by one
        sqlx::query(&q).execute(conn).await?;
    }
    Ok(())
}
async fn fill_database(conn: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = match fs::read_to_string("sql/fill.sql") {
        Ok(v) => v,
        Err(_) => panic!("Could not find the fill.sql file"),
    };
    let queries = query.split(";").filter(|x| *x != "");
    for q in queries {
        sqlx::query(&q).execute(conn).await?;
    }
    Ok(())
}
