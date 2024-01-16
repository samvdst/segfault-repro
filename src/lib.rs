#![allow(unused_imports)]
use napi::Result;
use sqlx::MssqlPool;
use sqlx_oldapi::{types::chrono::Utc, SqlitePool};
use std::{fmt::Debug, sync::Arc};
use tokio::{sync::Mutex, task::block_in_place};
use uuid::Uuid;

#[macro_use]
extern crate napi_derive;
extern crate sqlx as sqlx_oldapi;
