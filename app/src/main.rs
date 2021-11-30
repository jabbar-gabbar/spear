fn main() {
    let config = AppConfig {
        source: String::from("source"),
        destination: String::from("destination"),
    };
    println!(
        "source {}, destination {}",
        config.source, config.destination
    );
}

struct AppConfig {
    source: String,
    destination: String,
}
