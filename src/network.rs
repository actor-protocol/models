use sea_query::{ColumnDef, Iden, PostgresQueryBuilder, Table};
use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(FromRow, ToSchema, Serialize, Debug, Clone)]
pub struct Network {
    #[sqlx(try_from = "i32")]
    pub id: u16,
    #[sqlx(try_from = "i64")]
    pub chain_id: u32,
    pub code: String,
    pub name: String,
    pub symbol: String,
    pub icon: String,
    pub explorer: String,
    pub core_address: String,
    #[serde(skip_serializing)]
    pub rpc_url: String,
    #[sqlx(try_from = "i64")]
    #[serde(skip_serializing)]
    pub ticked_at: u32,
}

#[derive(Iden)]
pub enum NetworkIden {
    #[iden = "networks"]
    Table,
    Id,
    ChainId,
    Code,
    Name,
    Symbol,
    Icon,
    Explorer,
    CoreAddress,
    RpcUrl,
    TickedAt,
}

pub fn get_create_statement() -> String {
    Table::create()
        .table(NetworkIden::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(NetworkIden::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(NetworkIden::ChainId)
                .big_integer()
                .not_null(),
        )
        .col(ColumnDef::new(NetworkIden::Code).string().not_null())
        .col(ColumnDef::new(NetworkIden::Name).string().not_null())
        .col(ColumnDef::new(NetworkIden::Symbol).string().not_null())
        .col(ColumnDef::new(NetworkIden::Icon).string().not_null())
        .col(ColumnDef::new(NetworkIden::Explorer).string().not_null())
        .col(ColumnDef::new(NetworkIden::CoreAddress).string().not_null())
        .col(ColumnDef::new(NetworkIden::RpcUrl).string().not_null())
        .col(
            ColumnDef::new(NetworkIden::TickedAt)
                .big_integer()
                .not_null(),
        )
        .build(PostgresQueryBuilder)
}
