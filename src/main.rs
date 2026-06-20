mod custom_img;
use crate::custom_img::CustomImg;


fn main() {
    let img_path: String = String::from("img.jpg");
    let x = 100; let y = 100;
    let images: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = custom_img::open_resize_img(img_path, x, y);
    let mut pix:Vec<[u8;3]> = vec![];
    for i in images.pixels() {
        let arr = [i[0], i[1], i[2]];
        pix.push(arr);
    }
    let cus_img = CustomImg{horizontal: x, vertical: y, pixel: pix};

    for i in cus_img.pixel {
        print!("({}, {}, {}), ", i[0], i[1], i[2]);
    }
}
