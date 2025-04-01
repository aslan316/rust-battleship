use std::{
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use crate::{BattleShip, Coordinate, letter_to_num};

pub fn initialize_server(address: &str, player1: &mut BattleShip, player2: &mut BattleShip) {
    let listener = TcpListener::bind(address).expect("Failed to bind");

    match listener.accept() {
        Ok((stream, _)) => {
            println!("Game initiated");
            handle_server(stream, player1, player2);
        }
        Err(_) => panic!(),
    }
}

fn handle_server(stream: TcpStream, player1: &mut BattleShip, player2: &mut BattleShip) {
    let mut reader = BufReader::new(stream.try_clone().expect("Failed to clone stream"));
    let mut writer = stream.try_clone().expect("Failed to clone stream");

    while player1.get_ships_left() > 0 {
        print!("{}", player1.print_ship_board());
        println!("Enter the row-column coordinate pair of where to place the ship (start - end)");

        let mut coordinates = String::new();
        io::stdin()
            .read_line(&mut coordinates)
            .expect("Failed to read line");
        let ascii_uppercase = coordinates.to_ascii_uppercase();
        let coordinates: Vec<&str> = ascii_uppercase
            .trim()
            .split(&['-', ',', ':'][..])
            .map(|coord| coord.trim())
            .collect();
        let coordinates = Coordinate::new(coordinates[0], coordinates[1]);

        if player1.can_place_ship(&coordinates) {
            player1.place_ship(&coordinates);
        } else {
            println!(
                "Invalid coordinate entered (check if you have that size ship or if something is blocking your way)"
            );
        }
    }
    println!("You've placed all of your ships!");
    println!();
    print!("{}", player1.print_ship_board());

    println!("The other player is placing their ships...");

    writeln!(writer, "{}", player2.get_ships_left()).unwrap();
    while player2.get_ships_left() > 0 {
        write!(writer, "{}", player2.print_ship_board()).unwrap();
        writeln!(
            writer,
            "Enter the row-column coordinate pair of where to place the ship (start - end)"
        )
        .unwrap();

        let mut coordinates = String::new();
        reader
            .read_line(&mut coordinates)
            .expect("Failed to read line");
        let ascii_uppercase = coordinates.to_ascii_uppercase();
        let coordinates: Vec<&str> = ascii_uppercase
            .trim()
            .split(&['-', ',', ':'][..])
            .map(|coord| coord.trim())
            .collect();
        let coordinates = Coordinate::new(coordinates[0], coordinates[1]);
        if player2.can_place_ship(&coordinates) {
            player2.place_ship(&coordinates);
        } else {
            writeln!(
                writer,
                "Invalid coordinate entered (check if you have that size ship or if something is blocking your way)"
            ).unwrap();
        }

        writeln!(writer, "{}", player2.get_ships_left()).unwrap();
    }
    writeln!(writer, "You've placed all of your ships").unwrap();
    writeln!(writer).unwrap();
    write!(writer, "{}", player2.print_ship_board()).unwrap();

    println!();
    writeln!(writer).unwrap();

    let mut is_not_winner = !player1.check_if_winner() || !player2.check_if_winner();
    writeln!(writer, "{is_not_winner}").unwrap();
    while is_not_winner {
        print!("{}", player1.print_boards());
        println!("Enter the row-colun coordinate of where you want to guess");
        let mut coordinate = String::new();
        io::stdin().read_line(&mut coordinate).expect("Bad input");
        let coordinate = coordinate.to_ascii_uppercase();
        let coordinate = coordinate.trim();
        let coordinate = Coordinate {
            row_start: letter_to_num(&coordinate[0..1]).unwrap(),
            row_end: letter_to_num(&coordinate[0..1]).unwrap(),
            column_start: coordinate[1..].parse().unwrap(),
            column_end: coordinate[1..].parse().unwrap(),
        };
        player1.guess(player2, &coordinate);
        print!("{}", player1.print_boards());
        println!();

        write!(writer, "{}", player2.print_boards()).unwrap();
        writeln!(
            writer,
            "Enter the row-colun coordinate of where you want to guess"
        )
        .unwrap();
        let mut coordinate = String::new();
        reader.read_line(&mut coordinate).expect("Bad input");
        let coordinate = coordinate.to_ascii_uppercase();
        let coordinate = coordinate.trim();
        let coordinate = Coordinate {
            row_start: letter_to_num(&coordinate[0..1]).unwrap(),
            row_end: letter_to_num(&coordinate[0..1]).unwrap(),
            column_start: coordinate[1..].parse().unwrap(),
            column_end: coordinate[1..].parse().unwrap(),
        };
        player2.guess(player1, &coordinate);
        write!(writer, "{}", player2.print_boards()).unwrap();
        writeln!(writer).unwrap();

        is_not_winner = !player1.check_if_winner() || !player2.check_if_winner();
        writeln!(writer, "{is_not_winner}").unwrap();
    }

    if player1.check_if_winner() {
        println!("You have won!");
        writeln!(writer, "The other player has won... better luck next time!").unwrap();
    } else {
        println!("The other player has won... better luck next time!");
        writeln!(writer, "You have won!").unwrap();
    }
    println!("Good game!");
    writeln!(writer, "Good game!").unwrap();
}
