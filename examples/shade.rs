extern crate palette;
extern crate image;

use palette::{Rgb, Lab, Hsv, Shade};

use image::{RgbImage, GenericImage};

fn main() {
    //The same color in linear RGB, CIE L*a*b*, and HSV
    let rgb = Rgb::rgb(0.5, 0.0, 0.0);
    let lab = Lab::from(rgb.clone());
    let hsv = Hsv::from(rgb.clone());

    let mut image = RgbImage::new(220, 193);

    for i in 0..11 {
        let (rgb_r1, rgb_g1, rgb_b1, _) = rgb.darken(0.05 * i as f32).to_srgba8();
        let (rgb_r2, rgb_g2, rgb_b2, _) = rgb.lighten(0.05 * i as f32).to_srgba8();

        for (_, _, pixel) in image.sub_image(i as u32 * 20, 0, 20, 31).pixels_mut() {
            pixel.data = [rgb_r1, rgb_g1, rgb_b1];
        }

        for (_, _, pixel) in image.sub_image(i as u32 * 20, 32, 20, 31).pixels_mut() {
            pixel.data = [rgb_r2, rgb_g2, rgb_b2];
        }

        let (lab_r1, lab_g1, lab_b1, _) = Rgb::from(lab.darken(0.05 * i as f32)).to_srgba8();
        let (lab_r2, lab_g2, lab_b2, _) = Rgb::from(lab.lighten(0.05 * i as f32)).to_srgba8();

        for (_, _, pixel) in image.sub_image(i as u32 * 20, 65, 20, 31).pixels_mut() {
            pixel.data = [lab_r1, lab_g1, lab_b1];
        }

        for (_, _, pixel) in image.sub_image(i as u32 * 20, 97, 20, 31).pixels_mut() {
            pixel.data = [lab_r2, lab_g2, lab_b2];
        }

        let (hsv_r1, hsv_g1, hsv_b1, _) = Rgb::from(hsv.darken(0.05 * i as f32)).to_srgba8();
        let (hsv_r2, hsv_g2, hsv_b2, _) = Rgb::from(hsv.lighten(0.05 * i as f32)).to_srgba8();

        for (_, _, pixel) in image.sub_image(i as u32 * 20, 130, 20, 31).pixels_mut() {
            pixel.data = [hsv_r1, hsv_g1, hsv_b1];
        }

        for (_, _, pixel) in image.sub_image(i as u32 * 20, 162, 20, 31).pixels_mut() {
            pixel.data = [hsv_r2, hsv_g2, hsv_b2];
        }
    }

    match image.save("examples/shade.png") {
        Ok(()) => println!("see 'examples/shade.png' for the result"),
        Err(e) => println!("failed to write 'examples/shade.png': {}", e),
    }
}