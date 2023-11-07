use tungstenite::{connect, WebSocket, Message, Error};
use tungstenite::protocol::WebSocketConfig;
use url::Url;

pub fn connect_to_websocket(symbol: &str) -> Result<WebSocket<impl std::io::Read + std::io::Write>, tungstenite::Error>  {
    // format the url with the symbol
    //let binance_url = format!("wss://stream.binance.com:9443/ws/{}@depth", symbol);
    let binance_url = "wss://stream.binance.com:9443/ws/bnbbtc@depth";

    // connect to the socket
    let (mut socket, response) =connect(Url::parse(&binance_url).unwrap()).expect("Can't connect.");

    // parse the response and print out the headers
    println!("Connected to binance stream.");
    println!("HTTP status code: {}", response.status());
    println!("Response headers:");
    for (ref header, ref header_value) in response.headers() {
        println!("- {}: {:?}", header, header_value);
    }

    Ok(socket)
}