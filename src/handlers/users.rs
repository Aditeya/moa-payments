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
        .service(get_user)
        .service(update_user)
        .service(delete_user)
}

#[derive(Debug, Deserialize)]
struct NewUser {
    pub email: String,
    pub password: String,
}

#[post("")]
async fn create_user(
    moa_db: web::Data<MoaDB>,
    Json(NewUser { email, password }): Json<NewUser>,
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

#[get("/{id}")]
async fn get_user(moa_db: web::Data<MoaDB>, id: Path<i32>) -> Result<User, AppError> {
    let user = moa_db.get_user(*id).await?;
    if let Some(u) = user {
        Ok(u)
    } else {
        Err(AppError::Gone("User Not Found".into()))
    }
}

#[put("")]
async fn update_user() -> String {
    todo!()
}

#[delete("")]
async fn delete_user() -> String {
    todo!()
}
