use tiny_http::{Server, Response};

pub fn serve() -> Result<(), std::io::Error> {
    let server = Server::http("0.0.0.0:100").unwrap();

    for request in server.incoming_requests() {
        println!("received request! method: {:?}, url: {:?}, headers: {:?}",
            request.method(),
            request.url(),
            request.headers()
        );

        let response = Response::from_string("hello world");
        request.respond(response)?;
    };
    Ok(())
}