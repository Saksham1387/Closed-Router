use crate::Store;
use chrono::{NaiveDateTime,Utc};
use diesel::{prelude::*};
use uuid::Uuid;


#[derive(Debug)]
#[derive(Queryable, Selectable, Insertable,QueryableByName)]
#[diesel(table_name = crate::schema::request_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Requestlogs {
    id:String,
    model:String,
    prompt_tokens:Option<i32>,
    completion_tokens:Option<i32>,
    total_tokens:Option<i32>,
    status_code:i32,
    user_id:String,
    created_at:NaiveDateTime,
    error_message:Option<String>,
}

pub struct LogCreateInput{
    pub model:String,
    pub prompt_tokens:Option<i32>,
    pub completion_tokens:Option<i32>,
    pub total_tokens:Option<i32>,
    pub status_code:i32,
    pub user_id:String,
    pub error_messages_input:Option<String>   
}

impl Store {
    pub fn create_log(&mut self,data:LogCreateInput) -> Result<Requestlogs,diesel::result::Error> {
        use crate::schema::request_logs;

        let generated_uuid = Uuid::new_v4();

        let log = Requestlogs {
            id:generated_uuid.to_string(),
            model:data.model,
            prompt_tokens:data.prompt_tokens,
            completion_tokens:data.completion_tokens,
            total_tokens:data.total_tokens,
            status_code:data.status_code,
            user_id:data.user_id,
            created_at:Utc::now().naive_utc(),
            error_message:data.error_messages_input
        };

        let result = diesel::insert_into(request_logs::table)
            .values(&log)
            .returning(Requestlogs::as_returning())
            .get_result(&mut self.conn)?;

        Ok(result)
    }
}