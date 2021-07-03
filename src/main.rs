use image::io::Reader as ImageReader;
use image::GenericImage;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let size: u32 = args[1].parse().expect("Couldn't parse size");
    let file = &args[2];
    let output_dir = std::path::PathBuf::from(&args[3]);

    let mut img = ImageReader::open(file)
        .unwrap()
        .decode()
        .unwrap()
        .to_rgba8();
    let (width, height) = img.dimensions();

    let mut tiles_set = HashSet::new();
    let mut tiles = Vec::new();

    for tile_x in 0..(width / size) {
        for tile_y in 0..(height / size) {
            let start_x = tile_x * size;
            let start_y = tile_y * size;

            let mut transparent = true;
            'transparency_check: for x in 0..size {
                for y in 0..size {
                    if img.get_pixel(start_x + x, start_y + y).0[3] != 0 {
                        transparent = false;
                        break 'transparency_check;
                    }
                }
            }

            if transparent {
                continue;
            }

            let image = img.sub_image(start_x, start_y, size, size);
            if !tiles_set.insert(image.to_image()) {
                println!("WARN tile x:{}, y:{} was a duplicate", tile_x, tile_y);
            } else {
                tiles.push((image.to_image(), tile_y));
            }
        }
    }

    std::fs::create_dir_all(&output_dir).expect("Couldn't create output dir");

    let mut row_indexes = HashMap::new();
    for (_i, (image, row)) in tiles.into_iter().enumerate() {
        std::fs::create_dir_all(&output_dir.join(format!("{}", row)))
            .expect("Couldn't create output dir for row");

        let index = row_indexes.entry(row).or_insert(0);
        image
            .save(output_dir.join(format!("{}/image{}.png", row, index)))
            .unwrap();
        *index += 1;
    }
}
