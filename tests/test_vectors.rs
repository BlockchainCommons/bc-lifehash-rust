use serde::Deserialize;

#[derive(Deserialize)]
struct TestVector {
    input: String,
    input_type: String,
    version: String,
    module_size: usize,
    has_alpha: bool,
    width: usize,
    height: usize,
    colors: Vec<u8>,
}

fn parse_version(s: &str) -> bc_lifehash::Version {
    match s {
        "version1" => bc_lifehash::Version::Version1,
        "version2" => bc_lifehash::Version::Version2,
        "detailed" => bc_lifehash::Version::Detailed,
        "fiducial" => bc_lifehash::Version::Fiducial,
        "grayscale_fiducial" => bc_lifehash::Version::GrayscaleFiducial,
        _ => panic!("Unknown version: {s}"),
    }
}

#[test]
fn test_all_vectors() {
    let json_str = include_str!("test-vectors.json");
    let vectors: Vec<TestVector> = serde_json::from_str(json_str).unwrap();

    assert_eq!(vectors.len(), 35, "Expected 35 test vectors");

    for (i, tv) in vectors.iter().enumerate() {
        let version = parse_version(&tv.version);

        let image = if tv.input_type == "hex" {
            if tv.input.is_empty() {
                bc_lifehash::make_from_data(&[], version, tv.module_size, tv.has_alpha)
            } else {
                let data = hex::decode(&tv.input).unwrap();
                bc_lifehash::make_from_data(&data, version, tv.module_size, tv.has_alpha)
            }
        } else {
            bc_lifehash::make_from_utf8(&tv.input, version, tv.module_size, tv.has_alpha)
        };

        assert_eq!(
            image.width, tv.width,
            "Vector {i}: width mismatch for input={:?} version={}", tv.input, tv.version
        );
        assert_eq!(
            image.height, tv.height,
            "Vector {i}: height mismatch for input={:?} version={}", tv.input, tv.version
        );
        assert_eq!(
            image.colors.len(),
            tv.colors.len(),
            "Vector {i}: colors length mismatch for input={:?} version={}", tv.input, tv.version
        );

        if image.colors != tv.colors {
            // Find first mismatch
            for (j, (got, expected)) in image.colors.iter().zip(tv.colors.iter()).enumerate() {
                if got != expected {
                    let pixel = j / if tv.has_alpha { 4 } else { 3 };
                    let component = j % if tv.has_alpha { 4 } else { 3 };
                    let comp_name = ["R", "G", "B", "A"][component];
                    panic!(
                        "Vector {i}: pixel data mismatch for input={:?} version={}\n\
                         First diff at byte {j} (pixel {pixel}, {comp_name}): got {got}, expected {expected}",
                        tv.input, tv.version
                    );
                }
            }
        }
    }
}
