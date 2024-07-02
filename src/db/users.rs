use diesel::{
    result::DatabaseErrorKind, ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper,
};
use diesel_async::RunQueryDsl;

use super::{
    models::{NewUser, User},
    schema::users,
    Error, MoaDB,
};

impl MoaDB {
    pub async fn create_user(&self, email: &str, password: &str) -> Result<User, Error> {
        let new_user = NewUser { email, password };

        let mut conn = self.get_conn().await?;
        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(|e| {
                if matches!(
                    e,
                    diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)
                ) {
                    Error::Duplicate
                } else {
                    Error::DieselFailure(e)
                }
            })
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut conn = self.get_conn().await?;
        users::table
            .select(User::as_select())
            .load(&mut conn)
            .await
            .map_err(|e| {
                if matches!(
                    e,
                    diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)
                ) {
                    Error::Duplicate
                } else {
                    Error::DieselFailure(e)
                }
            })
    }

    pub async fn get_user(&self, id: i32) -> Result<Option<User>, Error> {
        let mut conn = self.get_conn().await?;
        Ok(users::table
            .select(User::as_select())
            .filter(users::id.eq(id))
            .first(&mut conn)
            .await
            .optional()?)
    }
}
