use std::borrow::Cow;

use async_trait::async_trait;
use ruma_identifiers::{DeviceId, UserId};
use sqlx::postgres::PgPool;
use sqlx::postgres::PgQueryAs;

use super::{Error, ErrorCode, Store};
use crate::models::auth::{PWHash, UserIdentifier};

// Convert from Sqlx errors to Store errors
impl From<sqlx::Error> for Error {
    #[inline]
    fn from(err: sqlx::Error) -> Self {
        let code = match err {
            sqlx::Error::Io(_) | sqlx::Error::PoolClosed | sqlx::Error::PoolTimedOut(_) => {
                ErrorCode::ConnectionFailed
            }
            sqlx::Error::RowNotFound => ErrorCode::RecordNotFound,
            //TODO: break out and match against database errors
            sqlx::Error::Database(_) | sqlx::Error::ColumnNotFound(_) => ErrorCode::InvalidSyntax,
            _ => ErrorCode::Unknown("Unknown error occurred.".to_string()),
        };

        Error { code }
    }
}
/// A Postgres Data Store
///
/// This implements the `Store` trait for Postgres.
#[derive(Clone)]
pub struct PostgresStore {
    pool: PgPool,
}

impl PostgresStore {
    /// Returns a new PostgresStore from database connection url.
    pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
        // TODO: Extract more config from env or such
        let pool = PgPool::builder()
            .max_size(5) // maximum number of connections in the pool
            .build(url)
            .await?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl Store for PostgresStore {
    fn get_type(&self) -> String {
        "PostgresStore".to_string()
    }

    async fn check_username_exists(&self, username: &str) -> Result<bool, Error> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM accounts where localpart = $1")
            .bind(username)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.0 > 0)
    }

    async fn fetch_user_id<'a>(
        &self,
        user_id: &'a UserIdentifier,
    ) -> Result<Option<Cow<'a, UserId>>, Error> {
        unimplemented!()
    }

    async fn fetch_password_hash(&self, user_id: &UserId) -> Result<PWHash, Error> {
        unimplemented!()
    }

    async fn fetch_display_name(&self, user_id: &UserId) -> Result<String, Error> {
        let row: (String,) =
            sqlx::query_as("SELECT display_name FROM account_profiles WHERE localpart = $1")
                .bind(user_id.localpart())
                .fetch_one(&self.pool)
                .await?;

        Ok(row.0)
    }

    async fn check_otp_exists(&self, user_id: &UserId, otp: &str) -> Result<bool, Error> {
        unimplemented!()
    }

    async fn check_device_id_exists(
        &self,
        user_id: &UserId,
        device_id: &DeviceId,
    ) -> Result<bool, Error> {
        let row: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM devices WHERE localpart = $1 AND device_id = $2")
                .bind(user_id.localpart())
                .bind(device_id)
                .fetch_one(&self.pool)
                .await?;

        Ok(row.0 > 0)
    }

    async fn remove_device_id(&self, device_id: &DeviceId, user_id: &UserId) -> Result<(), Error> {
        let _: (i64,) =
            sqlx::query_as("DELETE FROM devices WHERE localpart = $1 AND device_id = $2")
                .bind(user_id.localpart())
                .bind(device_id)
                .fetch_one(&self.pool)
                .await?;

        Ok(())
    }

    async fn remove_all_device_ids(&self, user_id: &UserId) -> Result<(), Error> {
        let _: (i64,) = sqlx::query_as("DELETE FROM devices WHERE localpart = $1")
            .bind(user_id.localpart())
            .fetch_one(&self.pool)
            .await?;

        Ok(())
    }

    async fn set_device<'a>(
        &self,
        user_id: &UserId,
        device_id: &DeviceId,
        display_name: Option<&str>,
    ) -> Result<(), Error> {
        unimplemented!()
    }
}
