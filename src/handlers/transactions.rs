use actix_web::{get, post, web};

pub fn get_transactions_handers() -> actix_web::Scope {
    web::scope("/transactions")
        .service(create_transaction)
        .service(get_transaction)
}

#[post("")]
async fn create_transaction() -> String {
    todo!()
}

#[get("")]
async fn get_transaction() -> String {
    todo!()
}
