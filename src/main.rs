use image::{open, imageops::FilterType};
fn open_resize_img(img_path:String, x:u32, y:u32) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let img = open(img_path).unwrap();
    let resimg = img.resize(x, y, FilterType::Lanczos3);
    let resimg: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = resimg.into_rgb8();
    resimg
}



fn main() {
    let img_path: String = String::from("img.jpg");
    let images: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = open_resize_img(img_path, 100, 100);
    for i in images.pixels() {
        print!("({}, {}, {})", i[0], i[1], i[2]);
    }
}
