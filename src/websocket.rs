use tungstenite::{connect, WebSocket};
use url::Url;

/// This function will connect to the depth websocket endpoing for the specified symbol
pub fn connect_to_websocket(symbol: &str) -> Result<WebSocket<impl std::io::Read + std::io::Write>, tungstenite::Error>  {
    let binance_url = format!("wss://stream.binance.com:9443/ws/{}@depth", symbol.to_lowercase());
    println!("binance ur {}", binance_url);
    let (mut socket, response) =connect(Url::parse(&binance_url).unwrap()).expect("Can't connect.");
    Ok(socket)
}
