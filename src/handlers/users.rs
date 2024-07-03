use actix_web::{
    delete, get, post, put,
    web::{self, Json, Path},
};
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};

use crate::{
    db::{models::User, MoaDB},
    error::AppError,
    impl_actix_responder,
};

pub fn get_users_handers() -> actix_web::Scope {
    web::scope("/users")
        .service(create_user)
        .service(get_all_users)
        .service(update_user)
        .service(get_user)
        .service(delete_user)
}

#[derive(Debug, Deserialize)]
struct UserInfo {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
struct PutUser {
    pub email: Option<String>,
    pub password: Option<String>,
}

#[post("")]
async fn create_user(
    moa_db: web::Data<MoaDB>,
    Json(UserInfo { email, password }): Json<UserInfo>,
) -> Result<User, AppError> {
    if !EmailAddress::is_valid(&email) {
        return Err(AppError::BadRequest("Invalid Email Address".into()));
    }

    if password.is_empty() {
        return Err(AppError::BadRequest("Password cannot be empty".into()));
    }
    let password = pwhash::bcrypt::hash(&password)?;

    moa_db.create_user(&email, &password).await.map_err(|e| {
        if matches!(e, crate::db::Error::Duplicate) {
            AppError::BadRequest("Email is already in use".into())
        } else {
            AppError::DatabaseFailure(e)
        }
    })
}

#[derive(Debug, Serialize)]
pub struct Users {
    pub data: Vec<User>,
}
impl_actix_responder!(Users);

#[get("")]
async fn get_all_users(moa_db: web::Data<MoaDB>) -> Result<Users, AppError> {
    let data = moa_db.get_all_users().await?;
    Ok(Users { data })
}

#[get("/{email}")]
async fn get_user(moa_db: web::Data<MoaDB>, email: Path<String>) -> Result<User, AppError> {
    let user = moa_db.get_user(&email).await?;
    if let Some(u) = user {
        Ok(u)
    } else {
        Err(AppError::Gone("User Not Found".into()))
    }
}

#[put("/id/{id}")]
async fn update_user(
    moa_db: web::Data<MoaDB>,
    id: Path<i32>,
    Json(PutUser {
        email,
        mut password,
    }): Json<PutUser>,
) -> Result<User, AppError> {
    tracing::error!("hello");
    if let Some(ref email) = email {
        if !EmailAddress::is_valid(email) {
            return Err(AppError::BadRequest("Invalid Email Address".into()));
        }
    }

    if let Some(ref mut password) = password {
        if password.is_empty() {
            return Err(AppError::BadRequest("Password cannot be empty".into()));
        }
        *password = pwhash::bcrypt::hash(&password)?;
    }

    Ok(moa_db
        .update_user(*id, email.as_deref(), password.as_deref())
        .await?)
}

#[delete("")]
async fn delete_user(
    moa_db: web::Data<MoaDB>,
    Json(UserInfo { email, password }): Json<UserInfo>,
) -> Result<String, AppError> {
    let Some(user) = moa_db.get_user(&email).await? else {
        return Err(AppError::BadRequest("User not Found".into()));
    };

    if !pwhash::bcrypt::verify(&password, &user.password) {
        return Err(AppError::Unauthorized("Incorrect Password".into()));
    }

    let result = moa_db.delete_user(user.id).await?;
    if result == 1 {
        Ok("Successfully deleted user".into())
    } else {
        Err(AppError::BadRequest("User not Found".into()))
    }
}
