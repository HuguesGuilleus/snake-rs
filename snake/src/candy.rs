use crate::Board;

pub struct Candy<'a> {
    pub coord: Option<(usize, usize)>,
    board: &'a Board,
    expiration: usize,
    rand: fn() -> usize,
}
impl<'a> Candy<'a> {
    // In tick
    const LIFE_RAND: usize = 50;
    const LIFE_MIN: usize = 40;

    // Create a new candy.
    pub fn new(board: &'a Board, rand: fn() -> usize) -> Self {
        Self {
            board: board,
            coord: None,
            expiration: Self::new_life(rand),
            rand,
        }
    }

    // Is the coord is the same as the coord of this candy, the candy is eated.
    pub fn eat(&mut self, coord: (usize, usize)) -> bool {
        match self.coord {
            Some(c) if c == coord => {
                self.coord = None;
                self.expiration = Self::new_life(self.rand);
                true
            }
            _ => false,
        }
    }

    /// If lifetime is over, move the candy.
    pub fn regenerate(&mut self) {
        if self.expiration == 0 {
            self.expiration = Self::new_life(self.rand);
            loop {
                let x = (&self.rand)() % self.board.width;
                let y = (&self.rand)() % self.board.height;
                if !self.board.on_wall((x, y)) {
                    self.coord = Some((x, y));
                    break;
                }
            }
        } else {
            self.expiration -= 1;
        }
    }

    /// Get a new random life.
    fn new_life(rand: fn() -> usize) -> usize {
        Self::LIFE_MIN + rand() % Self::LIFE_RAND
    }
}
