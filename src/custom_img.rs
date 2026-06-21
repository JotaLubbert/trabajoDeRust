use image::{open, imageops::FilterType};
use crate::custom_img;
use crate::search_types;
use crate::search_types::brillo_cercano;

pub struct CustomImg{
    horizontal : u32,
    vertical   : u32,
    pixel      : Vec<[u8; 3]>,
    acii_pixel : Vec<char>,
}

impl CustomImg {
    /* tanto el x como el y son los indices, (es decir desde 0)
    * la idea es hacer que muestre el array de rgb
    */
    pub fn new(h:u32, v:u32, pix:Vec<[u8;3]>) -> Self{
        Self { horizontal: h, vertical: v, pixel: pix, acii_pixel: vec![]}
    }
    pub fn print_rgb_val_of_a_pixel(&mut self, x:u32, y:u32){
        let ind = y*self.horizontal+x;
        let mut count = 0;
        let mut arr: [u8;3] = [0, 0 ,0];
        for i in &self.pixel{
            if count == ind {
                arr = *i;
                break;
            }
            count += 1;
        }
        println!("{}, {}, {}", arr[0], arr[1], arr[2]);
    }
    fn relative_Brightness(&mut self)->Vec<f64>{
        let mut rel_bright: Vec<f64> = vec![];
        for i in &self.pixel{
            rel_bright.push(0.7 * i[0] as f64 + 0.2 * i[1] as f64 + 0.1 * i[2] as f64);
        }
        rel_bright
    }
    pub fn rgb_to_ascii(&mut self, ascii:Vec<char>, bright_of_char:Vec<f64>){
        let rel_bright = self.relative_Brightness();
        for val in rel_bright.iter() {
            self.acii_pixel.push(ascii[brillo_cercano(&bright_of_char, *val)]);
        }
    }
    pub fn display_ascii_art(caracteres: &[char], ancho: u32) {
        let ancho = ancho as usize;
        let mut salida = String::with_capacity(caracteres.len() + caracteres.len() / ancho);
        for (i, c) in caracteres.iter().enumerate() {
            salida.push(*c);
            if (i + 1) % ancho == 0 {
                salida.push('\n');
            }
        }
        println!("{}", salida);
    }
}



pub fn open_resize_img(img_path:String, x:u32, y:u32) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let img = open(img_path).unwrap();
    let resimg = img.resize(x, y, FilterType::Lanczos3);
    let resimg: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = resimg.into_rgb8();
    resimg
}
