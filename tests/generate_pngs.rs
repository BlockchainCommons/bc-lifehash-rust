use std::{fs, path::Path};

#[test]
fn generate_pngs() {
    let versions = [
        ("version1", bc_lifehash::Version::Version1),
        ("version2", bc_lifehash::Version::Version2),
        ("detailed", bc_lifehash::Version::Detailed),
        ("fiducial", bc_lifehash::Version::Fiducial),
        (
            "grayscale_fiducial",
            bc_lifehash::Version::GrayscaleFiducial,
        ),
    ];

    let out_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("out");

    for (name, version) in &versions {
        let dir = out_dir.join(name);
        fs::create_dir_all(&dir).unwrap();

        for i in 0..100 {
            let input = i.to_string();
            let image = bc_lifehash::make_from_utf8(&input, *version, 1, false);

            let path = dir.join(format!("{i}.png"));
            let file = fs::File::create(&path).unwrap();
            let w = std::io::BufWriter::new(file);

            let mut encoder =
                png::Encoder::new(w, image.width as u32, image.height as u32);
            encoder.set_color(png::ColorType::Rgb);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&image.colors).unwrap();
        }
    }
}
