use std::{env, error::Error};

use axum::{
    http::StatusCode,
    routing::{get, get_service},
    Router,
};

use tokio::{net::TcpListener, signal, sync::mpsc};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

use crate::{
    app_state::{self, AppState},
    file::routes::file_routes,
    utils::{run_migrations, SHUTDOWN_CHANNEL, SPA_DIR},
};

use super::modbus_register::routes::modbus_register_routes;
use super::user::routes::user_routes;

use futures_util::{SinkExt, StreamExt};
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;

use tokio_tungstenite::accept_async;
use tokio_tungstenite::accept_async_with_config;
use tokio_tungstenite::accept_hdr_async;
use tokio_tungstenite::accept_hdr_async_with_config;

use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};

use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;

use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use uuid::Uuid;

fn routes_static() -> Router {
    Router::new().nest_service(
        "/",
        get_service(ServeDir::new(SPA_DIR.as_str())).handle_error(|_| async move {
            (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
        }),
    )
}

async fn health_check_handler() -> &'static str {
    println!("->> Health check");
    "OK"
}

// This function creates the application state and returns a router with all of the routes for the API.
pub async fn create_app(app_state: AppState) -> Result<Router, Box<dyn Error>> {
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    Ok(Router::new()
        .nest(
            "/api",
            modbus_register_routes()
                .merge(user_routes())
                .merge(file_routes())
                .route("/health", get(health_check_handler)),
        )
        .with_state(app_state)
        .fallback_service(routes_static())
        .layer(cors))
}

pub async fn server_start() -> Result<(), Box<dyn Error>> {
    // Initialize tracing
    if let Err(_) = tracing_subscriber::fmt().try_init() {
        // Handle the error or ignore it if reinitialization is not needed
    }

    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Run database migrations
    run_migrations().await?;

    // Create the application state
    let state = app_state::app_state().await?;

    // Create the application state
    let app = create_app(state.clone()).await?;

    // Get the server port from environment variable or default to 9103
    let server_port = env::var("PORT").unwrap_or_else(|_| "9103".to_string());

    // Bind the server to the specified port
    let listener = TcpListener::bind(format!("0.0.0.0:{}", &server_port)).await?;

    // Print the server address
    // println!("->> LISTENING on {:?}\n", listener.local_addr());

    start_websocket_server().await;

    // Start the server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(state))
        .await?;

    Ok(())
}

async fn shutdown_signal(state: AppState) {
    let (shutdown_tx, mut shutdown_rx) = mpsc::channel(1);

    // Store the sender in the SHUTDOWN_CHANNEL
    *SHUTDOWN_CHANNEL.lock().await = shutdown_tx;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
        _ = shutdown_rx.recv() => {}, // Listen for the shutdown signal
    }

    // Drop the database connection gracefully
    println!("->> SHUTTING DOWN: Closing database connection...");
    let _ = state.conn.lock().await; // Lock and drop the connection
}

type Clients = Arc<Mutex<Vec<(Uuid, tokio::sync::mpsc::UnboundedSender<Message>)>>>;

