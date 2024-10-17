use std::{io::Result, net::SocketAddr, str::FromStr, sync::Arc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokens_store::TokensStore;
use warp::{filters::ws::{Message, WebSocket, Ws}, Filter};
use dotenvy::dotenv;
use media_server::MediaServer;
use futures_util::{SinkExt, StreamExt};

mod media_server;
mod mediaserver_room;
mod tokens_store;

struct UserData {
    room_id: String
}

#[derive(Debug,Serialize, Deserialize)]
struct WsMessage {
    r#type: String,
    text: String
}

async fn handle_ws_connection(token: String, mut ws_stream: WebSocket, mediaserver: Arc<MediaServer>, tokens_store: Arc<TokensStore<UserData>>) {
    
    if let Some(user_data) = tokens_store.use_token(&token).await {
        let room = mediaserver.get_room(user_data.room_id.clone()).await;
        let (mut tx, mut rx) = ws_stream.split();

        let mut user = room.add_user().await;
        tokio::spawn(async move {
            while let Some(msg) = user.recv().await {
                let text_message = WsMessage { r#type: "message".to_string(), text: msg };  
                let _ = tx.send(Message::text(serde_json::json!(text_message).to_string())).await;
            }
        });

        while let Some(Ok(msg)) = rx.next().await {
            if msg.is_close() { break; }
            let msg: WsMessage = serde_json::from_slice(msg.as_bytes()).unwrap();
            if msg.r#type == "message" {
                room.send_text_message(msg.text).await
            }
        }
        tokens_store.release_token(&token).await;
    } else {
        let _ = ws_stream.send(Message::text(json!({ "error": "wrong auth token" }).to_string())).await;
        let _ = ws_stream.close().await;
    }    
}

#[derive(Serialize, Deserialize)]
struct WsQuery {
    token: String
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv();

    let mediaserver = Arc::new(MediaServer::new());
    let tokens_store = Arc::new(TokensStore::new());

    let tokens_store2 = tokens_store.clone();
    let ws_route = warp::path!("ws")
        .and(warp::ws())
        .and(warp::query::<WsQuery>())
        .and(warp::any().map(move || mediaserver.clone()))
        .and(warp::any().map(move || tokens_store2.clone()))
        .map(|ws: Ws, query: WsQuery, server: Arc<MediaServer>, tokens_store: Arc<TokensStore<UserData>> | {
            ws.on_upgrade(move | websocket: WebSocket | {
                handle_ws_connection(query.token, websocket, server, tokens_store)
            })
        });

    let register_user_route = warp::path!("rooms" / String / "register-user")
        .and(warp::any().map(move || tokens_store.clone()))
        .and_then(|room_id: String, tokens_store: Arc<TokensStore<UserData>> | async move {
            let token = tokens_store.register_token(UserData { room_id: room_id.clone() }).await;
            let result = serde_json::json!({ "token": token }).to_string();
            Ok::<_, warp::Rejection>(result)
        })
        .with(warp::reply::with::header("content-type", "application/json"));

    let routes = warp::any().and(
        ws_route
        .or(register_user_route)
    );

    let blue = console::Style::new().blue();
    let addr = SocketAddr::from_str(&std::env::var("HOST").unwrap_or("127.0.0.1:3000".to_string()).to_owned()).unwrap();
    println!("\nRust Warp Server ready at {}", blue.apply_to(format!("http://{}", addr)));
  
    warp::serve(routes)
      .run(addr)
      .await;  

    Ok(())
}