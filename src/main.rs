mod custom_img;
mod search_types;
mod read_file;
use crate::custom_img::CustomImg;
use crate::read_file::listed_file_content;
use std::io::{self, Write};
use std::{env, fs::read_to_string};

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    input.trim().to_string()
}
 
fn read_dimension(prompt: &str) -> u32 {
    loop {
        let input = read_line(prompt);
        match input.parse::<u32>() {
            Ok(n) if n > 0 => return n,
            _ => println!("Por favor ingresa un número entero mayor a 0."),
        }
    }
}
 
fn get_user_input() -> (String, u32, u32) {
    let img_path = read_line("Ruta de la imagen: ");
    let x = read_dimension("Ancho de salida: ");
    let y = read_dimension("Alto de salida: ");
    (img_path, x, y)
}

fn main() {
    let char_bright = "../resources/CharBright.txt";
    let (ascii, aparent_bright) = listed_file_content(char_bright);
    let (img_path, x, y) = get_user_input();
    let images: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = custom_img::open_resize_img(img_path, x, y);
    let mut pix:Vec<[u8;3]> = vec![];
    for i in images.pixels() {
        let arr: [u8; 3] = [i[0], i[1], i[2]];
        pix.push(arr);
    }
    let mut cus_img = CustomImg{horizontal: x, vertical: y, pixel: pix};
    cus_img.print_rgb_val_of_a_pixel(35, 75);
    cus_img.print_rgb_val_of_a_pixel(35, 75);
}
