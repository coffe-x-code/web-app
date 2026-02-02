use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::product)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub published: bool,
}
