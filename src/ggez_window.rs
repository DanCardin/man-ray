use crate::camera::Camera;
use crate::vector::Vector;
use crate::world::World;
use ggez::event;

use crate::color::Color;
use ggez::conf::WindowMode;
use ggez::graphics::{self, Drawable, Point2};
use ggez::{Context, GameResult};
use image::{ImageBuffer, RgbaImage};

pub struct Window {
    width: usize,
    height: usize,
    world: World,
    camera: Camera,
    image_buffer: RgbaImage,
    counter: usize,
}

impl Window {
    fn new(
        _ctx: &mut Context,
        world: World,
        camera: Camera,
        width: usize,
        height: usize,
    ) -> GameResult<Window> {
        let s = Window {
            width,
            height,
            world,
            camera,
            counter: 0,
            image_buffer: ImageBuffer::new(width as u32, height as u32),
        };
        Ok(s)
    }

    pub fn run(world: World, camera: Camera) -> GameResult<()> {
        let width = camera.pixel_width();
        let height = camera.pixel_height();
        let cb = ggez::ContextBuilder::new("super_simple", "ggez")
            .window_mode(WindowMode::default().dimensions(width as u32, height as u32));
        let ctx = &mut cb.build()?;
        let state = &mut Window::new(ctx, world, camera, width, height)?;
        event::run(ctx, state)
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

impl event::EventHandler for Window {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let mut origin = Vector::new(8.0, 2.0, 10.0);
        let offset = self.counter as f64 * 0.2;
        origin.z -= offset;

        self.camera.set_origin(origin);
        self.generate_image(self.camera.render(&self.world).as_ref());
        graphics::Image::from_rgba8(
            ctx,
            self.width as u16,
            self.height as u16,
            self.image_buffer.as_ref(),
        )?
        .draw(ctx, Point2::new(0.0, 0.0), 0.0)?;

        graphics::present(ctx);
        self.counter += 1;
        Ok(())
    }
}
