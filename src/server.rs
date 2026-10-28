pub fn serve() {
    let _server = tiny_http::Server::http("0.0.0.0:0").unwrap();
}