async fn handle_websocket(
    stream: TcpStream,
    clients: Clients,
    config: WebSocketConfig,
) -> Result<(), Box<dyn Error>> {
    /*
     println!("==Start handling websocket");
     println!("==Stream: {:?}", stream);
     println!("==Clients 1st: {:?}", clients);
     println!("");
    */

    log_message(&format!("==Start handling websocket"), true);
    log_message(&format!("==Stream: {:?}", stream), true);
    log_message(&format!("==Clients 1st: {:?}", clients), true);
    log_message(&format!(""), true);

    /*
    let ws_stream = match accept_async_with_config(stream, Some(config)).await {
        Ok(ws) => ws,
        Err(e) => {
            // println!("==Failed to accept websocket connection: {:?}", e);
            log_message(
                &format!("==Failed to accept websocket connection: {:?}", e),
                true,
            );
            return Err(Box::new(e));
        }
    };
    */

    let ws_stream = match accept_hdr_async_with_config(
        stream,
        |req: &Request, mut response: Response| {
            // Inspect and modify the request and response headers
            println!("Received a connection request: {:#?}", req);
            println!("Response with: {:#?}", response);
            // Optionally modify the response headers
            // response.headers_mut().insert(...);
            Ok(response)
        },
        Some(config),
    )
    .await
    {
        Ok(ws) => ws,
        Err(e) => {
            log_message(
                &format!("Failed to accept websocket connection: {:?}", e),
                true,
            );
            return Err(Box::new(e));
        }
    };

    // println!("==ws_stream: {:?}", ws_stream);

    let (mut write, mut read) = ws_stream.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Err(e) = write.send(msg).await {
                // println!("==Error sending message to client: {:?}", e);
                log_message(&format!("==Error sending message to client: {:?}", e), true);
                break;
            }
        }
    });

    // bind the client id to incoming session
    while let Some(msg) = read.next().await {
        let msg = msg?;

        // Log the frame size
        let info_msg = msg.clone();
        let frame_size = info_msg.into_data().len();
        log_message(&format!("==Frame size: {} bytes", frame_size), true);

        // println!("==Recived message 1st:{:?}", msg);
        log_message(&format!("==Recived message 1st:{:?}", msg), true);

        if msg.is_text() || msg.is_binary() {
            let msg_text = msg.to_text()?;

            let json_msg: serde_json::Value = match serde_json::from_str(msg_text) {
                Ok(json) => json,
                Err(_) => {
                    // println!("==1st message with incorrect format: {}", msg_text);
                    log_message(
                        &format!("==1st message with incorrect format: {}", msg_text),
                        true,
                    );
                    continue;
                }
            };

            // {"header":{"clientId":"-","from":"Firefox"},"message":{"action":-1,"clientId":"4aa7e8c2-437e-422c-a55d-e1ae4c757935"}}
            // {"header":{"clientId":"-","from":"Firefox"},"message":{"action":-1,"clientId":"11111111-1111-1111-1111-111111111111"}}
            if let Some(message) = json_msg.get("message") {
                if let Some(action) = message.get("action").and_then(|a| a.as_i64()) {
                    if action == 13 {
                        if let Some(client_id_str) =
                            message.get("clientId").and_then(|id| id.as_str())
                        {
                            let mut clients = clients.lock().unwrap();

                            if client_id_str == "11111111-1111-1111-1111-111111111111" {
                                clients.retain(|(id, _)| {
                                    *id != Uuid::parse_str("11111111-1111-1111-1111-111111111111")
                                        .unwrap()
                                });
                            }

                            let client_id = Uuid::parse_str(client_id_str)?;
                            clients.push((client_id, tx.clone()));
                        }
                    } else {
                        let clients = clients.lock().unwrap();
                        for (id, client) in clients.iter() {
                            if *id == Uuid::parse_str("11111111-1111-1111-1111-111111111111")? {
                                let text_message = Message::text(message.to_string());
                                if let Err(e) = client.send(text_message) {
                                    // println!("==Failed to send text msg to client ==1111: {:?}", e);
                                    log_message(
                                        &format!(
                                            "==Failed to send text msg to client ==1111: {:?}",
                                            e
                                        ),
                                        true,
                                    );
                                }
                            }
                        }
                    }
                }
            } else {
                // transfer processed data back to web client where client_id not equals to '11111111-1111-1111-1111-111111111111'
                // {"action":-1,"clientId":"11111111-1111-1111-1111-111111111111"}

                if let Some(action) = json_msg.get("action") {
                    // print!("==Start send back to WebBrowser");
                    log_message(&format!("==Start send back to WebBrowser"), true);

                    if let Some(action_int) = action.as_i64() {
                        // Handle action as an integer
                        if action_int != 13 {
                            let clients = clients.lock().unwrap();
                            for (id, client) in clients.iter() {
                                if *id != Uuid::parse_str("11111111-1111-1111-1111-111111111111")? {
                                    if let Err(e) = client.send(msg.clone()) {
                                        // println!("==Failed to send message to client !=1111: {:?}", e);
                                        log_message(
                                            &format!(
                                                "==Failed to send message to client !=1111: {:?}",
                                                e
                                            ),
                                            true,
                                        );
                                    }
                                }
                            }
                        }
                        //println!("==Action as integer: {}", action_int);
                    } else if let Some(_action_str) = action.as_str() {
                        // Handle action as a string
                        let clients = clients.lock().unwrap();
                        for (id, client) in clients.iter() {
                            if *id != Uuid::parse_str("11111111-1111-1111-1111-111111111111")? {
                                if let Err(e) = client.send(msg.clone()) {
                                    // println!("==Failed to send message to client !=1111: {:?}", e);
                                    log_message(
                                        &format!(
                                            "==Failed to send message to client !=1111: {:?}",
                                            e
                                        ),
                                        true,
                                    );
                                }
                            }
                        }
                    } else {
                        println!("==Action is neither an integer nor a string");
                    }
                }
            }
        }
    }

    Ok(())
}

