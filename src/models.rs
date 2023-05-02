use chrono::NaiveDateTime;
use diesel::Insertable;
use crate::schema::students;


#[derive(Insertable)]
#[diesel(table_name = students)]
pub struct Student {
    pub id: String,
    pub name_student: String, 
    pub created_at: NaiveDateTime,
}