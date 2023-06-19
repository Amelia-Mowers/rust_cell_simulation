use image::{Rgba, RgbaImage, GenericImage};
use gif::{Encoder, Repeat, Frame};
use std::fs::File;
use std::io::Write;

fn create_checkerboard(size: u32, cell_size: u32) -> RgbaImage {
    let mut img = RgbaImage::new(size, size);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let is_even_cell = ((x / cell_size) + (y / cell_size)) % 2 == 0;
        *pixel = if is_even_cell {
            Rgba([255, 255, 255, 255]) // White
        } else {
            Rgba([0, 0, 0, 255]) // Black
        };
    }
    img
}

fn create_animation(size: u16, cell_size: u32, frames: u32) -> Vec<u8> {
    let color_map = &[255, 255, 255, 0, 0, 0]; // White and black
    let mut output = Vec::new();
    {
        let mut encoder = Encoder::new(&mut output, size, size, color_map).unwrap();

        let size32: u32 = size.into();
        
        encoder.set_repeat(Repeat::Infinite).unwrap();

        let mut base_image = create_checkerboard(size32 + cell_size, cell_size);

        for i in 0..frames {
            let x = i % cell_size;
            let y = i % cell_size;
            let mut img = base_image.sub_image(x, y, size32, size32).to_image();

            let mut frame = Frame::from_rgba_speed(size, size, &mut *img, 10);
            frame.delay = 10; // 10/100ths of a second
            encoder.write_frame(&frame).unwrap();
        }
    }
    output
}

fn main() {
    let gif_data = create_animation(256, 32, 32);

    let path = std::path::Path::new("./output/animation.gif");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    let mut file = File::create(path).unwrap();
    file.write_all(&gif_data).unwrap();
}
