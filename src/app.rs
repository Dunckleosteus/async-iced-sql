use sqlx::PgPool;

use crate::{
    query::{Table, Traffic},
    Pages,
};

pub struct App {
    pub connected: bool,
    pub connection: Option<PgPool>,
    pub db_created: bool,
    pub current_page: Pages,
    pub tables: Option<Vec<Table>>,
    pub assise_material_list: Option<Vec<String>>,
    pub roulement_material_list: Option<Vec<String>>,
    pub chosen_assise_material: Option<String>,
    pub chosen_assise2_material: Option<String>,
    pub chosen_roulement_material: Option<String>,
    pub assise_thickness: Option<f32>,
    pub assise_thickness_input: String,
    pub roulement_thickness: Option<f32>,
    pub roulement_thickness_input: String,
    pub traffic_list: Option<Vec<Traffic>>,
    pub chosen_traffic: Option<String>,
}
