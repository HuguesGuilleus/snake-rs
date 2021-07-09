/// On wall where the snake can't go over.
pub struct Wall {
    begin_x: usize,
    end_x: usize,
    begin_y: usize,
    end_y: usize,
}
impl Wall {
    pub fn new(begin_x: usize, end_x: usize, begin_y: usize, end_y: usize) -> Self {
        Self {
            begin_x,
            end_x,
            begin_y,
            end_y,
        }
    }

    /// Return all point of the wall.
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (self.begin_x..self.end_x)
            .map(move |x| (self.begin_y..self.end_y).map(move |y| (x, y)))
            .flatten()
    }

    /// Tell if a point if over the wall.
    pub fn over(&self, x: usize, y: usize) -> bool {
        self.begin_x <= x && x < self.end_x && self.begin_y <= y && y < self.end_y
    }
}
#[test]
fn wall_iter() {
    let r = Wall::new(3, 5, 1, 2)
        .iter()
        .collect::<Vec<(usize, usize)>>();
    assert_eq!(vec![(3, 1), (4, 1)], r);
}
#[test]
fn wall_over() {
    let w = Wall::new(3, 5, 1, 2);
    assert_eq!(true, w.over(3, 1));
    assert_eq!(true, w.over(4, 1));

    assert_eq!(false, w.over(3, 2));
    assert_eq!(false, w.over(4, 2));
    assert_eq!(false, w.over(3, 0));
    assert_eq!(false, w.over(4, 0));

    assert_eq!(false, w.over(2, 1));
    assert_eq!(false, w.over(5, 1));
}
