use core::f64;

use eframe::egui;
use eframe::egui::{Color32, ColorImage, TextureHandle};
use egui::Vec2;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Astro Generator",
        native_options,
        Box::new(|cc| Ok(Box::new(GeneratorApp::new(cc)))),
    );
}

struct Matr3 {
    values: [[f64; 3]; 3],
}

impl Matr3 {
    fn new(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> Self {
        Self { values: [a, b, c] }
    }

    // fn zero() -> Self {
    //     Self {
    //         values: [[0.0; 3]; 3],
    //     }
    // }
    fn mul(&self, x: f64, y: f64, z: f64) -> (f64, f64, f64) {
        let a = self.values[0];
        let b = self.values[1];
        let c = self.values[2];
        (
            a[0] * x + a[1] * y + a[2] * z,
            b[0] * x + b[1] * y + b[2] * z,
            c[0] * x + c[1] * y + c[2] * z,
        )
    }
}

struct Planet {
    radius: f64,
    angle: f64,
}

impl Planet {
    fn get_color(&self, x: f64, y: f64) -> Option<Color32> {
        if self.inside(x, y) {
            let z = f64::sqrt(1.0 - x * x - y * y);
            let angle = self.angle * f64::consts::PI / 180.0;
            // let rotation_matrix = Matr3::new(
            //     [1.0, 0.0, 0.0],
            //     [0.0, angle.cos(), -angle.sin()],
            //     [0.0, angle.sin(), angle.cos()],
            // );

            let rotation_matrix = Matr3::new(
                [angle.cos(), 0.0, angle.sin()],
                [0.0, 1.0, 0.0],
                [-angle.sin(), 0.0, angle.cos()],
            );

            let (x, y, z) = rotation_matrix.mul(x, y, z);

            let r = (x * 255.0) as u8;
            let g = (y * 255.0) as u8;
            let b = (z * 255.0) as u8;
            let a = 255;
            Some(Color32::from_rgba_unmultiplied(r, g, b, a))
        } else {
            None
        }
    }

    fn inside(&self, x: f64, y: f64) -> bool {
        (x * x + y * y) < self.radius * self.radius
    }
}

struct GeneratorApp {
    texture: Option<TextureHandle>,
    window_size: (usize, usize),
    celestic_object: Planet,
}

impl GeneratorApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            texture: None,
            window_size: (0, 0),
            celestic_object: Planet {
                radius: 0.5,
                angle: 0.0,
            },
        }
    }

    fn set_window_size(&mut self, width: usize, height: usize) {
        self.window_size = (width, height);
    }

    fn update_widow_size(&mut self, size: Vec2) -> bool {
        let width = size.x as usize;
        let height = size.y as usize;

        if self.window_size != (width, height) {
            self.set_window_size(width, height);
            return true;
        }

        return false;
    }

    fn recreate_texture(&mut self, ctx: &egui::Context) {
        // we want texture to take half of the screen
        let texture_width = self.window_size.0 / 2;
        let texture_height = self.window_size.1 / 2;

        let image = self.create_image(texture_width, texture_height, 128);
        if let Some(image) = image {
            let texture = ctx.load_texture(
                "celestial render target",
                image,
                egui::TextureOptions::NEAREST,
            );
            self.texture = Some(texture);
        } else {
            self.texture = None;
        }
    }

    fn create_image(
        &self,
        width: usize,
        height: usize,
        pixel_resolution: usize,
    ) -> Option<ColorImage> {
        let scale_x = (width as f64 / pixel_resolution as f64).floor();
        let scale_y = (height as f64 / pixel_resolution as f64).floor();
        let scale = scale_x.min(scale_y);

        if scale < 1.0 {
            return None;
        }

        let image_size = (scale * pixel_resolution as f64) as usize;

        // let mut image = ColorImage::new(size, vec![Color32::BLACK; 128 * 128]);

        let mut image = ColorImage::filled([image_size, image_size], Color32::BLACK);

        let center = pixel_resolution as f64 / 2.0;
        for y in 0..image_size {
            for x in 0..image_size {
                let dx = (x as f64 / scale - center) / pixel_resolution as f64 * 2.0;
                let dy = (y as f64 / scale - center) / pixel_resolution as f64 * 2.0;

                if let Some(color) = self.celestic_object.get_color(dx, dy) {
                    image[(x, y)] = color;
                }
            }
        }
        Some(image)
    }
}

impl eframe::App for GeneratorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let size = ctx.available_rect().size();

            // if self.update_widow_size(size) {

            // TODO: recreate texture on window update or ui update
            self.update_widow_size(size);
            self.recreate_texture(ctx);
            // }

            ui.horizontal(|ui| {
                if let Some(texture) = &self.texture {
                    ui.image((texture.id(), texture.size_vec2()));
                }
                ui.add(egui::Slider::new(
                    &mut self.celestic_object.angle,
                    0.0..=360.0,
                ))
            });
        });
    }
}
