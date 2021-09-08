use super::*;
use std::ops::{Index, IndexMut};

/// Print the snake into the console.
pub struct TextView<W>
where
    W: FnMut(&str),
{
    width: usize,
    screen: Vec<char>,
    output_buffer: String,
    output_writer: W,
}

impl<W> Index<(usize, usize)> for TextView<W>
where
    W: FnMut(&str),
{
    type Output = char;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.screen[y * self.width + x]
    }
}
impl<W> IndexMut<(usize, usize)> for TextView<W>
where
    W: FnMut(&str),
{
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.screen[y * self.width + x]
    }
}

impl<W> TextView<W>
where
    W: FnMut(&str),
{
    pub fn new((width, height): (usize, usize), w: W) -> Self {
        let capacity = height * width;
        Self {
            width,
            screen: vec![' '; height * width],
            output_buffer: String::with_capacity(capacity),
            output_writer: w,
        }
    }
}

impl<W> View for TextView<W>
where
    W: FnMut(&str),
{
    fn clear(&mut self) {
        for i in 0..self.screen.len() {
            self.screen[i] = ' ';
        }
    }
    fn wall(&mut self, x: usize, y: usize) {
        self[(x, y)] = '#';
    }
    fn candy(&mut self, x: usize, y: usize) {
        self[(x, y)] = 'O';
    }
    fn snake(&mut self, x: usize, y: usize) {
        self[(x, y)] = '+';
    }
    fn print(&mut self) {
        {
            let s = &mut self.output_buffer;
            s.clear();
            self.screen.iter().map(|c| s.push(*c)).count();
        }

        (self.output_writer)(&self.output_buffer);
    }
    fn result(&mut self, score: Option<usize>) {
        self.output_buffer.clear();

        if let Some(score) = score {
            use std::fmt::Write as FmtWrite;
            write!(&mut self.output_buffer, "score: {}", score).unwrap();
        }

        (self.output_writer)(&self.output_buffer);
    }
}

#[test]
fn text_view() {
    let mut already_write = false;

    let walls = vec![Wall::new(1, 2, 0, 6)];
    let candys = vec![(0, 1), (2, 5)];
    let snake = vec![(2, 1), (2, 2), (2, 3)];

    TextView::new((3, 6), |s| {
        if already_write == true {
            panic!("the writer closure already print");
        }
        already_write = true;

        let mut expected = String::new();
        expected.push_str(" # ");
        expected.push_str("O#+");
        expected.push_str(" #+");
        expected.push_str(" #+");
        expected.push_str(" # ");
        expected.push_str(" #O");
        assert_eq!(&expected, s);
    })
    .all(&walls, candys.into_iter(), &snake);

    assert_eq!(true, already_write);
}
