use std::path::Path;

use qrcode_generator::QrCodeEcc;
use image::{DynamicImage, ImageBuffer, open, Rgb};

const CARGO_DIR : &str = concat!(env!("CARGO_MANIFEST_DIR"), "/");

pub type ImageBuff8 = ImageBuffer<Rgb<u8>, Vec<u8>>;

fn get_block(bits: [bool; 4], block_table: &[DynamicImage; 4]) -> DynamicImage {
    let (mode, rotation) = {
        let mut mode = 0;
        let mut rotation = 0;
        for i in 0..4{
            if bits[i] && mode == 0 {
                mode = 1;
                rotation = i;
            } else if bits[i] && mode == 1 && (bits[(i - 1)] || bits[(i + 1) % 4]) {
                mode = 2;
                rotation = if i == 3 && bits[0] { 3 } else { i - 1 };
            } else if bits[i] {
                mode = 3;
                rotation = 0;
            }
        }
        (mode, rotation)
    };

    let image = match rotation {
        1 => block_table[mode].rotate90(),
        2 => block_table[mode].rotate180(),
        3 => block_table[mode].rotate270(),
        _ => block_table[mode].clone(),
    };

    image
}

fn copy_image_into_another(dest: &mut ImageBuff8, src: &ImageBuff8, x: u32, y: u32) {
   if y + src.height() > dest.height() || x + src.width() > dest.width() {
       panic!("dest overflow in copy_image_into_another");
   }
   for i in 0..src.height() {
       for j in 0..src.width() {
            dest.put_pixel(j + x, i + y, src.get_pixel(j, i).clone());
       }
   }
}

pub fn qrcode_matrix(message: &str) -> Vec<Vec<bool>> {
    qrcode_generator::to_matrix(message, QrCodeEcc::High).expect("Unable to create QR Code for this message")
}

pub fn qrcode_generate(message: &str) -> ImageBuff8 {
    let qrcode_matrix = qrcode_matrix(message);
    let mut qrcode_image = ImageBuff8::from_fn(qrcode_matrix[0].len() as u32 * 35, qrcode_matrix.len() as u32 * 35, |x: u32, y: u32| { Rgb{ data: [255, 255, 255] }});

    let eye = open(Path::new(&format!("{}assets/eye.png", CARGO_DIR))).unwrap();
    let image_center = open(Path::new(&format!("{}assets/image_center.png", CARGO_DIR))).unwrap();
    let block_table: [DynamicImage; 4] = [
        open(Path::new(&format!("{}assets/0.png", CARGO_DIR))).unwrap(),
        open(Path::new(&format!("{}assets/1.png", CARGO_DIR))).unwrap(),
        open(Path::new(&format!("{}assets/2.png", CARGO_DIR))).unwrap(),
        open(Path::new(&format!("{}assets/3.png", CARGO_DIR))).unwrap(),
    ];

    for line_id in 0..qrcode_matrix.len() {
        for block_id in 0..qrcode_matrix[line_id].len() {
            if (line_id < 7 && block_id < 7)
                || (line_id >= qrcode_matrix.len() - 7 && block_id < 7)
                || (line_id < 7 && block_id >= qrcode_matrix[line_id].len() - 7)
                || (line_id <= (qrcode_matrix.len() / 2) + 4 && line_id >= (qrcode_matrix.len() / 2) - 4
                    && block_id <= (qrcode_matrix[line_id].len() / 2) + 4 && block_id >= (qrcode_matrix[line_id].len() / 2) - 4) {
                continue;
            }
            if qrcode_matrix[line_id][block_id] {
                let bits = [
                    line_id != 0 && qrcode_matrix[line_id - 1][block_id],
                    block_id != qrcode_matrix[line_id].len() - 1 && qrcode_matrix[line_id][block_id + 1],
                    line_id != qrcode_matrix.len() - 1 && qrcode_matrix[line_id + 1][block_id],
                    block_id != 0 && qrcode_matrix[line_id][block_id - 1],
                ];
                copy_image_into_another(&mut qrcode_image, &get_block(bits, &block_table).to_rgb(), block_id as u32 * 35, line_id as u32 * 35);
            }
        }
    }

    copy_image_into_another(&mut qrcode_image, &eye.to_rgb(), 0, 0);
    copy_image_into_another(&mut qrcode_image, &eye.rotate270().to_rgb(), 0, (qrcode_matrix.len() as u32 - 7) * 35);
    copy_image_into_another(&mut qrcode_image, &eye.rotate90().to_rgb(), (qrcode_matrix[0].len() as u32 - 7) * 35, 0);
    copy_image_into_another(&mut qrcode_image, &image_center.to_rgb(), (qrcode_matrix.len() as u32 / 2 - 4) * 35, (qrcode_matrix[0].len() as u32 / 2 - 4) * 35);

    qrcode_image
}
