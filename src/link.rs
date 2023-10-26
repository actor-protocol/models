use chrono::{DateTime, Utc};
use sea_query::{ColumnDef, Iden, PostgresQueryBuilder, Table};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(FromRow, ToSchema, Serialize, Debug, Clone)]
pub struct Link {
    #[sqlx(try_from = "i32")]
    pub id: u16,
    pub name: String,
    pub link_type: String,
    pub owner: String,
    pub label: Option<String>,
    pub confirmation: Option<String>,
    #[serde(skip_serializing)]
    pub value: Option<String>,
    #[schema(value_type = LinkStatus)]
    pub status: sqlx::types::Json<LinkStatus>,

    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema, Deserialize, Clone)]
#[serde(tag = "code")]
pub enum LinkStatus {
    Waiting,
    Connected,
}

#[derive(Iden)]
pub enum LinkIden {
    #[iden = "links"]
    Table,
    Id,
    Name,
    LinkType,
    Owner,
    Label,
    Confirmation,
    Value,
    Status,
    UpdatedAt,
    CreatedAt,
}

pub fn get_create_statement() -> String {
    Table::create()
        .table(LinkIden::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(LinkIden::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(LinkIden::Name).string().not_null())
        .col(ColumnDef::new(LinkIden::LinkType).string().not_null())
        .col(ColumnDef::new(LinkIden::Owner).string_len(42).not_null())
        .col(ColumnDef::new(LinkIden::Label).string())
        .col(ColumnDef::new(LinkIden::Confirmation).string())
        .col(ColumnDef::new(LinkIden::Value).string())
        .col(ColumnDef::new(LinkIden::Status).json_binary().not_null())
        .col(
            ColumnDef::new(LinkIden::UpdatedAt)
                .timestamp_with_time_zone()
                .not_null(),
        )
        .col(
            ColumnDef::new(LinkIden::CreatedAt)
                .timestamp_with_time_zone()
                .not_null(),
        )
        .build(PostgresQueryBuilder)
}
