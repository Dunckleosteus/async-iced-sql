use sqlx::PgPool;

use crate::{
    query::{Material, Table, Traffic},
    Pages,
};

#[derive(Debug, Clone)]
pub enum Messages {
    TryConnect,                                         // try to connect to database
    Connected(Result<PgPool, String>),                  // check to see if connection successful
    TryCreateDB,               // try to create and fill db (only works when connected)
    CreatedDB(Result<(), ()>), // checks if database connection successful
    ChangePage(Pages),         // change page
    TryListTables,             // try to get get a list of all tables
    ListTables(Result<Vec<Table>, ()>), // result of TryListTables
    TryAddValues,              // try to add values to table TODO: Implement
    AddValues(Result<(), String>), // result of TryAddValues TODO: Implement
    TryGetAssiseList,          // try to query database for assise list
    UpdateAssiseList(Result<Vec<Material>, String>), // resut of TryGetAssiseList
    TryGetRoulementList,       // try to get list of roulement materials
    UpdateRoulementList(Result<Vec<Material>, String>), // updateRoulementList array based
    SelectAssiseMaterial(String), // user selected assise material list from dropdownlist
    SelectRoulementMaterial(String), // user selected Roulement material list from dropdownlist
    TryGetList,
    UpdateList(Result<Vec<Traffic>, String>),
    Select(String), // user selected assise material list from dropdownlist
    // drop down list
    AssiseThicknessInputChanged(String), // text input for assise thickness
    ChangeAssiseThickness(String),
    RoulementThicknessInputChanged(String), // text input for assise thickness
    ChangeRoulementThickness(String),
}
