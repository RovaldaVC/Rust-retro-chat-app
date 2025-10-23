//importing the various moudles from the cursive library for UI dev.
use cursive::{
    Cursive, align::HAlign, event::Key, style::Rgb, theme::{BaseColor, BorderStyle, Color, Palette, PaletteColor, Theme}, traits::*, views::{Dialog, DummyView, EditView, LinearLayout, Panel, ScrollView, TextView}
};

use serde::{Serialize, Deserialize};

use chrono::Local;

use tokio::{
    net::{TcpStream},
    sync::Mutex,
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
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