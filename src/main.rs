use image::*;

fn main() {
    let img = image::open("image.png").expect("err 1");

    let img = img.resize(200, 400, image::imageops::FilterType::Nearest);

    let chars = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);

            let r = pixel[0] as u32;
            let g = pixel[1] as u32;
            let b = pixel[2] as u32;

            let brightness = (r + g + b) / 3;
            let index = (brightness * 9 / 255) as usize;

            print!("{}", chars[index]);
        }
        println!();
    }
}
