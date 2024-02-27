use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{self, Request},
    middleware::{self, Next},
    response::Response,
    routing::{get, patch, post},
    Json, Router,
};

use super::{
    models::{
        CreateModbusRegisterItemInput, ModbusRegister, ModbusRegisterQueryParams,
        ModbusRegisterResponse, UpdateModbusRegisterItemInput,
    },
    queries::{
        create_modbus_register_item, delete_modbus_register_item, list_modbus_register_items,
        update_modbus_register_item,
    },
    settings_queries,
};
use crate::{
    app_state::AppState,
    error::{Error, Result},
};

pub fn modbus_register_routes() -> Router<AppState> {
    let open_routes = Router::new()
        .route("/modbus-registers", get(list))
        .route("/modbus_register_settings", get(settings_queries::get_all));

    let protected_routes = Router::new()
        .route("/modbus-registers", post(create))
        .route("/modbus-registers/:id", patch(update).delete(delete))
        .route_layer(middleware::from_fn(require_auth));

    open_routes.merge(protected_routes)
}

async fn list(
    State(state): State<AppState>,
    Query(filters): Query<ModbusRegisterQueryParams>,
) -> Result<Json<ModbusRegisterResponse>> {
    let res = list_modbus_register_items(&state.conn, filters).await?;

    Ok(Json(res))
}

async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateModbusRegisterItemInput>,
) -> Result<Json<ModbusRegister>> {
    let item = create_modbus_register_item(&state.conn, payload).await?;

    Ok(Json(item))
}

async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateModbusRegisterItemInput>,
) -> Result<Json<ModbusRegister>> {
    let item = update_modbus_register_item(&state.conn, id, payload).await?;

    Ok(Json(item))
}

async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModbusRegister>> {
    let item = delete_modbus_register_item(&state.conn, id).await?;

    Ok(Json(item))
}

pub async fn require_auth(req: Request<Body>, next: Next) -> Result<Response> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .unwrap_or("");
    let secret = option_env!("API_SECRET_KEY").unwrap_or("secret");
    if auth_header != secret {
        return Err(Error::Unauthorized);
    }

    Ok(next.run(req).await)
}
