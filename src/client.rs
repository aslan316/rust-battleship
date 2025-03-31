use std::{
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

pub fn initialize_client(address: &str) {
    let listener = TcpListener::bind(address).expect("Failed to bind");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Game initiated");
                handle_client(stream);
            }
            Err(_) => panic!(),
        }
    }
}

fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().expect("Failed to clone stream"));
    let mut writer = stream.try_clone().expect("Failed to clone stream");

    println!("The other player is placing their ships...");

    let mut ships_left = String::new();
    let mut ships_left = reader.read_line(&mut ships_left).unwrap() as i32;
    while ships_left > 0 {
        accept_boards(&mut reader);
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        println!("{line}");

        line.clear();
        reader.read_line(&mut line).unwrap();
        writeln!(writer, "{line}").unwrap();

        line.clear();
        reader.read_line(&mut line).unwrap();
        match line.parse::<i32>() {
            Ok(num) => ships_left = num,
            // in the case that the next line is actually the error message
            Err(_) => {
                println!("{line}");
                line.clear();
                reader.read_line(&mut line).unwrap();
                ships_left = line.parse().unwrap();
            }
        }
    }

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    println!("{line}");
    line.clear();
    reader.read_line(&mut line).unwrap();
    println!("{line}");
    accept_boards(&mut reader);

    line.clear();
    reader.read_line(&mut line).unwrap();
    println!("{line}");

    // reading in if there is a winner
    // in java this was a static variable on the class but that is
    // not available here
    line.clear();
    reader.read_line(&mut line).unwrap();
    let mut is_not_winner = line.parse::<bool>().unwrap();
    while is_not_winner {
        accept_boards(&mut reader);
        line.clear();
        reader.read_line(&mut line).unwrap();
        print!("{line}");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Bad input");
        writeln!(writer, "{input}").unwrap();

        accept_boards(&mut reader);
        line.clear();
        reader.read_line(&mut line).unwrap();
        println!("{line}");

        line.clear();
        reader.read_line(&mut line).unwrap();
        is_not_winner = line.parse().unwrap();
    }

    line.clear();
    reader.read_line(&mut line).unwrap();
    println!("{line}");
    line.clear();
    reader.read_line(&mut line).unwrap();
    println!("{line}");
}

fn accept_boards(reader: &mut BufReader<TcpStream>) {
    for _ in 0..12 {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        println!("{line}");
    }
}
