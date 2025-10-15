use std::thread;
use std::time::{Duration, Instant};
use rand::Rng;
use std::io::{self, Write};

// Array structure to store an array of values easily and cleanly.
#[allow(dead_code)]
struct Array {
    width: usize,
    height: usize,
    data: Vec<Vec<u16>>,
}

// Basic functions for reading and modifying Arrays.
impl Array {
    // Create a new array.
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![vec![0; width]; height],
        }
    }

    // Get the value of a position.
    fn get(&self, x: usize, y: usize) -> u16 {
        self.data[y][x]
    }

    // Set the value of a position.
    fn set(&mut self, x: usize, y: usize, val: u16) -> () {
        self.data[y][x] = val;
    }
}

impl Array {
    fn perlin(
        detail: u16, 
        aspect_ratio:f32, 
        width: usize, 
        height: usize
    ) -> Array {
        fn fade(t: f32) -> f32 { t * t * t * (t * (t * 6.0 - 15.0) + 10.0) }
        fn lerp(a: f32, b: f32, t: f32) -> f32 { a + t * (b - a) }
        fn dot(ax: f32, ay: f32, bx: f32, by: f32) -> f32 { ax * bx + ay * by }

        let x_cells = (
                width as f32 / detail as f32
            ).ceil() as usize + 1;
        let y_cells = (
                height as f32 / (detail as f32*aspect_ratio)
            ).ceil() as usize + 1;

        let mut grad_x = Array::new(x_cells, y_cells);
        let mut grad_y = Array::new(x_cells, y_cells);
        for y in 0..y_cells {
            for x in 0..x_cells {
                let angle = rand::rng().random_range(
                    0.0..std::f32::consts::TAU
                );
                grad_x.set(x, y, (angle.cos() * 1000.0) as u16);
                grad_y.set(x, y, (angle.sin() * 1000.0) as u16);
            }
        }

        let mut arr = Array::new(width, height);
        let mut raw = vec![vec![0.0_f32; width]; height];
        let mut min_val = f32::MAX;
        let mut max_val = f32::MIN;

        for y in 0..height {
            for x in 0..width {
                let fx = x as f32 / detail as f32;
                let fy = y as f32 / (detail as f32)*aspect_ratio;

                let x0 = fx.floor() as usize;
                let y0 = fy.floor() as usize;
                let x1 = x0 + 1;
                let y1 = y0 + 1;

                let dx = fx - x0 as f32;
                let dy = fy - y0 as f32;

                let (gx0, gy0) = (
                    grad_x.get(x0, y0) as f32 / 1000.0,
                    grad_y.get(x0, y0) as f32 / 1000.0,
                );
                let (gx1, gy1) = (
                    grad_x.get(x1, y0) as f32 / 1000.0,
                    grad_y.get(x1, y0) as f32 / 1000.0,
                );
                let (gx2, gy2) = (
                    grad_x.get(x0, y1) as f32 / 1000.0,
                    grad_y.get(x0, y1) as f32 / 1000.0,
                );
                let (gx3, gy3) = (
                    grad_x.get(x1, y1) as f32 / 1000.0,
                    grad_y.get(x1, y1) as f32 / 1000.0,
                );

                let d0 = dot(gx0, gy0, dx,     dy);
                let d1 = dot(gx1, gy1, dx - 1., dy);
                let d2 = dot(gx2, gy2, dx,     dy - 1.);
                let d3 = dot(gx3, gy3, dx - 1., dy - 1.);

                let u = fade(dx);
                let v = fade(dy);

                let ix0 = lerp(d0, d1, u);
                let ix1 = lerp(d2, d3, u);
                let value = lerp(ix0, ix1, v);

                raw[y][x] = value;

                if value < min_val { min_val = value; }
                if value > max_val { max_val = value; }
            }
        }

        for y in 0..height {
            for x in 0..width {
                let v = raw[y][x];
                let norm = (v - min_val) / (max_val - min_val);
                let final_value = (norm * 4.0).round() as u16;
                arr.set(x, y, final_value);
            }
        }

        arr
    }
}

fn main() {
    let arr_x = 70;
    let arr_y = 30;
    let detail = 8;
    let dewarp = 1.0/1.9;
    let items: [&str; 5] = [" ", ".", "-", "+", "#"];
    let frame_time = Duration::from_millis(33);

    loop {
        let t_start = Instant::now();

        // I know x and y are the wrong way around, 
        // but it's gonna be a PITA to fix so deal.
        let array = Array::perlin(detail, dewarp,arr_y, arr_x);

        let mut output = String::with_capacity(
            (arr_x as usize + 1) * arr_y as usize
        );
        for row in 0..arr_y {
            for col in 0..arr_x {
                output.push_str(
                    items[array.get(row, col) as usize]
                );
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