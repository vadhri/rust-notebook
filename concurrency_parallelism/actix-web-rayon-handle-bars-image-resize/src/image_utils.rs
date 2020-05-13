use image::DynamicImage;
use image::ImageError;
use image::{imageops};
use std::time::{SystemTime, UNIX_EPOCH};
use std_logger::request;


pub fn read_image_and_resize(source_image: &String, target_width: u32, target_height:u32) -> Result<String, ImageError> {
    let img = image::open(source_image).unwrap();
    let mut template = "test-image/output".to_string();

    let generate_epoch_string_name = |s: &mut String, extn: &str| -> String {
        let epoch_time_nano_sec =  SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();

        s.push_str(&format!("_{:?}_{:?}_{:?}{}",
            &target_width,
            &target_height,
            epoch_time_nano_sec,
            extn));

        request!("generate_epoch_string_name -> {:?}", s);

        s.to_string()
    };

    let subimg = imageops::resize(
        &img,
        target_width,
        target_height,
        image::imageops::FilterType::CatmullRom,
    );

    let output_string = generate_epoch_string_name(&mut template, ".png");

    match subimg.save(&output_string) {
        Ok(_res) => Ok(output_string),
        Err(reason) => {
            Err(reason)
        }
    }
}

pub fn read_img_mem_resize(source_image: &DynamicImage, target_width: u32, target_height:u32) -> Result<String, ImageError> {
    let mut template = "test-image/output".to_string();

    let generate_epoch_string_name = |s: &mut String, extn: &str| -> String {
        let epoch_time_nano_sec =  SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();

        s.push_str(&format!("_{:?}_{:?}_{:?}{}",
            &target_width,
            &target_height,
            epoch_time_nano_sec,
            extn));

        request!("generate_epoch_string_name -> {:?}", s);

        s.to_string()
    };

    request!("Resize image {:?}", (target_width, target_height));
    let subimg = imageops::resize(
        source_image,
        target_width,
        target_height,
        image::imageops::FilterType::CatmullRom,
    );
    request!("Resize image {:?} complete ", (target_width, target_height));

    let output_string = generate_epoch_string_name(&mut template, ".png");

    match subimg.save(&output_string) {
        Ok(_res) => Ok(output_string),
        Err(reason) => {
            Err(reason)
        }
    }
}
