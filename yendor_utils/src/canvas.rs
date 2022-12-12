use crate::prelude::*;

pub struct Canvas {
    size: UVec2,
    string: String,
}

impl Canvas {
    pub fn new(size: impl Dimensions) -> Self {
        let string = str::repeat(" ", size.size());
        Self {
            size: size.as_uvec2(),
            string,
        }
    }

    pub fn put(&mut self, pos: impl Point, glyph: char) {
        let i = self.to_index(pos);
        self.string.replace_range(i..i + 1, std::str::from_utf8(&[glyph as u8]).unwrap());
    }

    fn to_index(&self, point: impl Point) -> usize { point.as_index_unchecked(self.size.width()) }

    pub fn print(&self) {
        let width = self.size.width();
        print!("  ");
        (0..width).for_each(|i| print!("{i}"));

        println!();

        let chars: Vec<_> = self.string.replace(' ', ".").chars().collect();
        for (i, line) in chars.chunks(self.size.x as usize).enumerate() {
            println!("{:>2} {}", i, String::from_iter(line.iter()));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::canvas::*;

    #[test]
    fn print_test() {
        let mut canvas = Canvas::new([10, 5]);
        canvas.put((1, 1), '*');
        canvas.put((2, 2), '*');
        canvas.put((3, 3), '*');
        canvas.put((4, 4), '*');
        canvas.print();
    }
}
