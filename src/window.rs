use crate::camera::Camera;
use crate::vector::Vector;
use crate::world::World;
use ggez::{conf, event};

use crate::color::Color;
use ggez::conf::WindowMode;
use ggez::graphics::{self, DrawParam, Drawable};
use ggez::{Context, GameResult};
use image::{ImageBuffer, RgbaImage};
use mint;

pub struct Window {
    width: usize,
    height: usize,
    world: World,
    camera: Camera,
    image_buffer: RgbaImage,
    counter: usize,
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
            counter: 0,
            image_buffer: ImageBuffer::new(width as u32, height as u32),
        }
    }

    pub fn run(&mut self) -> GameResult<()> {
        let width = self.camera.pixel_width();
        let height = self.camera.pixel_height();
        let cb = ggez::ContextBuilder::new("super_simple", "ggez")
            .conf(conf::Conf {
                modules: conf::ModuleConf {
                    audio: false,
                    gamepad: false,
                },
                ..Default::default()
            })
            .window_mode(
                WindowMode::default()
                    .dimensions(width as f32, height as f32)
                    .hidpi(false),
            );
        let (ctx, event_loop) = &mut cb.build()?;
        println!("{}", ggez::graphics::os_hidpi_factor(ctx));
        event::run(ctx, event_loop, self)
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
        println!("first draw");
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
        .draw(ctx, DrawParam::default())?;

        graphics::present(ctx)?;
        self.counter += 1;
        if (self.counter % 10) == 0 {
            println!("FPS: {}", ggez::timer::fps(ctx));
        }
        Ok(())
    }

    fn resize_event(&mut self, _ctx: &mut Context, width: f32, height: f32) {
        println!("resize {} {}", width, height);
    }
}
