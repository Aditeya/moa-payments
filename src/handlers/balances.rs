use actix_web::{delete, get, put, web};

pub fn get_balances_handers() -> actix_web::Scope {
    web::scope("/balances")
        .service(get_amount)
        .service(deposity_amount)
        .service(withdraw_amount)
}

#[get("")]
async fn get_amount() -> String {
    todo!()
}

#[put("")]
async fn deposity_amount() -> String {
    todo!()
}

#[delete("")]
async fn withdraw_amount() -> String {
    todo!()
}
