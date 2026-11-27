use case::parse_config;

fn config_text(port: u16, host: &str) -> String {
    let mut text = String::new();
    text.push_str("[server]\n");
    text.push_str(&format!("port = {port}\n"));
    text.push_str(&format!("host = \"{host}\"\n"));
    text
}

#[test]
fn reads_the_port() {
    let config = parse_config(&config_text(8080, "localhost")).unwrap();
    assert_eq!(config.port, 8080);
}
