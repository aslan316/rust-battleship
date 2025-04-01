use std::io;

use rust_battleship::{BattleShip, client, server};

fn main() {
    let mut input = String::new();
    let mut player1 = BattleShip::new();
    let mut player2 = BattleShip::new();
    loop {
        input.clear();
        println!("Act as server for this game? [y/n]");
        io::stdin().read_line(&mut input).expect("Bad input");
        input = input.trim_end().to_string();
        if input == "y" || input == "n" {
            break;
        } else {
            println!("Improper input.");
        }
    }

    if input == "y" {
        println!("On which port do you want to host?");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim_end().to_string();
        let ip = local_ip_address::local_ip().unwrap();
        println!("The server IP is {ip}");
        input = format!("{ip}:{input}");
        server::initialize_server(&input, &mut player1, &mut player2);
    } else if input == "n" {
        println!("Enter IP4 of host and port (ip:port):");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim_end().to_string();
        client::initialize_client(&input);
    } else {
        println!("somethings going wrong");
    }
}
