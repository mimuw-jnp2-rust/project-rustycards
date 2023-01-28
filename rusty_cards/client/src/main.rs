extern crate game;
extern crate rusty_cards;
mod action;
mod p2p_client;
mod utils;

use std::io;

use p2p_client::Client;

fn main() -> io::Result<()> {
    println!("Hello angel :]");

    // Constructing a client (default or custom)
    let mut client = Client::default();
    let input = utils::provide_input(
        "Do you want to use default options (listed above)? If so, type: \'y\':",
    );
    if input.to_lowercase() != "y" {
        println!("You chose not to use default configurations");
        let server_socket = utils::provide_server();
        let listener_socket = utils::provide_listener();
        client = Client::new(server_socket, listener_socket);
    }

    // Looking for opponent or leaving a game
    println!("Client ready to look for an opponent ^_^");
    let mut exit_game: bool = false;
    while !exit_game {
        let input = utils::provide_input("Do you want to look for an opponent? If so type \'y\':");

        if input.to_lowercase() == "y" {
            client.start()?;
        } else {
            println!("Understood. Exiting the game. Have a great day n_n");
            exit_game = true;
        }
    }

    Ok(())
}