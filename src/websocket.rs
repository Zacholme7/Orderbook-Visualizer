use tungstenite::connect;
use url::Url;

pub fn connect_to_websocket(symbol: &str) {
    // format the url with the symbol
    let binance_url = format!("wss://stream.binance.com:9443/ws/{}@depth", symbol);

    // connect to the socket
    let (mut socket, response) =connect(Url::parse(&binance_url).unwrap()).expect("Can't connect.");

    // parse the response and print out the headers
    println!("Connected to binance stream.");
    println!("HTTP status code: {}", response.status());
    println!("Response headers:");
    for (ref header, ref header_value) in response.headers() {
        println!("- {}: {:?}", header, header_value);
    }

    // main logic loop
    loop {
        // receive the message
        println!("hello");
        let msg = socket.read_message().expect("Error reading message");

        // error checking the message
        let msg = match msg {
            tungstenite::Message::Text(s) => s,
            _ => {
                panic!("Error getting text");
            }
        };
        println!("{}", msg);
    }
}