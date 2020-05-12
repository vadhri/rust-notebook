use image::{imageops, GenericImageView};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn read_image_and_create_two_variants() {
    let mut img = image::open("test-image/map.png").unwrap();
    let (orig_w, orig_h) = img.dimensions();

    let generate_epoch_string_name = |s: String, extn: String| -> String {
        let mut path_with_prefix: String = s.to_owned();
        let epoch_time_nano_sec: String =  SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().to_string();

        path_with_prefix.push_str(&epoch_time_nano_sec);
        path_with_prefix.push_str(&extn);

        path_with_prefix
    };

    let subimg = imageops::resize(
        &mut img,
        orig_w / 2,
        orig_h / 2,
        image::imageops::FilterType::CatmullRom,
    );

    match subimg.save(generate_epoch_string_name("test-image/output_half_".to_string(), ".png".to_string())) {
        Ok(_res) => {

        },
        Err(_reason) => {
            println!("Save failed  -> {:?}", _reason);
        }
    };

    let subimg = imageops::resize(
        &mut img,
        100,
        (orig_h as f32 * (100 as f32 / orig_w as f32)) as u32,
        image::imageops::FilterType::CatmullRom,
    );

    match subimg.save(generate_epoch_string_name("test-image/output_thumbnail_".to_string(), ".png".to_string())) {
        Ok(_res) => {

        },
        Err(_reason) => {
            println!("Save failed  -> {:?}", _reason);
        }
    };
}
