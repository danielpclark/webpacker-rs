use gotham::handler::assets::FileOptions;
use super::asset_path::*;

impl<'a> From<AssetPath> for FileOptions {
    fn from(t: AssetPath) -> FileOptions {
        FileOptions::new(String::from(t))
    }
}

#[test]
fn test_file_options_from_asset_path() {
    use super::Manifest;
    let mut manifest = Manifest::new();
    manifest.insert("asdf".into(), "/qwerty".into());
    let file = AssetPath::new("public", "asdf", &manifest);
    FileOptions::from(file);
    assert!(true); // made it this far
}

