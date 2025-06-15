use core::str;
use std::{net::SocketAddr, process::Stdio};
use axum::{extract::{ws::{Message, WebSocket}, ConnectInfo, WebSocketUpgrade}, response::{Html, IntoResponse}, Json};
use serde_json::{json, Value};
use tokio::{io::{AsyncBufReadExt, BufReader}, process::Command};
pub async fn index() -> Html<&'static str> {
    Html(std::include_str!("../index.html"))
}

pub async fn previous() -> Json<Value> {
    Command::new("playerctl").args(&["previous"]).spawn().unwrap().wait().await.unwrap();
    Json(json!({"message": "ok"}))
}

pub async fn play_pause() -> Json<Value> {
    Command::new("playerctl").args(&["play-pause"]).spawn().unwrap().wait().await.unwrap();
    Json(json!({"message": "ok"}))
}

pub async fn next() -> Json<Value> {
    Command::new("playerctl").args(&["next"]).spawn().unwrap().wait().await.unwrap();
    Json(json!({"message": "ok"}))
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

pub async fn handle_socket(mut socket: WebSocket, _addr: SocketAddr) {
    let mut status_child = Command::new("playerctl").args(&["-F", "status"]).stdout(Stdio::piped()).spawn().unwrap();
    let status_stdout = status_child.stdout.take().unwrap();
    let mut status_reader = BufReader::new(status_stdout);
    let mut status_line = String::new();

    let mut metadata_child = Command::new("playerctl").args(&["-F", "metadata", "--format", "{{ mpris:artUrl }}\r{{ artist }}\r{{ title }}\r{{ mpris:length }}"]).stdout(Stdio::piped()).spawn().unwrap();
    let metadata_stdout = metadata_child.stdout.take().unwrap();
    let mut metadata_reader = BufReader::new(metadata_stdout);
    let mut metadata_line = String::new();

    let mut position_child = Command::new("playerctl").args(&["-F", "position"]).stdout(Stdio::piped()).spawn().unwrap();
    let position_stdout = position_child.stdout.take().unwrap();
    let mut position_reader = BufReader::new(position_stdout);
    let mut position_line = String::new();
    
    loop {
        tokio::select! {
            Ok(size) = status_reader.read_line(&mut status_line) => {
                if size == 0 {
                    break;
                } 

                let json = json!({"type": "status", "status": status_line.trim() == "Playing"});
                if let Err(_) = socket.send(Message::Text(json.to_string().into())).await {
                    break;
                }
            }
            Ok(size) = metadata_reader.read_line(&mut metadata_line) => {
                if size == 0 {
                    break;
                }

                let mut split = metadata_line.split("\r");
                let art_url = split.next().unwrap();
                let artist = split.next().unwrap();
                let track  = split.next().unwrap();
                let length = split.next().unwrap().trim().parse::<i32>().unwrap() / 10^6; 

                let json = json!({"type": "metadata", "artUrl": art_url, "artist": artist, "track": track, "length": length});
                if let Err(_) = socket.send(Message::Text(json.to_string().into())).await {
                    break;
                }
            }
            Ok(size) = position_reader.read_line(&mut position_line) => {
                if size == 0 {
                    break;
                } 

                let position = position_line.trim().parse::<f32>().unwrap() as i32;

                let json = json!({"type": "position", "position": position});
                if let Err(_) = socket.send(Message::Text(json.to_string().into())).await {
                    break;
                }
            }
            
        }

        status_line.clear();
        metadata_line.clear();
        position_line.clear();
    }
}