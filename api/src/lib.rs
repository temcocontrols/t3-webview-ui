use utils::run_migrations;

pub mod app_state;
pub mod auth;
pub mod db_connection;
pub mod entity;
pub mod error;
pub mod modbus_register;
pub mod server;
pub mod user;
pub mod utils;

#[repr(C)]
pub enum RustError {
    Ok = 0,
    Error = 1,
    MigrationError = 2,
}

#[no_mangle]
pub extern "C" fn run_server() -> RustError {
    // Create a new tokio runtime for this function
    let runtime = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return RustError::Error,
    };

    // Run the server logic in a blocking thread
    let result = runtime.block_on(async {
        match run_migrations().await {
            Ok(_) => (),
            Err(err) => {
                // Handle migration errors here (log, convert to RustError)
                eprintln!("Migration error: {:?}", err);
                return RustError::MigrationError;
            }
        };
        match server::server_start().await {
            Ok(_) => RustError::Ok,
            Err(err) => {
                // Handle server errors here (log, convert to RustError)
                eprintln!("Server error: {:?}", err);
                RustError::Error
            }
        }
    });

    result
}
