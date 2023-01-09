use crate::prelude::*;

pub struct Canvas {
    size: UVec2,
    string: String,
}

impl Canvas {
    pub fn new(size: UVec2) -> Self {
        let string = str::repeat(" ", (size.x * size.y) as usize);
        Self { size, string }
    }

    pub fn put(&mut self, pos: IVec2, glyph: char) {
        let i = pos.y as usize * self.size.x as usize + pos.x as usize;
        self.string.replace_range(i..i + 1, std::str::from_utf8(&[glyph as u8]).unwrap());
    }

    pub fn print(&self) {
        let width = self.size.x;
        print!("   ");
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
    use crate::prelude::*;

    #[test]
    fn print_test() {
        let mut canvas = Canvas::new(UVec2::new(10, 5));
        canvas.put(UVec2::new(1, 1), '*');
        canvas.put(UVec2::new(2, 2), '*');
        canvas.put(UVec2::new(3, 3), '*');
        canvas.put(UVec2::new(4, 4), '*');
        canvas.print();
    }
}
