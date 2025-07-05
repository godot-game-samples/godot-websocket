use tungstenite::{connect, Message};
use url::Url;

fn main() {
    let url = Url::parse("ws://127.0.0.1:9080").unwrap();
    let (mut socket, response) = connect(url).expect("接続に失敗");

    println!("サーバーに接続しました: {:?}", response);

    socket.write_message(Message::Text("Hello from Rust!".into())).unwrap();

    if let Ok(msg) = socket.read_message() {
        println!("受信: {}", msg);
    }
}