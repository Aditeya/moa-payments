use actix_web::{delete, get, post, put, web};

pub fn get_users_handers() -> actix_web::Scope {
    web::scope("/users")
        .service(create_user)
        .service(get_user)
        .service(update_user)
        .service(delete_user)
}

#[post("")]
async fn create_user() -> String {
    todo!()
}

#[get("")]
async fn get_user() -> String {
    todo!()
}

#[put("")]
async fn update_user() -> String {
    todo!()
}

#[delete("")]
async fn delete_user() -> String {
    todo!()
}
