use case::parse_config;

#[test]
fn reads_the_port() {
    for (text, port) in [("[server]\nport = 8080\n", 8080), ("[server]\nport = 1\n", 1)] {
        assert_eq!(parse_config(text).unwrap().port, port);
    }
}
