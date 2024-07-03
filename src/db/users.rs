use diesel::{
    result::DatabaseErrorKind, BoolExpressionMethods, ExpressionMethods, OptionalExtension,
    QueryDsl, SelectableHelper,
};
use diesel_async::RunQueryDsl;

use super::{
    models::{NewUser, PutUser, User},
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

    pub async fn get_user(&self, email: &str) -> Result<Option<User>, Error> {
        let mut conn = self.get_conn().await?;
        Ok(users::table
            .select(User::as_select())
            .filter(users::email.eq(email))
            .first(&mut conn)
            .await
            .optional()?)
    }

    pub async fn update_user(
        &self,
        id: i32,
        email: Option<&str>,
        password: Option<&str>,
    ) -> Result<User, Error> {
        let mut conn = self.get_conn().await?;
        let result = diesel::update(users::table.find(id))
            .set(&PutUser {
                email,
                password,
                updated_at: chrono::Utc::now(),
            })
            .returning(User::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(result)
    }

    pub async fn delete_user(&self, id: i32) -> Result<usize, Error> {
        let mut conn = self.get_conn().await?;
        Ok(diesel::delete(users::table.filter(users::id.eq(id)))
            .execute(&mut conn)
            .await?)
    }
}
