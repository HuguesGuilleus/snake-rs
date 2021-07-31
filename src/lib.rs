mod candy;
mod wall;
use crate::wall::Wall;

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
#[derive(Debug)]
pub enum Control {
    /// The gamer want to exit.
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

#[derive(Debug)]
pub struct BoardDimension {
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
    pub fn new(dimension: BoardDimension) -> Self {
        let x = dimension.width / 2;
        let y = dimension.height / 2;
        let walls = vec![
            Wall::new(0, dimension.width, y, y + 1),
            Wall::new(x, x + 1, 0, dimension.height),
        ];

        Self {
            width: dimension.width,
            height: dimension.height,
            walls,
        }
    }

    /// Play at this game.
    pub fn play(control: impl Iterator<Item = Control>, view: impl View) -> Option<usize> {
        unimplemented!()
    }

    // Return true if the point is on a wall.
    fn on_wall(&self, x: usize, y: usize) -> bool {
        self.walls.iter().any(move |w| w.over(x, y))
    }
}

#[test]
fn board_on_wall() {
    let b = Board::new(BoardDimension {
        width: 30,
        height: 20,
    });
    assert_eq!(true, b.on_wall(15, 10));
    assert_eq!(true, b.on_wall(3, 10));
    assert_eq!(true, b.on_wall(15, 19));

    assert_eq!(false, b.on_wall(0, 0));
    assert_eq!(false, b.on_wall(39, 19));
    assert_eq!(false, b.on_wall(14, 9));
    assert_eq!(false, b.on_wall(16, 11));
    assert_eq!(false, b.on_wall(5, 9));
    assert_eq!(false, b.on_wall(14, 5));
}
