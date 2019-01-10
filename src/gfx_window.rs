use crate::camera::Camera;
use crate::color::Color;
use crate::vector::Vector;
use crate::world::World;

use gfx::format::{DepthStencil, Formatted, Srgba8};
use gfx::memory::Typed;
use gfx::traits::*;
use gfx_graphics::{Gfx2d, Texture, TextureSettings};
use glutin_window::{GlutinWindow, OpenGL};
use graphics;
use graphics::draw_state::Blend;
use image::{ImageBuffer, RgbaImage};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::*;
use piston::window::{OpenGLWindow, Window as PistonWindow, WindowSettings};
// use piston_window;
// use piston_window::{G2dTexture, PistonWindow, Texture, TextureSettings, WindowSettings};
// use piston_window::G2dTexture;

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
        let mut window: GlutinWindow =
            WindowSettings::new("example", (self.width as u32, self.height as u32))
                .exit_on_esc(true)
                .opengl(opengl)
                .build()
                .unwrap();

        let (mut device, mut factory) =
            gfx_device_gl::create(|s| window.get_proc_address(s) as *const std::os::raw::c_void);

        // Create the main color/depth targets.
        let aa = 4 as gfx::texture::NumSamples;
        let dim = (self.width as u16, self.height as u16, 1, aa.into());
        let color_format = <Srgba8 as Formatted>::get_format();
        let depth_format = <DepthStencil as Formatted>::get_format();
        let (output_color, output_stencil) =
            gfx_device_gl::create_main_targets_raw(dim, color_format.0, depth_format.0);
        let output_color = Typed::new(output_color);
        let output_stencil = Typed::new(output_stencil);

        let blends = [Blend::Alpha, Blend::Add, Blend::Invert, Blend::Multiply];
        let mut blend = 0;
        let mut clip_inside = true;

        let mut encoder = factory.create_command_buffer().into();
        let mut g2d = Gfx2d::new(opengl, &mut factory);
        let mut events = Events::new(EventSettings::new().lazy(true));

        let mut texture: G2dTexture =
            Texture::from_image(&mut factory, &self.image_buffer, &TextureSettings::new()).unwrap();

        while let Some(e) = events.next(&mut window) {
            if let Some(args) = e.render_args() {
                self.tick();
                self.generate_image(self.camera.render(&self.world).as_ref());

                // texture.update(&mut encoder, &self.image_buffer).unwrap();

                g2d.draw(
                    &mut encoder,
                    &output_color,
                    &output_stencil,
                    args.viewport(),
                    |c, g| {
                        graphics::Image::new().draw(
                            &texture,
                            &graphics::DrawState::new_inside(),
                            c.transform,
                            g,
                        );
                    },
                );
                // window.draw_2d(&e, |c, g| {
                //     piston_window::image(&texture, c.transform, g);
                // });
                // g2d.draw(
                //     &mut encoder,
                //     &output_color,
                //     &output_stencil,
                //     args.viewport(),
                //     |c, g| {},
                // );
                encoder.flush(&mut device);
            }

            if let Some(_) = e.after_render_args() {
                device.cleanup();
            }
        }
        // while let Some(e) = window.next() {
        //     self.tick();
        //     self.generate_image(self.camera.render(&self.world).as_ref());
        //
        //     texture
        //         .update(&mut window.encoder, &self.image_buffer)
        //         .unwrap();
        //     // window.draw_2d(&e, |c, g| {
        //     //     // piston_window::clear([1.0; 4], g);
        //     //     piston_window::image(&texture, c.transform, g);
        //     // });
        // }
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
