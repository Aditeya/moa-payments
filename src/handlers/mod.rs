use actix_web::web;

mod users;
mod transactions;
mod balances;

pub fn get_all_handlers() -> actix_web::Scope {
    web::scope("/api/v1")
        .service(users::get_users_handers())
        .service(transactions::get_transactions_handers())
        .service(balances::get_balances_handers())
}
