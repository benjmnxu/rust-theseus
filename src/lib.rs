use std::error::Error;
use std::fmt::Display;
use std::io;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameStatus {
    Win,
    Lose,
    Continue,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoardError {
    InvalidCharacter(char),
    InvalidSize,
    NoMinotaur,
    NoTheseus,
    NoGoal,
    MultipleMinotaur,
    MultipleTheseus,
    MultipleGoal,
}
impl Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardError::InvalidCharacter(c) => write!(f, "Invalid character: {}", c),
            BoardError::InvalidSize => write!(f, "Invalid size"),
            BoardError::NoMinotaur => write!(f, "No minotaur"),
            BoardError::NoTheseus => write!(f, "No theseus"),
            BoardError::NoGoal => write!(f, "No goal"),
            BoardError::MultipleMinotaur => write!(f, "Multiple minotaur"),
            BoardError::MultipleTheseus => write!(f, "Multiple theseus"),
            BoardError::MultipleGoal => write!(f, "Multiple goal"),
        }
    }
}
impl Error for BoardError {}

#[derive(Clone)]
pub struct Grid {
    board: Vec<Vec<char>>,
}

impl Grid {
    pub fn move_entity(&mut self, symbol: char, y: usize, x: usize, new_y: usize, new_x: usize) {
        self.board[y][x] = ' ';
        self.board[new_y][new_x] = symbol;
    }

    pub fn new(board: Vec<Vec<char>>) -> Result<Grid, BoardError> {
        Ok(Grid { board })
    }

    pub fn is_theseus(&self, row: usize, col: usize) -> bool {
        self.board[row][col] == 'T'
    }

    pub fn is_minotaur(&self, row: usize, col: usize) -> bool {
        self.board[row][col] == 'M'
    }

    pub fn is_wall(&self, row: usize, col: usize) -> bool {
        self.board[row][col] == 'X'
    }

    pub fn is_goal(&self, row: usize, col: usize) -> bool {
        self.board[row][col] == 'G'
    }

    pub fn is_empty(&self, row: usize, col: usize) -> bool {
        self.board[row][col] == ' '
    }
}

#[derive(Clone)]
pub struct Game {
    grid: Grid,
    theseus: (usize, usize),
    minotaur: (usize, usize),
    goal: (usize, usize), // TODO: Implement the Game struct
}

impl Game {
    // TODO: replace the function body with your implementation
    pub fn from_board(board: &str) -> Result<Game, BoardError> {
        let mut vector_grid = Vec::new();

        let mut theseus_count = 0;
        let mut theseus = (0, 0);
        let mut minotaur_count = 0;
        let mut minotaur = (0, 0);
        let mut goal_count = 0;
        let mut goal = (0, 0);

        for (i, line) in board.lines().enumerate() {
            let mut row = Vec::new();
            for (j, char) in line.chars().enumerate() {
                row.push(char);
                if char == 'T' {
                    theseus_count += 1;
                    if theseus_count > 1 {
                        return Err(BoardError::MultipleTheseus);
                    }
                    theseus = (i, j);
                } else if char == 'M' {
                    minotaur_count += 1;
                    if minotaur_count > 1 {
                        return Err(BoardError::MultipleTheseus);
                    }
                    minotaur = (i, j);
                } else if char == 'G' {
                    goal_count += 1;
                    if goal_count > 1 {
                        return Err(BoardError::MultipleTheseus);
                    }
                    goal = (i, j);
                }
            }

            vector_grid.push(row);
        }

        if theseus_count == 0 {
            return Err(BoardError::NoTheseus);
        }

        if minotaur_count == 0 {
            return Err(BoardError::NoMinotaur);
        }

        if goal_count == 0 {
            return Err(BoardError::NoGoal);
        }

        let game_grid = Grid::new(vector_grid)?;

        Ok(Game {
            grid: game_grid,
            theseus,
            minotaur,
            goal,
        })
    }

    // TODO
    pub fn show(&self) {
        for vec in &self.grid.board {
            for element in vec {
                if *element == 'X' {
                    print!("█");
                } else {
                    print!("{}", element);
                }
            }
            println!();
        }
    }

