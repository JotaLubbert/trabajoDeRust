use image::{open, imageops::FilterType};

pub struct CustomImg{
    pub horizontal: u32,
    pub vertical  : u32,
    pub pixel     : Vec<[u8; 3]>,
}

impl CustomImg {
    /* tanto el x como el y son los indices, (es decir desde 0)
    * la idea es hacer que muestre el array de rgb
    */
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
}

pub fn open_resize_img(img_path:String, x:u32, y:u32) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let img = open(img_path).unwrap();
    let resimg = img.resize(x, y, FilterType::Lanczos3);
    let resimg: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = resimg.into_rgb8();
    resimg
}