use image::{open, imageops::FilterType};

pub struct CustomImg{
    pub horizontal: u32,
    pub vertical  : u32,
    pub pixel     : Vec<[u8; 3]>,
}

impl CustomImg {
    
}

pub fn open_resize_img(img_path:String, x:u32, y:u32) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let img = open(img_path).unwrap();
    let resimg = img.resize(x, y, FilterType::Lanczos3);
    let resimg: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = resimg.into_rgb8();
    resimg
}