pub mod client;
pub mod server;

#[derive(Clone)]
pub struct BattleShip {
    turn: i32,
    a_winner: bool,
    num_hits: i32,
    guess_board: Vec<Vec<String>>,
    ship_board: Vec<Vec<String>>,
    ships: Vec<i32>,
}

pub struct Coordinate {
    row_start: i32,
    row_end: i32,
    column_start: i32,
    column_end: i32,
}

impl Coordinate {
    pub fn new(first_coord: &str, second_coord: &str) -> Self {
        Coordinate {
            row_start: letter_to_num(&first_coord[0..1]).unwrap(),
            row_end: letter_to_num(&second_coord[0..1]).unwrap(),
            column_start: first_coord[1..].parse().unwrap(),
            column_end: second_coord[1..].parse().unwrap(),
        }
    }
}

impl Default for BattleShip {
    fn default() -> Self {
        Self::new()
    }
}

impl BattleShip {
    pub fn new() -> Self {
        let board: Vec<Vec<String>> = vec![vec![String::from("-"); 10]; 10];
        Self {
            turn: 0,
            a_winner: false,
            num_hits: 0,
            guess_board: board.clone(),
            ship_board: board,
            ships: vec![2, 3, 3, 4, 5, 6],
        }
    }

    pub fn print_ship_board(&self) -> String {
        let mut result = String::from("  0 1 2 3 4 5 6 7 8 9\n");
        for i in 0..self.ship_board.len() {
            result = format!("{result}{} ", num_to_letter(&i).unwrap());
            for j in 0..self.ship_board[i].len() {
                result = format!("{result}{} ", self.ship_board[i][j]);
            }
            result = format!("{result}\n");
        }
        if !self.ships.is_empty() {
            result = format!("{result}Ships sizes available: ");
            for size in &self.ships {
                result = format!("{result}{size} ");
            }
        }
        result = format!("{result}\n");

        result
    }

    pub fn print_boards(&self) -> String {
        let mut result = String::from("  0 1 2 3 4 5 6 7 8 9    0 1 2 3 4 5 6 7 8 9\n");
        for i in 0..self.ship_board.len() {
            result = format!("{result}{} ", num_to_letter(&i).unwrap());
            for j in 0..self.ship_board[i].len() {
                result = format!("{result}{} ", self.guess_board[i][j]);
            }
            result = format!("{result}   ");
            for k in 0..self.ship_board[i].len() {
                result = format!("{result}{} ", self.ship_board[i][k]);
            }
            result = format!("{result}{}\n", num_to_letter(&i).unwrap());
        }
        result = format!("{result}X = hit\tO = miss           Your ships\n");

        result
    }

    fn can_place_ship(&mut self, coordinate: &Coordinate) -> bool {
        if coordinate.row_start == coordinate.row_end {
            let row = &coordinate.row_start;
            let start = &coordinate.column_start.min(coordinate.column_end);
            let end = &coordinate.column_start.max(coordinate.column_end);
            if !self
                .ships
                .iter()
                .any(|size| *size == (end - start).abs() + 1)
            {
                return false;
            }
            for i in *start..=*end {
                if self.ship_board[*row as usize][i as usize] == ">"
                    || self.ship_board[*row as usize][i as usize] == "^"
                {
                    return false;
                }
            }
            self.ships.remove(
                self.ships
                    .iter()
                    .enumerate()
                    .find(|size| *size.1 == (end - start).abs() + 1)
                    .unwrap()
                    .0,
            );
        } else if coordinate.column_start == coordinate.column_end {
            let column = &coordinate.column_start;
            let start = &coordinate.row_start.min(coordinate.row_end);
            let end = &coordinate.row_start.max(coordinate.row_end);
            if !self
                .ships
                .iter()
                .any(|size| *size == (end - start).abs() + 1)
            {
                return false;
            }
            for i in *start..=*end {
                if self.ship_board[*column as usize][i as usize] == ">"
                    || self.ship_board[*column as usize][i as usize] == "^"
                {
                    return false;
                }
            }
            self.ships.remove(
                self.ships
                    .iter()
                    .enumerate()
                    .find(|size| *size.1 == (end - start).abs() + 1)
                    .unwrap()
                    .0,
            );
        } else {
            return false;
        }
        true
    }

    pub fn place_ship(&mut self, coordinate: &Coordinate) {
        if coordinate.row_start == coordinate.row_end {
            let row = &coordinate.row_start;
            let start = &coordinate.column_start.min(coordinate.column_end);
            let end = &coordinate.column_start.max(coordinate.column_end);
            for i in *start..=*end {
                self.ship_board[*row as usize][i as usize] = ">".to_string();
            }
        } else {
            let column = &coordinate.column_start;
            let start = &coordinate.row_start.min(coordinate.row_end);
            let end = &coordinate.row_start.max(coordinate.row_end);
            for i in *start..=*end {
                self.ship_board[*column as usize][i as usize] = "^".to_string();
            }
        }
    }

    pub fn guess(&mut self, other: &mut BattleShip, coordinate: &Coordinate) {
        let row = coordinate.row_start;
        let column = coordinate.column_start;
        if self.is_hit(other, coordinate) {
            self.guess_board[row as usize][column as usize] = "X".to_string();
            other.ship_board[row as usize][column as usize] = "X".to_string();
            self.num_hits += 1;
        } else {
            self.guess_board[row as usize][column as usize] = "O".to_string();
            other.ship_board[row as usize][column as usize] = "O".to_string();
        }
        self.turn += 1;
    }

    fn is_hit(&self, other: &BattleShip, coordinate: &Coordinate) -> bool {
        let (row, column) = (
            coordinate.row_start as usize,
            coordinate.column_start as usize,
        );
        other.ship_board[row][column] == ">" || other.ship_board[row][column] == "^"
    }

    pub fn check_if_winner(&mut self) -> bool {
        if self.num_hits == 23 {
            self.a_winner = true;
            return true;
        }
        false
    }

    pub fn get_ships_left(&self) -> usize {
        self.ships.len()
    }

    pub fn turn(&self) -> i32 {
        self.turn
    }
}

fn num_to_letter(num: &usize) -> Result<String, &'static str> {
    match num {
        0 => Ok(String::from("A")),
        1 => Ok(String::from("B")),
        2 => Ok(String::from("C")),
        3 => Ok(String::from("D")),
        4 => Ok(String::from("E")),
        5 => Ok(String::from("F")),
        6 => Ok(String::from("G")),
        7 => Ok(String::from("H")),
        8 => Ok(String::from("I")),
        9 => Ok(String::from("J")),
        _ => Err("Bad argument to num_to_letter()"),
    }
}

fn letter_to_num(str: &str) -> Result<i32, &'static str> {
    match str {
        "A" => Ok(0),
        "B" => Ok(1),
        "C" => Ok(2),
        "D" => Ok(3),
        "E" => Ok(4),
        "F" => Ok(5),
        "G" => Ok(6),
        "H" => Ok(7),
        "I" => Ok(8),
        "J" => Ok(9),
        _ => Err("Bad argument to letter_to_num()"),
    }
}
