use std::{
    any::{type_name, Any, TypeId},
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use rgine_assets::AssetsEventQueueExt;
use rgine_logger::warn;
use rgine_modules::events::EventQueue;

pub trait FileAssetsRegistry: 'static {
    type Handle;
    type Data;
    fn new() -> Self;
    fn read_file(path: &Path) -> Self::Data;
    fn register(&mut self, data: Self::Data) -> Self::Handle;
    fn file_extensions() -> &'static [&'static str];
}

pub struct AssetLookup {
    map: HashMap<TypeId, Box<dyn Any>>,
}

impl AssetLookup {
    fn add_assets<R: FileAssetsRegistry>(&mut self, map: HashMap<String, R::Handle>) {
        self.map.insert(TypeId::of::<R>(), Box::new(map));
    }

    fn get<R: FileAssetsRegistry>(&self, key: &str) -> &R::Handle {
        self.map
            .get(&TypeId::of::<R>())
            .expect("Tried to access asset type that has not been loaded!")
            .downcast_ref::<HashMap<String, R::Handle>>()
            .unwrap()
            .get(key)
            .unwrap_or_else(|| {
                panic!(
                    "Requested asset {} of regtistry type {} doesn't exist!",
                    key,
                    type_name::<R>()
                )
            })
    }
}

pub trait FileAssetsEventQueueExt {
    fn load_asset_registry_from_disk<R: FileAssetsRegistry>(
        &mut self,
        path: &str,
        lookup: &mut AssetLookup,
    );
}
impl FileAssetsEventQueueExt for EventQueue {
    fn load_asset_registry_from_disk<R: FileAssetsRegistry>(
        &mut self,
        subpath: &str,
        lookup: &mut AssetLookup,
    ) {
        let mut registry = R::new();
        let mut lookup_map = HashMap::new();
        for file in std::fs::read_dir(assets_dir(subpath))
            .unwrap_or_else(|_| panic!("Could not read assets at {}", assets_dir(subpath)))
        {
            let file = file.unwrap();
            let metadata = file.metadata().unwrap();
            if !metadata.is_file()
                || R::file_extensions()
                    .iter()
                    .all(|ext| file.path().extension() != Some(OsStr::new(ext)))
            {
                return;
            }

            let name = skip_last(file.file_name().to_string_lossy().split("."))
                .fold(String::new(), |a, b| a + b);

            if !is_snake_case(&name) {
                warn!(
                    "Invalid asset name \"{}\" for sprite: {}. assets_name_must_be_written_in_snake_case ! ",
                    name,
                    file.path().to_string_lossy()
                );
                continue;
            }

            let path = file.path().to_string_lossy().into_owned();
            let data = R::read_file(path.as_ref());

            lookup_map.insert(name, registry.register(data));
        }

        lookup.add_assets::<R>(lookup_map);
        self.load_asset(registry);
    }
}

pub fn assets_dir(subpath: &str) -> String {
    format!("assets/{subpath}")
}

fn skip_last<T>(mut iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    let last = iter.next();
    iter.scan(last, |state, item| std::mem::replace(state, Some(item)))
}

fn is_snake_case(name: &str) -> bool {
    name.chars()
        .fold(true, |b, c| b && (c.is_ascii_alphanumeric() || c == '_'))
}
