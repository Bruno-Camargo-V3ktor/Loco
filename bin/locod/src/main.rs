use std::io;

use loco_server::Server;

fn main() -> io::Result<()> {
    let server = Server::new("0.0.0.0", 8080);
    server.run()?;

    Ok(())
}
