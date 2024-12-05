use std::io;

#[derive(Debug)]
struct Grid(Vec<String>);

impl Grid {
    fn new() -> Grid {
        Grid(Vec::new())
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn get_char(&self, x: usize, y: usize) -> char {
        self.0[y].as_bytes()[x] as char
    }

    fn find_word(&self, word: &str, x: usize, y: usize, (dx, dy): (i32, i32)) -> bool {
        word.chars().enumerate().all(|(i, c)| {
            let ix = x as i32 + dx * i as i32;
            let iy = y as i32 + dy * i as i32;
            if ix < 0 || ix as usize >= self.width() || iy < 0 || iy as usize >= self.height() {
                false
            } else {
                c == self.get_char(ix as usize, iy as usize)
            }
        })
    }

    fn is_x_mas(&self, x: usize, y: usize) -> bool {
        if x < 1 || x >= self.width() - 1 || y < 1 || y >= self.height() - 1 {
            false
        } else {
            let tl = self.get_char(x - 1, y - 1);
            let tr = self.get_char(x + 1, y - 1);
            let bl = self.get_char(x - 1, y + 1);
            let br = self.get_char(x + 1, y + 1);

            self.get_char(x, y) == 'A'
                && (tl == 'M' && br == 'S' || tl == 'S' && br == 'M')
                && (tr == 'M' && bl == 'S' || tr == 'S' && bl == 'M')
        }
    }
}

impl FromIterator<String> for Grid {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        let mut g = Grid::new();

        for s in iter {
            g.0.push(s);
        }

        g
    }
}

fn main() {
    let input: Grid = io::stdin().lines().map(Result::unwrap).collect();
    let width = input.width();
    let height = input.height();
    let directions = vec![
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];
    let word_search = (0..height).fold(0, |sum, y| {
        sum + (0..width).fold(0, |sum, x| {
            sum + directions.iter().fold(0, |sum, direction| {
                sum + if input.find_word("XMAS", x, y, *direction) {
                    1
                } else {
                    0
                }
            })
        })
    });
    let x_mas = (1..height - 1).fold(0, |sum, y| {
        sum + (1..width - 1).fold(0, |sum, x| sum + if input.is_x_mas(x, y) { 1 } else { 0 })
    });

    println!("word search: {}, x-mas: {}", word_search, x_mas);
}
