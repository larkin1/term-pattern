use std::thread;
use std::time::{Duration, Instant};
use rand::Rng;
use std::io::{self, Write};

// Reminder: x is the lowest level in Array3D.
// This means it is: [z[y[x[val]]]] or val = array[z][y][x].
// In other words: z contains y, y contains x, and x contains the values.
#[allow(dead_code)]
struct Array3D {
    x_size: usize,
    y_size: usize,
    z_size: usize,
    data: Vec<Vec<Vec<f32>>>,
}

#[allow(dead_code)]
struct ArrayF32 {
    width: usize,
    height: usize,
    data: Vec<Vec<f32>>,
}

// Helper functions
impl ArrayF32 {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![vec![0f32; width]; height],
        }
    }

    fn get(&self, x: usize, y: usize) -> f32 {
        self.data[y][x]
    }

    fn set(&mut self, x: usize, y: usize, val: f32) -> () {
        self.data[y][x] = val;
    }
}

//Helper Functions
impl Array3D {
    fn new(x_size:usize, y_size:usize, z_size:usize) -> Self {
        Self {
            x_size,
            y_size,
            z_size,
            data: vec![vec![vec![0f32;x_size]; y_size]; z_size]
        }
    }

    fn set(&mut self, x:usize, y:usize, z: usize, val:f32) -> () {
        self.data[z][y][x] = val
    }
}

// Algorithms: Algorithms should produce a 3d array of slices in which
// each slice is one frame for the animation. Each frame should have
// cells, which are f32s and range from 0..1. Frames are baked, then
// rendered, so heavy algorithms cause a wait time at the start of the program.
impl Array3D {
    #[allow(non_snake_case)]
    fn perlin_3D(
        detail: u32,
        dewarp:f32,
        speed_scale:f32,
        x_size: u32,
        y_size: u32,
        z_size: u32
    ) -> Self {
        fn fade(t: f32) -> f32 { t * t * t * (t * (t * 6.0 - 15.0) + 10.0) }
        fn lerp(a: f32, b: f32, t: f32) -> f32 { a + t * (b - a) }
        fn dot(
            ax: f32,
            ay: f32,
            az: f32,
            bx: f32,
            by: f32,
            bz: f32
        ) -> f32 { ax * bx + ay * by + az * bz }

        let x_size_usize = x_size as usize;
        let y_size_usize = y_size as usize;
        let z_size_usize = z_size as usize;

        let mut vol = Array3D::new(x_size_usize, y_size_usize, z_size_usize);

        let gx_cells = (
            x_size as f32 / detail as f32
        ).ceil() as usize + 1;
        let gy_cells = (
            y_size as f32 / (detail as f32 * dewarp)
        ).ceil() as usize + 1;
        let gz_cells = (
            z_size as f32 / detail as f32
        ).ceil() as usize + 1;

        let mut grad_x = vec![
            vec![
                vec![
                    0.0_f32; gz_cells
                ]; gy_cells
            ]; gx_cells
        ];
        let mut grad_y = vec![
            vec![
                vec![
                    0.0_f32; gz_cells
                ]; gy_cells
            ]; gx_cells
        ];
        let mut grad_z = vec![
            vec![
                vec![
                    0.0_f32; gz_cells
                ]; gy_cells
            ]; gx_cells
        ];

        for gx in 0..gx_cells {
            for gy in 0..gy_cells {
                for gz in 0..gz_cells {

                    let angle1: f32 = rand::rng().random_range(
                        0.0..std::f32::consts::TAU
                    );
                    let angle2: f32 = rand::rng().random_range(
                        0.0..std::f32::consts::TAU
                    );
                    grad_x[gx][gy][gz] = angle1.cos() * angle2.sin();
                    grad_y[gx][gy][gz] = angle1.sin() * angle2.sin();
                    grad_z[gx][gy][gz] = angle2.cos();
                }
            }
        }

        for z in 0..z_size_usize {
            let mut min_val = f32::MAX;
            let mut max_val = f32::MIN;
            let mut slice = ArrayF32::new(x_size_usize, y_size_usize);
            for y in 0..y_size_usize {
                for x in 0..x_size_usize {
                    let fx = x as f32 / detail as f32;
                    let fy = y as f32 / (detail as f32 * dewarp);
                    let fz = (z as f32 / detail as f32) * speed_scale;

                    let x0 = fx.floor() as usize;
                    let y0 = fy.floor() as usize;
                    let z0 = fz.floor() as usize;
                    let x1 = x0 + 1;
                    let y1 = y0 + 1;
                    let z1 = z0 + 1;

                    let dx = fx - x0 as f32;
                    let dy = fy - y0 as f32;
                    let dz = fz - z0 as f32;

                    macro_rules! G { ($ix:expr, $iy:expr, $iz:expr) => {
                        (
                            grad_x[$ix][$iy][$iz],
                            grad_y[$ix][$iy][$iz],
                            grad_z[$ix][$iy][$iz]
                        )
                    };}

                    let (gx000, gy000, gz000) = G!(x0, y0, z0);
                    let (gx100, gy100, gz100) = G!(x1, y0, z0);
                    let (gx010, gy010, gz010) = G!(x0, y1, z0);
                    let (gx110, gy110, gz110) = G!(x1, y1, z0);
                    let (gx001, gy001, gz001) = G!(x0, y0, z1);
                    let (gx101, gy101, gz101) = G!(x1, y0, z1);
                    let (gx011, gy011, gz011) = G!(x0, y1, z1);
                    let (gx111, gy111, gz111) = G!(x1, y1, z1);

                    let i000 = dot(
                        gx000, gy000, gz000,
                        dx, dy, dz
                    );
                    let i100 = dot(
                        gx100, gy100, gz100,
                        dx - 1.0, dy, dz
                    );
                    let i010 = dot(
                        gx010, gy010, gz010,
                        dx, dy - 1.0, dz
                    );
                    let i110 = dot(
                        gx110, gy110, gz110,
                        dx - 1.0, dy - 1.0, dz
                    );
                    let i001 = dot(
                        gx001, gy001, gz001,
                        dx, dy, dz - 1.0
                    );
                    let i101 = dot(
                        gx101, gy101, gz101,
                        dx - 1.0, dy, dz - 1.0
                    );
                    let i011 = dot(
                        gx011, gy011, gz011,
                        dx, dy - 1.0, dz - 1.0
                    );
                    let i111 = dot(
                        gx111, gy111, gz111,
                        dx - 1.0, dy - 1.0, dz - 1.0
                    );

                    let u = fade(dx);
                    let v = fade(dy);
                    let w = fade(dz);

                    let x00 = lerp(i000, i100, u);
                    let x10 = lerp(i010, i110, u);
                    let x01 = lerp(i001, i101, u);
                    let x11 = lerp(i011, i111, u);

                    let y0v = lerp(x00, x10, v);
                    let y1v = lerp(x01, x11, v);

                    let value = lerp(y0v, y1v, w);

                    slice.set(x, y, value);

                    if value < min_val { min_val = value; }
                    if value > max_val { max_val = value; }
                }
            }

            for ty in 0..y_size_usize {
                for tx in 0..x_size_usize {
                    let v = slice.get(tx, ty);
                    let norm = (v - min_val) / (max_val - min_val);
                    vol.set(tx, ty, z, norm);
                }
            }
        }
        vol
    }
}

