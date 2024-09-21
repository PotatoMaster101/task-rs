use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use actix_web::web::Data;
use alcoholic_jwt::{token_kid, validate, ValidJWT, Validation, JWKS};
use std::env;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, LazyLock, Mutex};
use cached::{Cached, TimedCache};
use crate::error::*;
use crate::repo::*;
use crate::user::{*, repo::*, requests::*};

const ADMIN: &str = "Admin";
const EMAIL: &str = "email";
const JWKS: &str = "jwks";
const ROLES: &str = "roles";
const SUB: &str = "sub";
const TOKEN: &str = "token";

static ENV_JWKS_URL: LazyLock<String> = LazyLock::new(|| {
    const JWKS_URL: &str = "JWKS_URL";
    env::var(JWKS_URL).unwrap_or_else(|_| panic!("{} not set", JWKS_URL))
});

static ENV_ISSUER: LazyLock<String> = LazyLock::new(|| {
    const ISSUER: &str = "ISSUER";
    env::var(ISSUER).unwrap_or_else(|_| panic!("{} not set", ISSUER))
});

static ENV_ROLES: LazyLock<String> = LazyLock::new(|| {
    const NAMESPACE: &str = "AUTH0_NAMESPACE";
    let var = env::var(NAMESPACE).unwrap_or_else(|_| panic!("{} not set", NAMESPACE));
    format!("{}/{}", var, ROLES)
});

static JWKS_CACHE: LazyLock<Arc<Mutex<TimedCache<String, JWKS>>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(TimedCache::with_lifespan(24 * 60 * 60)))
});

impl FromRequest for AuthedUser {
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
            let roles: Vec<_> = jwt
                .claims[&*ENV_ROLES]
                .as_array()
                .ok_or(ApiError::Unauthorized(ROLES))?
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();

            let user = repo.get_or_create(&CreateUserRequest {
                auth0_id: sub.to_owned(),
                email: email.to_owned(),
            }).await.map_err(|_| ApiError::ServerError)?;
            Ok(AuthedUser { user, roles })
        })
    }
}

impl FromRequest for AuthedAdmin {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let user = AuthedUser::from_request(req, payload);

        Box::pin(async move {
            let user = user.await?;
            if user.roles.contains(&String::from(ADMIN)) {
                Ok(AuthedAdmin(user))
            } else {
                Err(ApiError::Unauthorized(TOKEN))?
            }
        })
    }
}

async fn get_jwks() -> Result<JWKS, ApiError<'static>> {
    if let Some(result) = JWKS_CACHE.lock().map_err(|_| ApiError::ServerError)?.cache_get(&*ENV_JWKS_URL) {
        return Ok(result.clone());
    }

    let jwks = reqwest::get(&*ENV_JWKS_URL)
        .await
        .map_err(|_| ApiError::Unauthorized(JWKS))?
        .json::<JWKS>()
        .await
        .map_err(|_| ApiError::Unauthorized(JWKS))?;

    let mut cache = JWKS_CACHE.lock().map_err(|_| ApiError::ServerError)?;
    cache.cache_set(ENV_JWKS_URL.clone(), jwks.clone());
    Ok(jwks)
}

async fn get_jwt(token: &str) -> Result<ValidJWT, ApiError<'static>> {
    let jwks = get_jwks().await?;
    let kid = token_kid(token)
        .map_err(|_| ApiError::Unauthorized(TOKEN))?
        .ok_or(ApiError::Unauthorized(TOKEN))?;

    let validations = vec![Validation::Issuer(ENV_ISSUER.clone()), Validation::SubjectPresent, Validation::NotExpired];
    let jwk = jwks.find(&kid).ok_or(ApiError::Unauthorized(TOKEN))?;
    validate(token, jwk, validations).map_err(|_| ApiError::Unauthorized(TOKEN))
}
