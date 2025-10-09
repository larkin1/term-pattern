use std::thread;
use std::time::{Duration, Instant};
use rand::Rng;
use std::io::{self, Write};

struct Array {
    width: usize,
    height: usize,
    data: Vex<Vec<u16>>,
}

// General Functions
impl Array {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![vec![0; width]; height],
        }
    }

    fn get(&self, x: usize, y: usize) -> Self {
        self.data[y][x]
    }

    fn set(&mut self, x: usize, y: usize, val: u16) -> () {
        self.data[y][x] = val;
    }
}

// Algorithms
impl Array {
    fn perlin() {
        let arr = Array::new(width, height);

    }
}

fn main() {
    let arr_x = 300;
    let arr_y = 300;
    let items: [&str; 5] = [".", "-", "+", "#" ,"@"];
    let frame_time = Duration::from_millis(33);

    let mut array = Array::new(arr_x, arr_y);
    loop {
        let t_start = Instant::now();

        for y in 0..size.y {
            let start = rand::rng().random_range(0..=(2000 - size.x as usize));
            let end = start + size.x as usize;
            let slice=&fullframe[start..end];
            array[y as usize] = slice.to_vec();
        }

        let mut output = String::with_capacity((size.x as usize + 1) * size.y as usize);
        for row in &array {
            for col in row {
                output.push_str(items[*col as usize]);
            }
            output.push('\n');
        }

        print!("\x1B[2J\x1B[H{}", output);
        io::stdout().flush().unwrap();

        let elapsed = t_start.elapsed();
        if elapsed < frame_time {
        thread::sleep(frame_time - elapsed);}

        break;
    }
}