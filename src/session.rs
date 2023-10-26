use chrono::{DateTime, Utc};
use sea_query::{ColumnDef, Iden, PostgresQueryBuilder, Table};
use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(FromRow, ToSchema, Serialize, Debug)]
pub struct Session {
    #[sqlx(try_from = "i32")]
    pub id: u16,
    pub address: String,
    pub signature: String,
    pub timestamp: String,
    #[serde(skip_serializing)]
    pub ip_address: String,
    #[sqlx(try_from = "i64")]
    pub nonce: u32,
    pub token: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Iden)]
pub enum SessionIden {
    #[iden = "sessions"]
    Table,
    Id,
    Address,
    Signature,
    Timestamp,
    IpAddress,
    Nonce,
    Token,
    CreatedAt,
}

pub fn get_create_statement() -> String {
    Table::create()
        .table(SessionIden::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(SessionIden::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(SessionIden::Address)
                .string_len(42)
                .not_null(),
        )
        .col(
            ColumnDef::new(SessionIden::Signature)
                .string_len(132)
                .not_null(),
        )
        .col(ColumnDef::new(SessionIden::Timestamp).string().not_null())
        .col(ColumnDef::new(SessionIden::IpAddress).string().not_null())
        .col(ColumnDef::new(SessionIden::Nonce).big_integer().not_null())
        .col(ColumnDef::new(SessionIden::Token).string().not_null())
        .col(
            ColumnDef::new(SessionIden::CreatedAt)
                .timestamp_with_time_zone()
                .not_null(),
        )
        .build(PostgresQueryBuilder)
}
