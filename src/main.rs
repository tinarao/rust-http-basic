mod server;
use server::Server;

fn main() {
    let server = Server::default("3000");
    server.run();
}
