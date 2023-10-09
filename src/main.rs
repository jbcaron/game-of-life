enum ParseError {
    InvalidWidth { arg: &'static str },
    InvalidHeight { arg: &'static str },
    InvalidPercentage { arg: &'static str },
    TooManyArguments,
    NotEnoughArguments,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidWidth { arg } => write!(f, "invalid width: {arg}"),
            ParseError::InvalidHeight { arg } => write!(f, "invalid height: {arg}"),
            ParseError::InvalidPercentage { arg } => write!(f, "invalid percentage: {arg}"),
            ParseError::TooManyArguments => write!(f, "too many arguments"),
            ParseError::NotEnoughArguments => write!(f, "not enough arguments"),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Cell {
    Dead,
    Alive,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Alive => write!(f, "#"),
            Cell::Dead => write!(f, "."),
        }
    }
}

impl Cell {
    fn is_alive(self) -> bool {
        self == Cell::Alive
    }
    fn is_dead(self) -> bool {
        self == Cell::Dead
    }
}

struct Board {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Board {
    fn new(width: usize, height: usize, percentage: u32) -> Self {
        let mut cells = Vec::new();
        for _ in 0..(width * height) {
            let state = if (ftkit::random_number(0..=100) as u32) < percentage {
                Cell::Alive
            } else {
                Cell::Dead
            };

            cells.push(state);
        }
        Self {
            width,
            height,
            cells,
        }
    }

	/// This function is used to parse the arguments passed to the program.
	/// It returns a `Result` containing a `Board` if the arguments are valid,
    fn from_args() -> Result<Self, ParseError> {
        if ftkit::ARGS.len() < 4 {
            return Err(ParseError::NotEnoughArguments);
        } else if ftkit::ARGS.len() > 4 {
            return Err(ParseError::TooManyArguments);
        }

        let width = match ftkit::ARGS[1].parse::<usize>() {
            Ok(ok) if ok > 0 => ok,
            _ => {
                return Err(ParseError::InvalidWidth {
                    arg: &ftkit::ARGS[1],
                })
            }
        };

        let height = match ftkit::ARGS[2].parse::<usize>() {
            Ok(ok) if ok > 0 => ok,
            _ => {
                return Err(ParseError::InvalidHeight {
                    arg: &ftkit::ARGS[2],
                })
            }
        };

        let percent_alive = match ftkit::ARGS[3].parse::<u32>() {
            Ok(ok) if ok <= 100 => ok,
            _ => {
                return Err(ParseError::InvalidPercentage {
                    arg: &ftkit::ARGS[3],
                })
            }
        };
        Ok(Board::new(width, height, percent_alive))
    }

	/// This function is used to count the number of alive neighbors of a cell.
	/// It returns the number of alive neighbors.
	/// 
	/// # Arguments
	/// 
	/// * `index` - The index of the cell in the `cells` vector.
	fn count_neighbors_alive(&self, index: usize) -> u8 {
		let mut alive_count = 0;
	
		let directions = [
			(-1, -1), (-1, 0), (-1, 1),
			(0, -1),          (0, 1),
			(1, -1), (1, 0),  (1, 1)
		];
	
		let row = index / self.width;
		let col = index % self.width;
	
		for &(dx, dy) in &directions {
			let new_row = row as isize + dx;
			let new_col = col as isize + dy;
	
			if new_row >= 0 && new_row < self.height as isize &&
			   new_col >= 0 && new_col < self.width as isize {
				let neighbor_index = (new_row * self.width as isize + new_col) as usize;
	
				if self.cells[neighbor_index] == Cell::Alive {
					alive_count += 1;
				}
			}
		}
	
		alive_count
	}
	
	// This function is used to compute the next state of the board.
	// A new vector of cells is created and then assigned to the `cells` field.
	// A cell keep alive if it has 2 or 3 alive neighbors.
	// A cell is born if it has 3 alive neighbors.
    fn step(&mut self) {
        let mut next: Vec<Cell> = Vec::new();

        for i in 0..(self.width * self.height) {
        //for i in 0..self.cells.len() {
			let neighbors_alive = self.count_neighbors_alive(i);

			let cell = match (self.cells[i], neighbors_alive) {
				(Cell::Alive, 2 | 3) => Cell::Alive,
				(Cell::Dead, 3) => Cell::Alive,
				(_, _) => Cell::Dead,
			};
			next.push(cell);
		}

		self.cells = next;
    }

    fn print(&self, clear: bool) {
        if clear {
            for _ in 0..self.height {
				// clear current line
                print!("\x1B[2K");
				// move cursor to the beginning of the line
                print!("\x1B[1A");
            }
        }
        for i in 0..(self.width * self.height) {
            if i % self.width == 0 {
                println!();
            }
            print!("{} ", self.cells[i]);
        }
    }
}

/// This program is used to simulate the Game of Life.
/// Usage: ./prgm <width> <heightftkit = "^0.1.0"> <percentage-alive>
/// 
/// # Exemple
/// 
/// ```
/// $ ./prgm 10 10 50
/// ```
fn main() {
    let mut board = match Board::from_args() {
        Ok(ok) => ok,
        Err(error) => {
            eprintln!("Error: {}", error);
            return;
        }
    };

    board.print(false);
    loop {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.2));
        board.step();
        board.print(true);
    }
}
