use chrono::{DateTime, Utc};
use sea_query::{ColumnDef, Iden, PostgresQueryBuilder, Table};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

use crate::utils::NullableU32;

#[derive(FromRow, ToSchema, Serialize, Debug, Clone)]
pub struct Task {
    #[sqlx(try_from = "i32")]
    pub id: u16,
    #[sqlx(try_from = "i64")]
    pub network: u32,
    pub owner: String,
    #[sqlx(try_from = "i32")]
    pub scenario_id: u16,
    #[sqlx(try_from = "Option<i64>")]
    #[schema(value_type = Option<u32>)]
    pub actor_id: NullableU32,
    #[sqlx(try_from = "i32")]
    pub script_idx: u16,
    pub on_chain_tasks: bool,
    pub off_chain_tasks: bool,
    #[schema(value_type = TaskStatus)]
    pub on_chain_status: sqlx::types::Json<TaskStatus>,
    #[schema(value_type = TaskStatus)]
    pub off_chain_status: sqlx::types::Json<TaskStatus>,
    pub tx_hash: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema, Deserialize, Clone, PartialEq)]
#[serde(tag = "code")]
pub enum TaskStatus {
    Waiting,
    Processing,
    Failed { reason_code: String },
    Finished,
}

#[derive(Iden)]
pub enum TaskIden {
    #[iden = "tasks"]
    Table,
    Id,
    Network,
    Owner,
    ScenarioId,
    ActorId,
    ScriptIdx,
    OnChainTasks,
    OffChainTasks,
    OnChainStatus,
    OffChainStatus,
    TxHash,
    UpdatedAt,
    CreatedAt,
}

pub fn get_create_statement() -> String {
    Table::create()
        .table(TaskIden::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(TaskIden::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(TaskIden::Network).big_integer().not_null())
        .col(ColumnDef::new(TaskIden::Owner).string_len(42).not_null())
        .col(ColumnDef::new(TaskIden::ScenarioId).integer().not_null())
        .col(ColumnDef::new(TaskIden::ActorId).big_integer())
        .col(ColumnDef::new(TaskIden::ScriptIdx).integer().not_null())
        .col(ColumnDef::new(TaskIden::OnChainTasks).boolean().not_null())
        .col(ColumnDef::new(TaskIden::OffChainTasks).boolean().not_null())
        .col(
            ColumnDef::new(TaskIden::OnChainStatus)
                .json_binary()
                .not_null(),
        )
        .col(
            ColumnDef::new(TaskIden::OffChainStatus)
                .json_binary()
                .not_null(),
        )
        .col(ColumnDef::new(TaskIden::TxHash).string_len(66))
        .col(
            ColumnDef::new(TaskIden::UpdatedAt)
                .timestamp_with_time_zone()
                .not_null(),
        )
        .col(
            ColumnDef::new(TaskIden::CreatedAt)
                .timestamp_with_time_zone()
                .not_null(),
        )
        .build(PostgresQueryBuilder)
}
