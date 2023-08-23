use sqlx::{FromRow, PgPool};
#[derive(Debug, FromRow)]
struct Structure {
    id: i32,
    name: String,
}
#[derive(Debug, Clone, FromRow)]
pub struct Material {
    pub name: String,
}
#[derive(Debug, Clone, FromRow)]
pub struct Table {
    // this struct is used to display tables on a iced page
    pub name: String,
}
#[derive(Debug, Clone, FromRow)]
pub struct Traffic {
    pub name: String,
    pub traffic: String,
}
