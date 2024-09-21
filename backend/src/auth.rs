use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use alcoholic_jwt::{token_kid, validate, ValidJWT, Validation, JWKS};
use std::env;
use std::future::Future;
use std::pin::Pin;
use actix_web::web::Data;
use crate::error::*;
use crate::repo::*;
use crate::user::repo::*;
use crate::user::requests::*;
use crate::user::*;

const EMAIL: &str = "email";
const JWKS: &str = "jwks";
const SUB: &str = "sub";
const TOKEN: &str = "token";

impl FromRequest for User {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let bearer = BearerAuth::from_request(req, payload);
        let repo = req.app_data::<Data<UserRepository>>().unwrap().clone();

        Box::pin(async move {
            let bearer = bearer.await.map_err(|_| ApiError::Unauthorized(TOKEN))?;
            let jwt = get_jwt(bearer.token()).await?;
            let sub = jwt.claims[SUB].as_str().ok_or(ApiError::Unauthorized(SUB))?;
            let email = jwt.claims[EMAIL].as_str().ok_or(ApiError::Unauthorized(EMAIL))?;

            Ok(repo.get_or_create(&CreateUserRequest {
                auth0_id: sub.to_owned(),
                email: email.to_owned(),
            }).await.map_err(|_| ApiError::ServerError)?)
        })
    }
}

async fn get_jwt(token: &str) -> Result<ValidJWT, ApiError<'static>> {
    let jwks_url = env::var("JWKS_URL").expect("missing JWKS_URL");
    let issuer = env::var("ISSUER").expect("missing ISSUER");
    let jwks = reqwest::get(&jwks_url)
        .await
        .map_err(|_| ApiError::Unauthorized(JWKS))?
        .json::<JWKS>()
        .await
        .map_err(|_| ApiError::Unauthorized(JWKS))?;
    let kid = token_kid(token)
        .map_err(|_| ApiError::Unauthorized(TOKEN))?
        .ok_or(ApiError::Unauthorized(TOKEN))?;

    let validations = vec![Validation::Issuer(issuer), Validation::SubjectPresent, Validation::NotExpired];
    let jwk = jwks.find(&kid).ok_or(ApiError::Unauthorized(TOKEN))?;
    validate(token, jwk, validations).map_err(|_| ApiError::Unauthorized(TOKEN))
}
