use std::{sync::{Arc, Mutex}};
use std::time::{SystemTime, UNIX_EPOCH};

use poem::{
    Error, handler,
    http::StatusCode,
    web::{Data, Json},
};

use crate::{
    request_inputs::{CreateUser, SigninInput},
    request_outputs::{CreateUserOuput, SigninOutput},
};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use crate::db::Store;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: usize,
}

#[handler]
pub fn create_user(
    Json(data): Json<CreateUser>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Result<Json<CreateUserOuput>, Error> {
    let mut locked_s = s.lock().unwrap();

    let pass = data.password;
    let username = data.email;
    let db_result = locked_s
        .create_user(username, pass)
        .map_err(|_| Error::from_status(StatusCode::CONFLICT))?;

    let response = CreateUserOuput { id: db_result };

    Ok(Json(response))
}

#[handler]
pub fn signin(
    Json(data): Json<SigninInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Result<Json<SigninOutput>, Error> {
    let mut locked_s = s.lock().unwrap();
    let pass = data.password;
    let email = data.email;

    let db_result = locked_s.signin(email, pass);

    match db_result {
        Ok(user_id) => {
            let expiration = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() + (24 * 60 * 60);

            let my_claims = Claims {
                sub: user_id,
                exp: expiration as usize,
            };
            let token = encode(
                &Header::default(),
                &my_claims,
                &EncodingKey::from_secret("secret".as_ref()),
            )
            .map_err(|_| Error::from_status(StatusCode::UNAUTHORIZED))?;

            let response = SigninOutput { jwt: token };
            Ok(Json(response))
        }
        Err(_) => Err(Error::from_status(StatusCode::UNAUTHORIZED)),
    }
}