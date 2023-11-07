use tungstenite::{connect, WebSocket};
use url::Url;

/// This function will connect to the depth websocket endpoing for the specified symbol
pub fn connect_to_websocket(symbol: &str) -> Result<WebSocket<impl std::io::Read + std::io::Write>, tungstenite::Error>  {
    let binance_url = "wss://stream.binance.com:9443/ws/bnbbtc@depth";
    let (mut socket, response) =connect(Url::parse(&binance_url).unwrap()).expect("Can't connect.");

    println!("Connected to binance stream.");
    println!("Response headers:");
    for (ref header, ref header_value) in response.headers() {
        println!("- {}: {:?}", header, header_value);
    }

    Ok(socket)
}
