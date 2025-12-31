use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::{db::Store};
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable,QueryableByName)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug)]
pub struct Users{
    pub id: i64,
    pub email: String,
    pub api_key: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Store {
    pub fn create_user(&mut self,username:String, password:String) -> Result<String,diesel::result::Error> {
        use crate::schema::user;
        
        let uuid = Uuid::new_v4();
        let numeric_uuid = uuid.as_i;
        let user = User {

        }


    }
}




