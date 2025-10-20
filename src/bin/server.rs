use cursive::views::ListChild;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
};
use serde::{Deserialize, Serialize};
use std::{error::Error, io::BufWriter};
use chrono::Local;

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
        //accepting the new connection
        let (socket, addr) = listener.accept().await?;

        //display the connection onformation
        //Local is for time: (Hour, Minute, Second)
        println!("new connection {}", Local::now().format("%H:%M:%S"));
        println!("Address: {}", addr);

        //clone sender for this connection and subscribe a receiver
        let tx = tx.clone();
        let rx = tx.subscribe();
    }
    //Fnction to handle the client connection
    async fn handle_connection(
        mut socket: TcpStream,
        tx: broadcast::Sender<String>,
        mut rx: broadcast::Receiver<String>
    ){
        //splitting the socket into reader and writer
        let (reader, mut writer) = socket.split();
        let mut reader = BufReader::new(reader);

        //read the username sent by client.
        let mut username =  String::new();
        reader.read_line(&mut username).await.unwrap();
        let username = username.trim().to_string();

        //send a system notification "user joined the chat."
        let join_message = ChatMessage{
            username: username.clone(),
            content: "joined the chat.".to_string(),
            timestamp: Local::now().format("%H:&M:%S").to_string(),
            message_type: MessageType::SystemNotification,
        };
        let join_json = serde_json::to_string(&join_message).unwrap();
        tx.send(join_json).unwrap();
        
        //a buffer for incoming messages from the client.
        let mut line = String::new();
        loop {
            tokio::select! {
                result = reader.read_line(&mut line) =>{
                    if result.unwrap() == 0 {
                        break;
                    }
                    //create and broadcast user message
                    let message = ChatMessage{
                        username: username.clone(),
                        content: line.trim().to_string(),
                        timestamp: Local::now().format("%H:%M:%S").to_string(),
                        message_type: MessageType::UserMessage,
                    };
                    let json = serde_json::to_string(&message).unwrap();
                    tx.send(json).unwrap();
                    line.clear();
                }
                //Handle the incoming broadcast and send them to the client.
                result = rx.recv() => {
                    let message = result.unwrap();
                    writer.write_all(message.as_bytes()).await.unwrap();
                }
            }
        }
    }
}

// ctrl + shift + a opens a comment place for you. /*  */