// Rendering the algorithms
fn main() {
    let arr_x = 70;
    let arr_y = 35;
    let detail = 10;
    let arr_z = 2000;
    let dewarp = 1.0/2.2;
    // let items: [&str; 5] = [" ", ".", "-", "+", "#"];
    let items: [&str; 5] = [" ", "░", "▒", "▓", "█"];
    let frame_time = Duration::from_millis(33);
    let speed_scale = 0.2;

    let volume = Array3D::perlin_3D(
        detail,
        dewarp,
        speed_scale,
        arr_x,
        arr_y,
        arr_z,
    );
    let mut reverse = false;
    let mut i = 0;
    loop {
        if i >= arr_z-1 {
            // break;
            // i=0;
            reverse=true;
        } else if i <= 0 {
            reverse=false;
        }

        if reverse {i-=1;} else {i+=1;}

        let t_start = Instant::now();

        let array = &volume.data[i as usize];

        let mut output = String::with_capacity(
            (arr_x as usize + 1) * arr_y as usize
        );
        for row in 0..arr_y {
            for col in 0..arr_x {

                let val = (array[row as usize][col as usize] * 4.0).round();

                output.push_str(
                    items[val as usize]
                );
            }
            output.push('\n');
        }

        print!("\x1B[2J\x1B[H{}", output);

        io::stdout().flush().unwrap();

        let elapsed = t_start.elapsed();
        if elapsed < frame_time {
        thread::sleep(frame_time - elapsed);}
    }
}