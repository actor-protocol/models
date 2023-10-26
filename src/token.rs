use chrono::{DateTime, Utc};
use sea_query::{ColumnDef, Iden, PostgresQueryBuilder, Table};
use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(FromRow, ToSchema, Serialize)]
pub struct Token {
    #[sqlx(try_from = "i32")]
    pub id: u16,
    #[sqlx(try_from = "i64")]
    pub network: u32,
    pub address: String,
    pub name: String,
    pub symbol: String,
    #[sqlx(try_from = "i32")]
    pub decimals: u16,
    pub icon: String,

    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Iden)]
pub enum TokenIden {
    #[iden = "tokens"]
    Table,
    Id,
    Network,
    Address,
    Name,
    Symbol,
    Decimals,
    Icon,
    UpdatedAt,
    CreatedAt,
}

pub fn get_create_statement() -> String {
    Table::create()
        .table(TokenIden::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(TokenIden::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(TokenIden::Network).big_integer().not_null())
        .col(ColumnDef::new(TokenIden::Address).string_len(42).not_null())
        .col(ColumnDef::new(TokenIden::Name).string().not_null())
        .col(ColumnDef::new(TokenIden::Symbol).string().not_null())
        .col(ColumnDef::new(TokenIden::Decimals).integer().not_null())
        .col(ColumnDef::new(TokenIden::Icon).string().not_null())
        .col(
            ColumnDef::new(TokenIden::UpdatedAt)
                .timestamp_with_time_zone()
                .not_null(),
        )
        .col(
            ColumnDef::new(TokenIden::CreatedAt)
                .timestamp_with_time_zone()
                .not_null(),
        )
        .build(PostgresQueryBuilder)
}
