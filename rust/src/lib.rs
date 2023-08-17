use gdnative::prelude::*;

pub mod image_search;
pub use image_search::ImageSearch;

fn init(handle: InitHandle) {
    handle.add_class::<ImageSearch>();
}

godot_init!(init);
