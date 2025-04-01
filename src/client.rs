use std::{
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
};

pub fn initialize_client(address: &str) {
    let stream = TcpStream::connect(address);

    match stream {
        Ok(stream) => {
            println!("Game initiated");
            handle_client(stream);
        }
        Err(_) => panic!("Failed to connect"),
    }
}

fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().expect("Failed to clone stream"));
    let mut writer = stream.try_clone().expect("Failed to clone stream");

    println!("The other player is placing their ships...");

    let mut ships_left = String::new();
    reader.read_line(&mut ships_left).unwrap();
    let ships_left = ships_left.trim();
    let mut ships_left = ships_left.parse().unwrap();
    while ships_left > 0 {
        accept_boards(&mut reader);
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        print!("{line}");

        loop {
            line.clear();
            io::stdin().read_line(&mut line).unwrap();
            line = line.trim().to_string();
            if !line.is_empty() {
                break;
            }
        }
        writeln!(writer, "{line}").unwrap();

        line.clear();
        reader.read_line(&mut line).unwrap();
        line = line.trim().to_string();
        match line.parse::<i32>() {
            Ok(num) => ships_left = num,
            // in the case that the next line is actually the error message
            Err(_) => {
                println!("{line}");
                line.clear();
                reader.read_line(&mut line).unwrap();
                line = line.trim().to_string();
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
    line = line.trim().to_string();
    let mut is_not_winner = line.parse::<bool>().unwrap();
    while is_not_winner {
        accept_boards(&mut reader);
        line.clear();
        reader.read_line(&mut line).unwrap();
        print!("{line}");

        let mut input = String::new();
        loop {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();
            if !input.is_empty() {
                break;
            }
        }
        writeln!(writer, "{input}").unwrap();

        accept_boards(&mut reader);
        line.clear();
        reader.read_line(&mut line).unwrap();
        println!("{line}");

        line.clear();
        reader.read_line(&mut line).unwrap();
        line = line.trim().to_string();
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
        print!("{line}");
    }
}
