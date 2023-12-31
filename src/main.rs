mod app;
mod messages;
mod page_emission2;
mod query;

use app::App;
use iced::widget::{button, column, pick_list, row, text, text_input, PickList, TextInput};
use iced::{executor, Alignment, Command, Length, Renderer, Theme};
use iced::{Application, Settings};
use messages::Messages;
use page_emission2::emmission2_page;
use query::{Material, Table, Traffic};
// this program will take a csv file as input and add it to a a database as a table
use sqlx::PgPool;
use std::fs;
// Constant values
const URL: &str = "postgres://postgres:266399@localhost:5432/postgres";
const DENSITE_ASSISE: f32 = 2.3;
const DENSITE_ROULEMENT: f32 = 2.252;

#[derive(Debug, Clone)]
pub enum Pages {
    DBManager,
    Tables,
    AddValues,
    CalculEmission2,
}
impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;
    type Message = Messages;
    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                connected: false,                // checks connection to database
                connection: None,                // variable storing connection to postgres database
                db_created: false,               // checks ot see id create_database was successful
                current_page: Pages::DBManager,  // the page the user is currently looking at
                tables: None,                    // tables contained in database
                assise_material_list: None,      // list of assise material queried from database
                roulement_material_list: None, // list of roulement materials queried from database
                chosen_assise_material: None,  // assise material chosen in drop down list
                chosen_assise2_material: None, // assise 2 material chosen in drop down list
                chosen_roulement_material: None, // roulement material chosen from drop down list
                assise_thickness: None,
                assise_thickness_input: String::from(""),
                roulement_thickness: None,
                roulement_thickness_input: String::from(""),
                traffic_list: None,
                chosen_traffic: None,
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
            Messages::TryAddValues => Command::none(),
            Messages::AddValues(res) => {
                if let Err(e) = res {
                    println!("Failed to add values: ");
                    println!("{:?}", e);
                    Command::none()
                } else {
                    Command::none()
                }
            }
            Messages::TryGetAssiseList => {
                if let Some(val) = &self.connection {
                    let conn = val.clone();
                    Command::perform(get_assise_list(conn), Messages::UpdateAssiseList)
                } else {
                    Command::none()
                }
            }
            Messages::UpdateAssiseList(x) => match x {
                Ok(val) => {
                    self.assise_material_list =
                        Some(val.iter().map(|y| y.name.clone()).collect::<Vec<String>>());
                    Command::none()
                }
                Err(err) => {
                    println!("{}", err);
                    Command::none()
                }
            },
            Messages::SelectAssiseMaterial(val) => {
                self.chosen_assise_material = Some(val);
                Command::none()
            }
            Messages::TryGetRoulementList => {
                if let Some(val) = &self.connection {
                    let conn = val.clone();
                    Command::perform(get_roulement_list(conn), Messages::UpdateRoulementList)
                } else {
                    Command::none()
                }
            }
            Messages::UpdateRoulementList(x) => match x {
                Ok(val) => {
                    let return_value = val
                        .iter()
                        .map(|y| -> String { y.name.clone() })
                        .collect::<Vec<String>>();
                    self.roulement_material_list = Some(return_value);
                    Command::none()
                }
                Err(err) => {
                    println!("{}", err);
                    Command::none()
                }
            },
            Messages::SelectRoulementMaterial(val) => {
                self.chosen_roulement_material = Some(val);
                Command::none()
            }
            Messages::TryGetList => {
                if let Some(val) = &self.connection {
                    let conn = val.clone();
                    Command::perform(get_traffic_list(conn), Messages::UpdateList)
                } else {
                    Command::none()
                }
            }
            Messages::UpdateList(x) => {
                match x {
                    Ok(val) => self.traffic_list = Some(val),
                    Err(e) => println!("failed to get traffic list from database: {}", e),
                }
                Command::none()
            }
            Messages::Select(x) => {
                self.chosen_traffic = Some(x);
                Command::none()
            }
            Messages::AssiseThicknessInputChanged(x) => {
                self.assise_thickness_input = x;
                Command::none()
            }
            Messages::ChangeAssiseThickness(x) => {
                match x.parse::<f32>() {
                    Ok(val) => self.assise_thickness = Some(val),
                    Err(e) => println!("{}", e),
                }
                Command::none()
            }
            Messages::RoulementThicknessInputChanged(x) => {
                self.roulement_thickness_input = x;
                Command::none()
            }
            Messages::ChangeRoulementThickness(x) => {
                match x.parse::<f32>() {
                    Ok(val) => self.roulement_thickness = Some(val),
                    Err(e) => println!("{}", e),
                }
                Command::none()
            }
        }
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {
        let mut col = column![row![
            button("DB manager page").on_press(Messages::ChangePage(Pages::DBManager)),
            button("Tables page").on_press(Messages::ChangePage(Pages::Tables)),
            button("Calcul emission 2").on_press(Messages::ChangePage(Pages::CalculEmission2)),
        ]];
        match self.current_page {
            Pages::DBManager => col = col.push(db_manager_page(&self)),
            Pages::Tables => col = col.push(tables_page(&self)),
            Pages::AddValues => col = col.push(add_values_page(&self)),
            Pages::CalculEmission2 => col = col.push(emmission2_page(&self)),
        }
        col.into()
    }
}

//=========================================GRAPHICAL==================================
// These functions are used to display the page the user is currently looking at
fn db_manager_page(app: &App) -> iced::Element<'static, Messages> {
    column![
        button("Connect to database").on_press(Messages::TryConnect),
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
fn add_values_page(app: &App) -> iced::Element<'static, Messages> {
    let mut col = column![];
    col.into()
}
//===========================================Main====================================================
fn main() {
    let _ = App::run(Settings::default());
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
async fn get_assise_list(conn: sqlx::PgPool) -> Result<Vec<Material>, String> {
    let q = "SELECT nom name FROM assise_material;";
    let query = sqlx::query_as::<_, Material>(q);
    match query.fetch_all(&conn).await {
        Ok(val) => Ok(val),
        Err(e) => Err(format!("{}", e)),
    }
}
async fn get_roulement_list(conn: sqlx::PgPool) -> Result<Vec<Material>, String> {
    let q = "SELECT nom name FROM roulement_material;";
    let query = sqlx::query_as::<_, Material>(q);
    match query.fetch_all(&conn).await {
        Ok(val) => Ok(val),
        Err(e) => Err(format!("{}", e)),
    }
}
async fn get_traffic_list(conn: sqlx::PgPool) -> Result<Vec<Traffic>, String> {
    let q = "SELECT nom name, traffic FROM traffic;";
    let query = sqlx::query_as::<_, Traffic>(q);
    match query.fetch_all(&conn).await {
        Ok(val) => Ok(val),
        Err(e) => Err(format!("{}", e)),
    }
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
