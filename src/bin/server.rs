use cursive::views::ListChild;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
};
use serde::{Deserialize, Serialize, de::Error};
use chrono::Local;
use std::error:Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
///this is an attribute instructing the compiler to automatically generate implemations for the 4 traits.
struct ChatMessage{
    username: String,
    content: String,
    timestamp: String,
    message_type: MessageType
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum MessageType{
    UserMessage,
    SystemNotification,
}

//what is Tokio?
//Asynchronous runtime for the rust language.







#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8082").await?;
    
    //Create a broadcast channel for message distribution
    let (tx, _) = broadcast::channel::<String>(100);
    
    loop{
        let (socket, addr) = listener.accept().await?;
    }
}