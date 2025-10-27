//importing the various moudles from the cursive library for UI dev.
use cursive::{
    Cursive, align::HAlign, event::Key, theme::{BaseColor, BorderStyle, Color, Palette, PaletteColor, Theme}, traits::*, views::{Dialog, DummyView, EditView, LinearLayout, Panel, ScrollView, TextView}
};

use serde::{Serialize, Deserialize};

use chrono::Local;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpStream, sync::Mutex
};

use std::{env, error::Error, sync::Arc};

#[derive(Debug, Clone, Serialize, Deserialize)]
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



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //fetching the name from the command line arguments.
    let username = env::args()
    .nth(1)
    .expect("please provide a username as argumant");

    //initialize the cursive object [the cursive Ui framework]
    let mut siv = cursive::default();
    siv.set_theme(create_retro_theme());

    //create a header to display the chat title and username
    let header = TextView::new(format!("Retro Chat | User: {} | Date: {}",
    username,
    Local::now().format("%H:%M:%S")
    ))
    .style(Color::Light(BaseColor::Green))
    .h_align(HAlign::Center);

    //Creating the message area with a scrollable text view.
    let messages = TextView::new("")
    .with_name("messages")
    .min_height(20);
 
    let messages = ScrollView::new(messages)
    .scroll_strategy(cursive::view::ScrollStrategy::StickToBottom)
    .min_width(60)
    .full_width();

    //Creating an input area for typing messages
    let input = EditView::new()
    .on_submit(move |s, text| send_messages(s, text.to_string()))
    .with_name("input")
    .max_height(3)
    .min_width(50)
    .full_width();

    //Create help text for user commands.
    let help_text = TextView::new("ESC:quit | Enter:send | Commands:/help, /clear, /quit")
        .style(Color::Dark(BaseColor::White));

    //Assembling the main layout 
    let layout = LinearLayout::vertical()
            .child(Panel::new(header))
            .child(
                Dialog::around(messages)
                        .title("Messages")
                        .title_position(HAlign::Center)
                        .full_width()
            )
            .child(
                Dialog::around(input)
                        .title("Message")
                        .title_position(HAlign::Center)
                        .full_width()
            )
            .child(Panel::new(help_text).full_width());
        
    let centered_layout = LinearLayout::horizontal()
            .child(DummyView.full_width())
            .child(layout)
            .child(DummyView.full_width());

    //adding the centered layout to the cursive root.
    siv.add_fullscreen_layer(centered_layout);
    //adding global key binfings
    siv.add_global_callback(Key::Esc, |s|s.quit()); //ESC to quite
    siv.add_global_callback('/', |s| {s.call_on_name("input", |view: &mut EditView| {
        view.set_content("/"); //inserting the "/" in the input box
    });
});

    //Establishing a connection with the server
    //-----------------------------------------
    let stream = TcpStream::connect("127.0.0.1:8082").await?;
    let (reader, mut writer) = stream.into_split();
    writer.write_all(format!("{}\n", username).as_bytes()).await?;

    let writer = Arc::new(Mutex::new(writer));
    let writer_clone = Arc::clone(&writer);
    siv.set_user_data(writer);

    let reader = BufReader::new(reader);
    let mut lines = reader.lines();
    let sink = siv.cb_sink().clone();

    //spawn async task to handle incoming messages.
    tokio::spawn(async move{
        while let Ok(Some(line)) = lines.next_line().await{
            if let Ok(msg) = serde_json::from_str::<ChatMessage>(&line){
                let formatted_msg = match msg.message_type {
                    MessageType::UserMessage => format!("{}\n{} | {}\n",
                        msg.timestamp, msg.username, msg.content),
                    MessageType::SystemNotification => format!("\n{} {}\n",
                        msg.username, msg.content),
                };

                if sink.send(Box::new(move |siv:&mut Cursive|{
                    siv.call_on_name("messages", |view:&mut TextView|{
                        view.append(formatted_msg);
                    });
                })).is_err(){
                    break;
                }
            }
        };
    });
    //Run the cursive event loop.
    siv.run();
    let _ = writer_clone.lock().await.shutdown().await; //closing the writer.
    Ok(())
}


fn send_messages(siv: &mut Cursive, msg: String){
    if msg.is_empty(){
        return;
    }

     match msg.as_str(){
        "/help" => {
            siv.call_on_name("message", |view: &mut TextView|{
                view.append("\n=== Commands ===\n/help - show this help\n/clear - Clear messages\n/quit - Exit chat\n\n");
            });
            siv.call_on_name("input", |view: &mut EditView|{
                view.set_content("");
            });
            return;
        }
        "/clear" => {
            siv.call_on_name("messages", |view:&mut TextView|{
                view.set_content("");
            });
            siv.call_on_name("input", |view: &mut EditView|{
                view.set_content("");
            });
            return;
        }
        "/quit" => {
            siv.quit();
            return;
        }
        _ => {}
    }

    let writer = siv.user_data::<Arc<Mutex<tokio::net::tcp::OwnedWriteHalf>>>().unwrap().clone();
    tokio::spawn(   async move {
        let _ = writer.lock().await.write_all(format!("{}\n", msg).as_bytes()).await;
    });

}




//create the retro style theme function 
fn create_retro_theme() -> Theme {
    let mut theme = Theme::default();
    theme.shadow = true;
    theme.borders = BorderStyle::Simple;

    let mut palette = Palette::default();
    palette[PaletteColor::Background] = Color::Rgb(0,0,20);
    palette[PaletteColor::View] = Color::Rgb(0,0,20);
    palette[PaletteColor::Primary] = Color::Rgb(0,255,0);
    palette[PaletteColor::TitlePrimary] = Color::Rgb(0,255,128);
    palette[PaletteColor::Secondary] = Color::Rgb(255,191,0);
    palette[PaletteColor::Highlight] = Color::Rgb(0,255,255);
    palette[PaletteColor::HighlightInactive] = Color::Rgb(0,128,128);
    palette[PaletteColor::Shadow] = Color::Rgb(0,0,40);
    theme.palette = palette;
    theme
}