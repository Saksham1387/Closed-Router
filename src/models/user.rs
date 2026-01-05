use diesel::{prelude::*,result::Error};
use chrono::{NaiveDateTime, Utc};
use crate::{db::Store};
use uuid::Uuid;

#[derive(Debug)]
#[derive(Queryable, Selectable, Insertable,QueryableByName)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users{
    pub id: String,
    pub email: String,
    pub api_key: String,
    pub password_hash: Option<String>,
    pub username: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Store {
    pub fn create_user(&mut self,email:String, password:String) -> Result<String,diesel::result::Error> {
        use crate::schema::users;
        
        let api_key = String::from("fjisojfidsioj");

        let generated_uuid = Uuid::new_v4();
        let user = Users {
            id: generated_uuid.to_string(),
            email,
            api_key,
            password_hash:Some(password),
            username: Some(String::from("saksham")),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc()
        };

        diesel::insert_into(users::table)
            .values(&user)
            .returning(Users::as_returning())
            .get_result(&mut self.conn)?;

        Ok(generated_uuid.to_string())
    }


    pub fn signin(&mut self,input_email:String, password:String) -> Result<String,diesel::result::Error> {
        use crate::schema::users::dsl::*;

        let user_result = users
            .filter(email.eq(input_email))
            .select(Users::as_select())
            .first(&mut self.conn)?;

        if user_result.password_hash != Some(password) {
            return Err(Error::NotFound);
        }
        Ok(user_result.id)
    }

    pub fn verify_api_key(&mut self,input_api_key:String) -> (Option<String>, bool) {
        use crate::schema::users::dsl::*;

        let user_result = users
            .filter(api_key.eq(input_api_key))
            .select(id)
            .first(&mut self.conn).optional().unwrap();

        let is_valid = user_result.is_some();
        (user_result, is_valid)
    }   
}