    // TODO
    pub fn minotaur_move(&mut self) {
        let (ty, tx) = self.theseus;
        let (my, mx) = self.minotaur;

        let d_x;
        let d_y;

        let diff_x = (tx as isize - mx as isize).abs();
        let diff_y = (ty as isize - my as isize).abs();

        if diff_x > 0 {
            d_x = if tx < mx { -1 } else { 1 };

            let new_x = (self.minotaur.1 as isize + d_x) as usize;

            if !self.is_wall(self.minotaur.0, new_x) {
                self.grid.move_entity(
                    'M',
                    self.minotaur.0,
                    self.minotaur.1,
                    self.minotaur.0,
                    new_x,
                );
                self.minotaur = (self.minotaur.0, new_x);
                return;
            }
        }

        if diff_y > 0 {
            d_y = if ty < my { -1 } else { 1 };

            let new_y = (self.minotaur.0 as isize + d_y) as usize;

            if !self.is_wall(new_y, self.minotaur.1) {
                self.grid.move_entity(
                    'M',
                    self.minotaur.0,
                    self.minotaur.1,
                    new_y,
                    self.minotaur.1,
                );
                self.minotaur = (new_y, self.minotaur.1);
            }
        }
    }

    // TODO
    pub fn theseus_move(&mut self, command: Command) {
        let d_x;
        let d_y;
        match command {
            Command::Up => {
                d_x = 0;
                d_y = -1;
            }
            Command::Down => {
                d_x = 0;
                d_y = 1;
            }
            Command::Left => {
                d_x = -1;
                d_y = 0;
            }
            Command::Right => {
                d_x = 1;
                d_y = 0;
            }
            Command::Skip => return,
        }

        let new_y = (self.theseus.0 as isize + d_y) as usize;
        let new_x = (self.theseus.1 as isize + d_x) as usize;

        if self.is_wall(new_y, new_x) {
            return;
        }
        self.grid
            .move_entity('T', self.theseus.0, self.theseus.1, new_y, new_x);

        self.theseus = (new_y, new_x);
    }

    // TODO: replace the function body with your implementation
    pub fn status(&self) -> GameStatus {
        if self.theseus == self.goal {
            GameStatus::Win
        } else if self.theseus == self.minotaur {
            GameStatus::Lose
        } else {
            GameStatus::Continue
        }
    }
}

impl Game {
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is Theseus
    pub fn is_theseus(&self, row: usize, col: usize) -> bool {
        self.grid.is_theseus(row, col)
    }
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is Minotaur
    pub fn is_minotaur(&self, row: usize, col: usize) -> bool {
        self.grid.is_minotaur(row, col)
    }
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is a wall
    pub fn is_wall(&self, row: usize, col: usize) -> bool {
        self.grid.is_wall(row, col)
    }
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is the goal
    pub fn is_goal(&self, row: usize, col: usize) -> bool {
        self.grid.is_goal(row, col)
    }
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is empty
    pub fn is_empty(&self, row: usize, col: usize) -> bool {
        self.grid.is_empty(row, col)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command {
    /// Move one tile up
    Up,
    /// Move one tile down
    Down,
    /// Move one tile left
    Left,
    /// Move one tile right
    Right,
    /// Don't move at all
    Skip,
}

//  To get a command from the user, you can use the following code:
//  ```
//  let line = stdin.lines().next().unwrap().unwrap();
//  ```
//  This will read a line from the user and store it in the `buffer` string.
//
//  Unfortunately, since stdin is line-buffered, everytime you enter a command while playing the
//  game you will have to press "enter" afterwards to send a new line.
//
//  While using the arrow keys to take inputs would be natural, it can be difficult to handle arrow
//  keys in a way that works on all devices. Therefore, it's recommended that you either use "w",
//  "a", "s", and "d" to take input, or else the words "up", "down", "left", "right". You can take
//  input however you like, so long as you document it here in a comment and it is reasonable to
//  use as a player.
pub fn input(stdin: impl io::Read + io::BufRead) -> Option<Command> {
    let line = stdin.lines().next().unwrap().unwrap();

    return match line.as_str() {
        "w" => Some(Command::Up),
        "a" => Some(Command::Left),
        "s" => Some(Command::Down),
        "d" => Some(Command::Right),
        _ => Some(Command::Skip),
    };
}
