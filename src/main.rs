use iced::widget::{button, column, row, text};
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
#[derive(Debug, Clone, FromRow)]
pub struct Table {
    // this struct is used to display tables on a iced page
    pub name: String,
}
#[derive(Debug, Clone)]
pub enum Pages {
    DBManager,
    Tables,
}
struct App {
    connected: bool,
    connection: Option<PgPool>,
    db_created: bool,
    current_page: Pages,
    tables: Option<Vec<Table>>,
}
#[derive(Debug, Clone)]
pub enum Messages {
    TryConnect,
    Connected(Result<PgPool, String>),
    TryCreateDB,
    CreatedDB(Result<(), ()>),
    ChangePage(Pages),
    TryListTables,
    ListTables(Result<Vec<Table>, ()>),
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
                db_created: false,
                current_page: Pages::DBManager,
                tables: None,
            },
            Command::none(),
        )
    }
    fn title(&self) -> String {
        String::from("DB manager")
    }
    fn update(&mut self, message: Messages) -> iced::Command<Messages> {
        match message {
            Messages::TryConnect => Command::perform(connect_db(), Messages::Connected),
            Messages::Connected(conn) => {
                // This function waits for the message try connect to finish returning a message
                // and adds it to state.
                self.connected = true;
                match conn {
                    Ok(val) => {
                        self.connection = Some(val);
                    }
                    Err(_) => {}
                }
                Command::none()
            }
            Messages::TryCreateDB => match &self.connection {
                Some(val) => {
                    let conn = val.clone();
                    println!("Filling database");
                    Command::perform(create_database(conn), Messages::CreatedDB)
                }
                None => Command::none(),
            },
            Messages::CreatedDB(e) => {
                if let Ok(()) = e {
                    self.db_created = true
                } else {
                    self.db_created = false
                };
                Command::none()
            }
            Messages::ChangePage(page) => {
                self.current_page = page;
                Command::none()
            }
            Messages::TryListTables => match &self.connection {
                Some(val) => {
                    let conn = val.clone();
                    Command::perform(get_tables(conn), Messages::ListTables)
                }
                None => Command::none(),
            },
            Messages::ListTables(val) => {
                match val {
                    Ok(v) => self.tables = Some(v),
                    Err(()) => {
                        println!("get_tables returned and error");
                        self.tables = None
                    }
                };
                Command::none()
            }
        }
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {
        let mut col = column![row![
            button("DB manager page").on_press(Messages::ChangePage(Pages::DBManager)),
            button("Tables page").on_press(Messages::ChangePage(Pages::Tables)),
        ]];
        match self.current_page {
            Pages::DBManager => col = col.push(db_manager_page(&self)),
            Pages::Tables => col = col.push(tables_page(&self)),
        }
        col.into()
    }
}
fn db_manager_page(app: &App) -> iced::Element<'static, Messages> {
    column![
        button("hello").on_press(Messages::TryConnect),
        text(format!("Connected: {}", app.connected)),
        button("Create database tables").on_press(Messages::TryCreateDB),
        text(format!("Filled database: {}", app.db_created)),
    ]
    .into()
}
fn tables_page(app: &App) -> iced::Element<'static, Messages> {
    let mut col = column![button("List all tables").on_press(Messages::TryListTables)];
    if let Some(tables) = &app.tables {
        for table in tables.iter() {
            println!("{:?}", table);
            col = col.push(text(table.name.clone()));
        }
    }
    col.into()
}
//===========================================Main====================================================
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = App::run(Settings::default());
    Ok(())
}
// ===========================================Functions===============================================
async fn connect_db() -> Result<PgPool, String> {
    let conn = match sqlx::postgres::PgPool::connect(URL).await {
        Ok(value) => {
            println!("Connected to database");
            value
        }
        Err(_) => {
            panic!("Failed to connect to database");
        }
    };
    Ok(conn)
}
async fn get_tables(conn: sqlx::PgPool) -> Result<Vec<Table>, ()> {
    let q = "SELECT tablename name FROM pg_catalog.pg_tables tables WHERE tables.schemaname = 'public';";
    let query = sqlx::query_as::<_, Table>(q);
    match query.fetch_all(&conn).await {
        Ok(val) => Ok(val),
        Err(e) => {
            println!("{:?}", e);
            Err(())
        }
    }
}
async fn create_database(conn: sqlx::PgPool) -> Result<(), ()> {
    // this function created creates all the tables in the database
    let query = match fs::read_to_string("sql/build.sql") {
        Ok(v) => v,
        Err(_) => panic!("Did not find the build.sql file used to create database"),
    };
    let queries = query.split(";").filter(|x| *x != "");
    for q in queries {
        // splitting each query by ';' and executing them one by one
        if let Err(_) = sqlx::query(&q).execute(&conn).await {
            return Err(());
        }
    }
    // Filling database from fill.sql file
    let query = match fs::read_to_string("sql/fill.sql") {
        Ok(v) => v,
        Err(_) => panic!("Could not find the fill.sql file"),
    };
    let queries = query.split(";").filter(|x| *x != "");
    for q in queries {
        if let Err(_) = sqlx::query(&q).execute(&conn).await {
            return Err(());
        };
    }
    Ok(())
}
async fn get_structures(conn: &sqlx::PgPool) -> Result<Vec<Structure>, Box<dyn Error>> {
    let q = "SELECT * FROM structures";
    let query = sqlx::query_as::<_, Structure>(q);
    let row = query.fetch_all(conn).await?;
    Ok(row)
}
