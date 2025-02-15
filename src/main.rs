
fn main() {
    println!("Hello, world!");
}


#[test]
#[allow(unused_must_use)]
fn test_file_to_file() {
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let readme = manifest.join("README.md");
    let dest = manifest.join("target").join("test1");

    // clear previous iteration
    std::fs::remove_dir_all(&dest);

    // create structure
    std::fs::create_dir_all(&dest).unwrap();


    std::fs::copy(&readme, &dest.join("README.md"))
        .expect(format!("Failed to copy from {:?} to {:?}", &readme, &dest).as_str());
}

#[test]
#[allow(unused_must_use)]
fn test_file_to_folder() {
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let readme = manifest.join("README.md");
    let dest = manifest.join("target").join("test2");

    // clear previous iteration
    std::fs::remove_dir_all(&dest);

    // create structure
    std::fs::create_dir_all(&dest).unwrap();


    std::fs::copy(&readme, &dest)
        .expect(format!("Failed to copy from {:?} to {:?}", &readme, &dest).as_str());
}

#[test]
#[allow(unused_must_use)]
fn test_file_to_folder_with_backslash() {
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let readme = manifest.join("README.md");
    let mut dest = manifest.join("target").join("test2").to_str().unwrap().to_string();
    dest.push(std::path::MAIN_SEPARATOR);

    println!("dest: {:?}",dest);


    // clear previous iteration
    std::fs::remove_dir_all(&dest);

    // create structure
    std::fs::create_dir_all(&dest).unwrap();


    std::fs::copy(&readme, &dest)
        .expect(format!("Failed to copy from {:?} to {:?}", &readme, &dest).as_str());
}