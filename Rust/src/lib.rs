use std::error::Error;

use godot::prelude::*;

//mod planet;
mod player;
mod satelite;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

use godot::classes::{ISprite2D, Sprite2D, Texture2D};
use imageproc::contours::*;
use imageproc::image::{DynamicImage, ImageBuffer, Rgba};

#[derive(GodotClass)]
#[class(base=Sprite2D, init)]
struct DebugDraw {
    contours: Vec<Contour<i16>>,
    base: Base<Sprite2D>,
}

fn texture2d_to_dynamic_image(texture: Gd<Texture2D>) -> Result<DynamicImage, String> {
    // Get Image from Texture
    let mut image = texture.get_image().ok_or("Texture has no image data")?;

    // Ensure correct format (RGBA8)
    if image.get_format() != godot::classes::image::Format::RGBA8 {
        image.convert(godot::classes::image::Format::RGBA8);
    }

    // Get dimensions
    let width = image.get_width();
    let height = image.get_height();

    // Extract raw pixel data
    let data: PackedByteArray = image.get_data();
    let raw: Vec<u8> = data.to_vec();

    // Create ImageBuffer
    let buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width as u32, height as u32, raw)
            .ok_or("Failed to create ImageBuffer")?;

    // Wrap into DynamicImage
    Ok(DynamicImage::ImageRgba8(buffer))
}

fn load_image_contours(
    texture: Gd<Texture2D>,
) -> Result<Vec<Contour<i16>>, Box<dyn Error + 'static>> {
    // Open the image
    let img = texture2d_to_dynamic_image(texture)?;

    // Conversions
    let grey_img = img.grayscale();
    let luma8_img = grey_img.to_luma8();

    // Collect contours
    let contours: Vec<Contour<i16>> = find_contours(&luma8_img);
    // Strip contours to only the outer
    /*let outer_contours: Vec<Contour<i16>> = contours
    .iter()
    .filter(|c| c.border_type == BorderType::Outer)
    .cloned()
    .collect();*/

    Ok(contours)
}

fn contour_to_points(contour: &Contour<impl Into<f32> + Copy>) -> PackedVector2Array {
    contour
        .points
        .iter()
        .map(|p| Vector2::new(p.x.into(), p.y.into()))
        .collect()
}

#[godot_api]
impl ISprite2D for DebugDraw {
    fn draw(&mut self) {
        let contours: Vec<PackedVector2Array> =
            self.contours.iter().map(|c| contour_to_points(c)).collect();
        for p in contours {
            self.base_mut().draw_polyline(&p, Color::RED);
        }
    }
}

#[godot_api]
impl DebugDraw {
    #[func]
    fn print_image_contours(&self) {
        godot_print!("{:?}", self.contours);
    }

    #[func]
    fn load_image_contours(&mut self, tex: Gd<Texture2D>) {
        self.contours = load_image_contours(tex).unwrap()
    }
}
