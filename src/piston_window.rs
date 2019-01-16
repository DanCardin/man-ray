use crate::camera::Camera;
use crate::vector::Vector;
use crate::world::World;

use crate::color::Color;
use image::{ImageBuffer, RgbaImage};
use piston_window;
use piston_window::{G2dTexture, OpenGL, PistonWindow, Texture, TextureSettings, WindowSettings};

pub struct Window {
    width: usize,
    height: usize,
    world: World,
    camera: Camera,
    image_buffer: RgbaImage,
    origin: Vector,
}

impl Window {
    pub fn new(world: World, camera: Camera) -> Window {
        let width = camera.pixel_width();
        let height = camera.pixel_height();

        Window {
            width,
            height,
            world,
            camera,
            origin: Vector::new(8.0, 2.0, 10.0),
            image_buffer: ImageBuffer::new(width as u32, height as u32),
        }
    }

    pub fn run(&mut self) {
        let opengl = OpenGL::V4_5;
        let mut window: PistonWindow =
            WindowSettings::new("example", (self.width as u32, self.height as u32))
                .exit_on_esc(true)
                .opengl(opengl)
                .build()
                .unwrap();

        let mut texture: G2dTexture = Texture::from_image(
            &mut window.factory,
            &self.image_buffer,
            &TextureSettings::new(),
        )
        .unwrap();

        while let Some(e) = window.next() {
            self.tick();
            self.generate_image(self.camera.render(&self.world).as_ref());

            texture
                .update(&mut window.encoder, &self.image_buffer)
                .unwrap();
            window.draw_2d(&e, |c, g| {
                // piston_window::clear([1.0; 4], g);
                piston_window::image(&texture, c.transform, g);
            });
        }
    }

    fn tick(&mut self) {
        self.origin.z -= 0.1;
        self.camera.set_origin(self.origin);
    }

    fn generate_image(&mut self, pixels: &[Color]) {
        for (y, row) in pixels.chunks(self.width).enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                self.image_buffer
                    .put_pixel(x as u32, y as u32, pixel.into());
            }
        }
    }
}
