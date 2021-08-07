// mod candy;
mod text_view;
mod wall;
use crate::wall::Wall;
pub use text_view::TextView;

/// Interface to print the current party.
pub trait View {
    /// Clear the view.
    fn clear(&mut self);
    /// Draw one part of the wall.
    fn wall(&mut self, x: usize, y: usize);
    /// Display a candy.
    fn candy(&mut self, x: usize, y: usize);
    /// Display the snake.
    fn snake(&mut self, x: usize, y: usize);
    /// After all display data are send, print to user.
    fn print(&mut self);

    /// take all information and print.
    fn all(
        &mut self,
        walls: &[Wall],
        candys: impl Iterator<Item = (usize, usize)>,
        snake: &[(usize, usize)],
    ) {
        self.clear();
        walls
            .iter()
            .map(|w| w.iter())
            .flatten()
            .for_each(|(x, y)| self.wall(x, y));
        candys.for_each(|(x, y)| self.candy(x, y));
        snake.into_iter().for_each(|&(x, y)| self.snake(x, y));
        self.print();
    }

    /// Display the result at the end of the game.
    fn result(&mut self, score: Option<usize>);
}

/// The result of Controler.Control()
#[derive(Debug, Copy, Clone)]
pub enum Control {
    /// No action from player.
    None,
    /// The player want to exit.
    Exit,
    /// The user want go to left.
    Left,
    /// The user want to go the the right.
    Right,
    /// The user want to go the the top.
    Up,
    /// The user want to go the the bottom.
    Down,
}

/// The direction of the snake.
#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct BoardDimension {
    width: usize,
    height: usize,
}

#[derive(Debug)]
pub struct Board {
    width: usize,
    height: usize,
    walls: Vec<Wall>,
}
impl Board {
    /// Create a new baord with a cross walls.
    pub fn new((width, height): (usize, usize)) -> Self {
        let x = width / 2;
        let y = height / 2;
        let walls = vec![
            Wall::new(0, width, y, y + 1),
            Wall::new(x, x + 1, 0, height),
        ];

        Self {
            width,
            height,
            walls,
        }
    }

    /// Create a board without any walls.
    pub fn no_wall((width, height): (usize, usize)) -> Self {
        Self {
            width,
            height,
            walls: vec![],
        }
    }

    /// Play at this game.
    pub fn play(&self, mut control: impl FnMut() -> Control, mut view: impl View) -> Option<usize> {
        // generate candys
        let mut snake = Snake::new(self);

        let score = loop {
            match control() {
                Control::Left => snake.set_direction(Direction::Left),
                Control::Right => snake.set_direction(Direction::Right),
                Control::Down => snake.set_direction(Direction::Down),
                Control::Up => snake.set_direction(Direction::Up),
                Control::None => {}
                Control::Exit => break None,
            };

            // regenerate candy

            if snake.walk() {
                break Some(snake.score());
            }

            view.all(&self.walls, vec![].into_iter(), &snake.body);
        };

        view.result(score);
        return score;
    }

    /// Return true if the point is on a wall.
    fn on_wall(&self, (x, y): (usize, usize)) -> bool {
        self.walls.iter().any(move |w| w.over(x, y))
    }
}

#[test]
fn board_on_wall() {
    let b = Board::new((30, 20));
    assert_eq!(true, b.on_wall((15, 10)));
    assert_eq!(true, b.on_wall((3, 10)));
    assert_eq!(true, b.on_wall((15, 19)));

    assert_eq!(false, b.on_wall((0, 0)));
    assert_eq!(false, b.on_wall((39, 19)));
    assert_eq!(false, b.on_wall((14, 9)));
    assert_eq!(false, b.on_wall((16, 11)));
    assert_eq!(false, b.on_wall((5, 9)));
    assert_eq!(false, b.on_wall((14, 5)));
}

/// One snake
struct Snake<'a> {
    body: Vec<(usize, usize)>,
    direction: Direction,
    board: &'a Board,
}

impl<'a> Snake<'a> {
    const INIT_SIZE: usize = 3;

    fn new(board: &'a Board) -> Self {
        Self {
            body: (0..Snake::INIT_SIZE).map(|x| (x + 3, 2)).collect(),
            direction: Direction::Right,
            board,
        }
    }

    /// Return the number of boady element screated
    fn score(&self) -> usize {
        self.body.len() - Snake::INIT_SIZE
    }

    /// Change the direction of the snake.
    fn set_direction(&mut self, direction: Direction) {
        match (self.direction, direction) {
            (Direction::Down, Direction::Up)
            | (Direction::Up, Direction::Down)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left) => {}
            _ => self.direction = direction,
        }
    }

    /// Move the snake and return true if the snake walk over itself.
    fn walk(&mut self) -> bool {
        let head = self.next();
        if self.board.on_wall(head) {
            true
        } else {
            let l = self.body.len() - 1;
            for i in 0..l {
                self.body[i] = self.body[i + 1];
            }
            self.body[l] = head;
            false
        }
    }

    /// Return the next head.
    ///
    /// The head is on the board, but can be
    fn next(&self) -> (usize, usize) {
        let head = self.body[self.body.len() - 1];
        match self.direction {
            Direction::Left if head.0 == 0 => (self.board.width - 1, head.1),
            Direction::Left => (head.0 - 1, head.1),
            Direction::Right if head.0 == self.board.width - 1 => (0, head.1),
            Direction::Right => (head.0 + 1, head.1),

            Direction::Down if head.1 == self.board.height - 1 => (head.0, 0),
            Direction::Down => (head.0, head.1 + 1),
            Direction::Up if head.1 == 0 => (head.0, self.board.height - 1),
            Direction::Up => (head.0, head.1 - 1),
        }
    }
}

#[test]
fn snake_next() {
    let new = |body: (usize, usize), d: Direction| -> (usize, usize) {
        let b = Board::no_wall((6, 4));
        Snake {
            board: &b,
            body: vec![body],
            direction: d,
        }
        .next()
    };

    assert_eq!((0, 1), new((1, 1), Direction::Left));
    assert_eq!((2, 1), new((1, 1), Direction::Right));
    assert_eq!((1, 2), new((1, 1), Direction::Down));
    assert_eq!((1, 0), new((1, 1), Direction::Up));

    assert_eq!((0, 3), new((5, 3), Direction::Right));
    assert_eq!((5, 0), new((5, 3), Direction::Down));
    assert_eq!((5, 0), new((0, 0), Direction::Left));
    assert_eq!((0, 3), new((0, 0), Direction::Up));
}
