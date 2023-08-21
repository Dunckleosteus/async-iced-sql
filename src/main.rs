use iced::widget::{button, column, Container};
use iced::{executor, Command, Theme};
use iced::{Application, Settings};
// this program will take a csv file as input and add it to a a database as a table
use serde::Deserialize;
use sqlx::{FromRow, PgPool};
use std::error::Error;
use std::fs;
const URL: &str = "postgres://postgres:266399@localhost:5432/postgres";

#[derive(Debug, FromRow)]
struct Structure {
    id: i32,
    name: String,
}
struct App {
    connected: bool,
    connection: Option<sqlx::PgPool>,
}
#[derive(Debug, Clone)]
pub enum Messages {
    TryConnect,
    Connected(Result<MyConnection, String>),
}
impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;
    type Message = Messages;
    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                connected: false,
                connection: None,
            },
            Command::none(),
        )
    }
    fn title(&self) -> String {
        String::from("DB manager")
    }
    fn update(&mut self, message: Messages) -> iced::Command<Messages> {
        match message {
            Messages::TryConnect => {
                Command::perform(MyConnection::connect_db(), Messages::Connected)
            }
            Messages::Connected(data) => Command::none(),
        }
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {
        column![button("hello").on_press(Messages::TryConnect)].into()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = App::run(Settings::default());
    let conn = match sqlx::postgres::PgPool::connect(URL).await {
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
    let a = get_structures(&conn).await?;
    println!("{:?}", a);
    Ok(())
}
#[derive(Debug, Clone)]
pub struct MyConnection {
    conn: PgPool,
}
impl MyConnection {
    async fn connect_db() -> Result<MyConnection, String> {
        let conn = match sqlx::postgres::PgPool::connect(URL).await {
            Ok(value) => {
                println!("Connected to database");
                MyConnection { conn: value }
            }
            Err(_) => {
                panic!("Failed to connect to database");
            }
        };
        Ok(conn)
    }
}
async fn get_structures(conn: &sqlx::PgPool) -> Result<Vec<Structure>, Box<dyn Error>> {
    let q = "SELECT * FROM structures";
    let query = sqlx::query_as::<_, Structure>(q);
    let row = query.fetch_all(conn).await?;
    Ok(row)
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
