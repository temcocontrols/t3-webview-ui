use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use sea_orm::entity::prelude::*;
use sea_orm::Set;
use sea_orm::TryIntoModel;

use crate::app_state::AppState;
use crate::entity::modbus_register_device_name_id_mapping as device_mappings;
use crate::entity::prelude::*;
use crate::error::{Error, Result};

use super::inputs::CreateDeviceNameIdMappingInput;

pub async fn get_all(State(state): State<AppState>) -> Result<Json<Vec<device_mappings::Model>>> {
    let results = ModbusRegisterDeviceNameIdMapping::find()
        .all(&state.conn)
        .await;
    match results {
        Ok(items) => Ok(Json(items)),
        Err(error) => Err(Error::DbError(error.to_string())),
    }
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<device_mappings::Model>> {
    let result = ModbusRegisterDeviceNameIdMapping::find_by_id(id)
        .one(&state.conn)
        .await
        .map_err(|error| Error::DbError(error.to_string()))
        .unwrap();
    match result {
        Some(item) => Ok(Json(item)),
        None => Err(Error::NotFound),
    }
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateDeviceNameIdMappingInput>,
) -> Result<Json<device_mappings::Model>> {
    let model = device_mappings::ActiveModel {
        name: Set(payload.name),
        id: Set(payload.id),
    };

    let res = ModbusRegisterDeviceNameIdMapping::insert(model.clone())
        .exec_with_returning(&state.conn)
        .await
        .map_err(|error| Error::DbError(error.to_string()))?;

    Ok(Json(res.try_into_model().unwrap()))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<device_mappings::Model>> {
    let setting = ModbusRegisterDeviceNameIdMapping::find_by_id(id)
        .one(&state.conn)
        .await
        .map_err(|error| Error::DbError(error.to_string()))?
        .ok_or(Error::NotFound)
        .map(Into::into)?;

    ModbusRegisterDeviceNameIdMapping::delete_by_id(id)
        .exec(&state.conn)
        .await
        .map_err(|error| Error::DbError(error.to_string()))
        .unwrap();

    Ok(Json(setting))
}