async fn start_websocket_server() {
    let clients: Clients = Arc::new(Mutex::new(Vec::new()));
    tokio::spawn(async move {
        let ws_listener = TcpListener::bind(format!("0.0.0.0:{}", 9104))
            .await
            .unwrap();

        // println!("==WebSocket server listening on {:?}", ws_listener.local_addr());
        log_message(
            &format!(
                "==WebSocket server listening on {:?}",
                ws_listener.local_addr()
            ),
            true,
        );

        loop {
            match ws_listener.accept().await {
                Ok((socket, addr)) => {
                    /*
                     println!("");
                     println!("");
                     println!("==New client connected: {:?}", addr);
                     println!("==Socket details: {:?}", socket);
                    */

                    log_message(&format!("",), true);
                    log_message(&format!("",), true);
                    log_message(&format!("==New client connected: {:?}", addr), true);
                    log_message(&format!("==Socket details: {:?}", socket), true);

                    let config = WebSocketConfig {
                        max_message_size: Some(100 * 1024 * 1024), // 100 MB
                        max_frame_size: None,                      // No frame limit
                        // max_frame_size: Some(16 << 20), // 32 MiB
                        ..Default::default()
                    };

                    let clients = clients.clone();
                    tokio::spawn(async move {
                        if let Err(e) = handle_websocket(socket, clients, config.clone()).await {
                            // println!("==WebSocket error: {:?}", e);
                            log_message(&format!("==WebSocket error: {:?}", e), true);
                        }
                    });
                }
                Err(e) => {
                    // println!("==Failed to accept connection: {:?}", e);
                    log_message(&format!("==Failed to accept connection: {:?}", e), true);
                }
            }
        }
    });
}

fn log_message_to_file(message: &str) -> Result<(), Box<dyn Error>> {
    let now = Local::now();
    let file_name = format!(
        "log/log_{}_{}.txt",
        now.format("%Y-%m-%d"),
        now.format("%H")
    );

    // Check if the log directory exists, if not, create it
    let log_dir = std::path::Path::new("log");
    if !log_dir.exists() {
        std::fs::create_dir_all(log_dir)?;
    }

    let mut file = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_name)
    {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to open log file: {:?}", e);
            return Err(Box::new(e));
        }
    };
    if let Err(e) = writeln!(file, "{}", message) {
        println!("Failed to write to log file: {:?}", e);
        return Err(Box::new(e));
    }
    Ok(())
}

fn log_message(message: &str, log_to_file: bool) {
    let print_to_console = true;
    if log_to_file {
        if let Err(e) = log_message_to_file(message) {
            println!("Failed to log message: {:?}", e);
        }
    }
    if print_to_console {
        println!("{}", message);
    }
}
