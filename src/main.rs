use tungstenite::connect;
use url::Url;

fn main() {
    get_depth_snapshot();
    connect_to_websocket();
}

fn get_depth_snapshot() {
    let snapshot_url = "https://api.binance.com/api/v3/depth";
    let params = [("symbol", "BNBBTC")];

    let client = reqwest::blocking::Client::new();

    let res = client.get(snapshot_url)
        .query(&params)
        .send()
        .unwrap();

    match res.text() {
        Ok(text) => println!("Response text: {}", text),
        Err(e) => eprintln!("Error getting text : {}", e)
    }
}

fn connect_to_websocket() {
    let binance_url = "wss://stream.binance.com:9443/ws/bnbbtc@depth";
    let (mut socket, response) =
        connect(Url::parse(&binance_url).unwrap()).expect("Can't connect.");

    println!("Connected to binance stream.");
    println!("HTTP status code: {}", response.status());
    println!("Response headers:");
    for (ref header, ref header_value) in response.headers() {
        println!("- {}: {:?}", header, header_value);
    }

    loop {
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => s,
            _ => {
                panic!("Error getting text");
            }
        };
        println!("{}", msg);
    }
}