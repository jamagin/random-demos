use nannou::{
    image::{GenericImage, Rgba},
    prelude::*,
};

struct Model {
    texture: wgpu::Texture,
}

fn main() {
    nannou::app(model).run();
}

const HEIGHT: u32 = 512;
const WIDTH: u32 = 257 * 2;

fn model(app: &App) -> Model {
    // Create a new window!
    app.new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();
    let mut image = nannou::image::DynamicImage::new_rgb8(WIDTH, HEIGHT);
    let mut state = 0x45u16;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let val = lfsr(&mut state) * 255;
            image.put_pixel(x, y, Rgba::<u8>([val, val, val, 255]));
        }
    }
    let texture = wgpu::Texture::from_image(app, &image);
    Model { texture }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let draw = app.draw();
    draw.texture(&model.texture);

    draw.to_frame(app, &frame).unwrap();
}

fn lfsr(state: &mut u16) -> u8 {
    let out = *state >> 15;
    // maximal sequence for a 16 bit LFSR, per wikipedia
    // period = 65535 = 3 * 5 * 17 * 257
    *state = (*state << 1) | (*state & 0b1101000000001000u16).count_ones() as u16 % 2;
    out as u8
}
