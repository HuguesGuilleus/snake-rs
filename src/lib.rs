mod wall;

/// Interface to print the current party.
pub trait View {
    /// Clear the view.
    fn clear();
    /// Draw one part of the wall.
    fn wall(x: usize, y: usize);
    /// Display a candy.
    fn candy(x: usize, y: usize);
    // /// Display the snake.
    fn snake(s: &[(usize, usize)]);
    /// Display the result at the end of the game.
    fn result(score: Option<usize>);
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
