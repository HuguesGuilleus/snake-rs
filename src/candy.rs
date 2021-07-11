use crate::Board;
use rand::rngs::ThreadRng;

pub struct Candy<'a> {
    pub coord: Option<(usize, usize)>,
    board: &'a Board,
    expiration: usize,
}
impl<'a> Candy<'a> {
    // In tick
    const LIFE_RAND: usize = 50;
    const LIFE_MIN: usize = 30;

    // Create a new candy.
    pub fn new(board: &'a Board) -> Self {
        Self {
            board: board,
            coord: None,
            expiration: Self::new_life(),
        }
    }

    /// The candy is eated by the snake.
    pub fn eat(&mut self) {
        self.coord = None;
        self.expiration = Self::new_life();
    }

    /// If lifetime is over, move the candy.
    pub fn regenerate(&mut self) {
        if self.expiration == 0 {
            self.expiration = Self::new_life();
            loop {
                let x = rand::random::<usize>() % self.board.width;
                let y = rand::random::<usize>() % self.board.height;
                if self.board.on_wall(x, y) {
                    self.coord = Some((x, y));
                    break;
                }
            }
        } else {
            self.expiration -= 1;
        }
    }

    /// Get a new random life.
    fn new_life() -> usize {
        Self::LIFE_MIN + rand::random::<usize>() % Self::LIFE_RAND
    }
}
