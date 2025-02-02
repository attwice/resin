use std::{
    fs::{copy, read_dir, read_to_string},
    path::Path,
    process::Command,
};

use crate::metadata::NFTMetadata;

pub fn generate(_config_location: &String, assets_directory: &String, output_directory: &String) {
    println!("Generating artwork from metadata...");
    read_metadata(assets_directory, output_directory);
}

fn read_metadata(assets_directory: &String, output_directory: &String) {
    let files = read_dir(output_directory).expect("Could not read assets directory");
    for file_raw in files {
        let file = file_raw.expect("Could not read file");
        match file.path().extension() {
            Some(extension) => {
                if extension != "json" {
                    continue;
                }
            }
            None => continue,
        }

        let contents = read_to_string(file.path()).expect(&format!(
            "Could not read file contents for file {}",
            file.path().display()
        ));
        let parsed_metadata: NFTMetadata = serde_json::from_str(&contents).expect(&format!(
            "Could not parse metadata JSON for file {}",
            file.path().display()
        ));
        let file_name = file.file_name();
        let id = file_name
            .to_str()
            .unwrap()
            .split('.')
            .next()
            .expect(&format!(
                "Could not get ID from file name for file {}",
                file.path().display()
            ));

        create_image(id, &parsed_metadata, &assets_directory, &output_directory);
    }
}

fn create_image(id: &str, metadata: &NFTMetadata, assets_directory: &str, output_directory: &str) {
    let image_path_buffer = Path::new(output_directory).join(format!("{}.png", id));
    let image_path = image_path_buffer.to_str().expect(&format!(
        "Image is not valid path at {}",
        image_path_buffer.display()
    ));

    copy(
        Path::new(assets_directory)
            .join(metadata.attributes[0].trait_type.clone())
            .join(format!("{}.png", &metadata.attributes[0].value)),
        image_path,
    )
    .expect(&format!("Could not copy base layer for image {}", id));

    for attribute in &metadata.attributes[1..] {
        let layer_path_buffer = Path::new(assets_directory)
            .join(attribute.trait_type.clone())
            .join(format!("{}.png", &attribute.value));
        let layer_path = layer_path_buffer.to_str().expect(&format!(
            "Layer is not valid path at {}",
            layer_path_buffer.display()
        ));

        Command::new("composite")
            .arg(layer_path)
            .arg(image_path)
            .arg(image_path)
            .output()
            .expect(&format!("Error creating image {}", id));
    }
}
