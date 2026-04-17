pub mod client;
pub mod models;
pub mod server;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("USAGE: ./myteams_server port");
        std::process::exit(84);
    }

    let port: u16 = match args[1].parse() {
        Ok(p) => p,
        Err(_) => {
            println!("Invalid port number.");
            std::process::exit(84);
        }
    };

    my_teams::ffi::setup_signal_handler();

    match server::Server::new(port) {
        Ok(mut srv) => srv.run(),
        Err(e) => {
            println!("Error initialising server: {e}");
            std::process::exit(84);
        }
    }
}
