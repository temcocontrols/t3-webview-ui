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
use tokio_tungstenite::accept_async;
// use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{SinkExt, StreamExt};
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::protocol::Message;

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

// async fn start_websocket_server(){
//   tokio::spawn(async move {
//     let ws_listener = TcpListener::bind(format!("0.0.0.0:{}", 9104)).await.unwrap();
//     // println!("WebSocket server listening on {:?}", ws_listener.local_addr());

//     loop {
//       let (socket, _) = ws_listener.accept().await.unwrap();
//       tokio::spawn(async move {
//         if let Err(e) = handle_websocket(socket).await {
//           println!("WebSocket error: {:?}", e);
//         }
//       });
//     }
//   });

//   /*
//   // Create a WebSocket server
//   tokio::spawn(async move {
//     let ws_listener = TcpListener::bind(format!("0.0.0.0:{}", 9104)).await.unwrap();
//     println!("WebSocket server listening on {:?}", ws_listener.local_addr());

//     loop {
//       let (socket, _) = ws_listener.accept().await.unwrap();
//       tokio::spawn(async move {
//         if let Err(e) = handle_websocket(socket).await {
//           println!("WebSocket error: {:?}", e);
//         }
//       });
//     }
//   });
//   */
// }

// async fn handle_websocket(stream: TcpStream) -> Result<(), Box<dyn Error>> {
//   let ws_stream = accept_async(stream).await?;
//   let (mut write, mut read) = ws_stream.split();

//   while let Some(msg) = read.next().await {
//     let msg = msg?;

//     if msg.is_text() || msg.is_binary() {
//       println!("Received message: {}", msg);
//         // Process the message and forward it to clients and T3000
//       write.send(msg).await?;
//     }

//     /*
//     if msg.is_text() {
//       println!("Received text message: {}", msg.to_text()?);
//       write.send(msg).await?;
//     } else if msg.is_binary() {
//       let data = msg.into_data();
//       let json: serde_json::Value = serde_json::from_slice(&data)?;
//       println!("Received binary message as JSON: {}", json);
//       write.send(msg).await?;
//     }
//     */
//   }

//   Ok(())
// }

type Clients = Arc<Mutex<Vec<(Uuid, tokio::sync::mpsc::UnboundedSender<Message>)>>>;

async fn handle_websocket(stream: TcpStream, clients: Clients) -> Result<(), Box<dyn Error>> {
    println!("==Start handling websocket");
    println!("==Stream: {:?}", stream);
    println!("==Clients 1st: {:?}", clients);
    println!("");

    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            println!("==Failed to accept websocket connection: {:?}", e);
            return Err(Box::new(e));
        }
    };

    //  println!("==ws_stream: {:?}", ws_stream);

    let (mut write, mut read) = ws_stream.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    // let client_id = Uuid::new_v4();
    // println!("==Assigned client ID: {}", client_id);

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Err(e) = write.send(msg).await {
                println!("==Error sending message to client: {:?}", e);
                break;
            }
        }
    });

    // bind the client id to incoming session
    while let Some(msg) = read.next().await {
        let msg = msg?;

        println!("==Recived message 1st:{:?}", msg);

        if msg.is_text() || msg.is_binary() {
            let msg_text = msg.to_text()?;
            let json_msg: serde_json::Value = serde_json::from_str(msg_text)?;

            //{"header":{"clientId":"-","from":"Firefox"},"message":{"action":-1,"clientId":"4aa7e8c2-437e-422c-a55d-e1ae4c757935"}}
            //{"header":{"clientId":"-","from":"Firefox"},"message":{"action":-1,"clientId":"11111111-1111-1111-1111-111111111111"}}
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
                                    println!("==Failed to send text msg to client ==1111: {:?}", e);
                                }
                            } else {
                                // println!("==Clients 2nd: {:?}", clients);
                                // println!("==Cannot find client 1111");
                            }
                        }
                    }
                }
            } else {
                // transfer processed data back to web client where client_id not equals to '11111111-1111-1111-1111-111111111111'
                // {"action":-1,"clientId":"11111111-1111-1111-1111-111111111111"}

                if let Some(_action) = json_msg.get("action").and_then(|a| a.as_i64()) {
                    if _action != 13 {
                        let clients = clients.lock().unwrap();
                        for (id, client) in clients.iter() {
                            if *id != Uuid::parse_str("11111111-1111-1111-1111-111111111111")? {
                                if let Err(e) = client.send(msg.clone()) {
                                    println!("==Failed to send message to client !=1111: {:?}", e);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // clients.lock().unwrap().push((client_id, tx));

    /*
    clients.lock().unwrap().push(tx);

    tokio::spawn(async move {
      while let Some(msg) = rx.recv().await {
        if let Err(e) = write.send(msg).await {
          println!("==Error sending message to client: {:?}", e);
          break;
        }
      }
    });

    while let Some(msg) = read.next().await {
      let msg = msg?;
      println!("------------------------");
      println!("==Received message: {}", msg);

      if msg.is_text() || msg.is_binary() {
        println!("==Server Received message from client: {}", msg);

        let clients = clients.lock().unwrap();
        for client in clients.iter() {
          println!("==Sending message to client: {:?}", client);
          if let Err(e) = client.send(msg.clone()) {
            println!("==Failed to send message to client: {:?}", e);
          }
        }
      }
    }
    */

    Ok(())
}

async fn start_websocket_server() {
    let clients: Clients = Arc::new(Mutex::new(Vec::new()));
    tokio::spawn(async move {
        let ws_listener = TcpListener::bind(format!("0.0.0.0:{}", 9104))
            .await
            .unwrap();
        println!(
            "==WebSocket server listening on {:?}",
            ws_listener.local_addr()
        );
        loop {
            match ws_listener.accept().await {
                Ok((socket, addr)) => {
                    println!("");
                    println!("");
                    println!("==New client connected: {:?}", addr);
                    println!("==Socket details: {:?}", socket);
                    let clients = clients.clone();
                    tokio::spawn(async move {
                        if let Err(e) = handle_websocket(socket, clients).await {
                            println!("==WebSocket error: {:?}", e);
                        }
                    });
                }
                Err(e) => {
                    println!("==Failed to accept connection: {:?}", e);
                }
            }
        }
    });
}
