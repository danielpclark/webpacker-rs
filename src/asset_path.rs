use super::Manifest;

pub struct AssetPath {
    dir: String,
    file: String, 
}

impl AssetPath {
    pub fn new<S>(dir: S, file_key: S, manifest: Manifest) -> Self 
    where S: Into<String> {
        let file = file_key.into();
        AssetPath {
            dir: dir.into(),
            file: manifest.get(&file)
                .expect(&format!("File key '{}' not found in file manifest!", &file))
                .to_owned()
        }
    }
}

impl<'a> From<AssetPath> for String {
    fn from(t: AssetPath) -> String {
         format!("{}{}", t.dir.trim_end_matches('/'), t.file)
    }
}

#[test]
fn test_asset_path() {
    let mut manifest = Manifest::new();
    manifest.insert("asdf".into(), "/qwerty".into());
    let file = AssetPath::new("public", "asdf", manifest);
    assert_eq!(&String::from(file), "public/qwerty");
}
