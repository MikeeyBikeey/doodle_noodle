use gdnative::prelude::*;
use image::{EncodableLayout, RgbaImage};
use std::collections::VecDeque;

/// Searches and interprets images for game objects and game aspects.
#[derive(NativeClass, Default)]
#[inherit(Reference)]
pub struct ImageSearch;

#[methods]
impl ImageSearch {
    fn new(_base: &Reference) -> Self {
        ImageSearch
    }

    #[method]
    pub fn find_objects(&self, gd_img: Ref<Image>) -> Option<VariantArray> {
        let gd_img = unsafe { gd_img.assume_safe() };

        // Converts Godot image to a native Rust image
        let mut img = RgbaImage::from_raw(
            gd_img.get_width() as u32,
            gd_img.get_height() as u32,
            Vec::from(&(*gd_img.get_data().read())),
        )
        .unwrap();

        // Finds objects in image
        let mut info = ImageInfo::new(img.width(), img.height());
        let mut objects = Vec::default();

        for y in 0..img.height() {
            for x in 0..img.width() {
                if !info.pixel(x, y).was_checked && !is_background_color(img.get_pixel(x, y).0) {
                    objects.push(ImageSearch::find_entire_object(&img, &mut info, x, y));
                }
            }
        }

        // Updates object images and main image
        for obj in &mut objects {
            obj.take_pixels_from_image(&mut img);
        }

        // Sets original image data
        gd_img.create_from_data(
            img.width() as i64,
            img.height() as i64,
            false,
            Image::FORMAT_RGBA8,
            PoolArray::from_slice(img.as_bytes()),
        );

        // Sets up the return information
        let obj_array = VariantArray::new();
        for obj in &objects {
            obj_array.push(obj.create_dictionary());
        }

        Some(obj_array.into_shared())
    }

    fn find_entire_object(
        img: &RgbaImage,
        info: &mut ImageInfo,
        start_x: u32,
        start_y: u32,
    ) -> ObjectInfo {
        let mut object = ObjectInfo::new(start_x, start_y);
        let mut steps = VecDeque::<(u32, u32)>::default();
        steps.push_back((start_x, start_y));
        while let Some((x, y)) = steps.pop_front() {
            if x >= img.width() || y >= img.height() || info.pixel(x, y).was_checked || is_background_color(img.get_pixel(x, y).0) {
                continue;
            }
            info.pixel_mut(x, y).was_checked = true;

            object.add_pixel_position(x, y);

            // Prepares next steps around the current pixel step
            if x > 0 {
                steps.push_back((x - 1, y));
            }
            if x <= img.width() - 1 {
                steps.push_back((x + 1, y));
            }
            if y > 0 {
                steps.push_back((x, y - 1));
            }
            if y <= img.height() - 1 {
                steps.push_back((x, y + 1));
            }
        }
        object
    }
}

fn is_background_color([r, g, b, _a]: [u8; 4]) -> bool {
    r > 128 || g > 128 || b > 128
}

/// Contains information about an object that is a collection of pixels in the image.
#[derive(Default)]
struct ObjectInfo {
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
    pixel_positions: Vec<(u32, u32)>,
    image: RgbaImage,
}

impl ObjectInfo {
    pub fn new(start_x: u32, start_y: u32) -> Self {
        ObjectInfo {
            left: start_x,
            top: start_y,
            right: start_x,
            bottom: start_y,
            ..Default::default()
        }
    }
}

impl ObjectInfo {
    /// Adds the pixel position and updates the object bounds.
    pub fn add_pixel_position(&mut self, x: u32, y: u32) {
        self.pixel_positions.push((x, y));
        self.left = self.left.min(x);
        self.top = self.top.min(y);
        self.right = self.right.max(x);
        self.bottom = self.bottom.max(y);
    }

    /// Updates the object's `image` and replaces the corresponding pixels on the image to another color.
    pub fn take_pixels_from_image(&mut self, img: &mut RgbaImage) {
        self.image = RgbaImage::new(self.right - self.left + 1, self.bottom - self.top + 1);
        for (x, y) in &self.pixel_positions {
            let (x, y) = (*x, *y);
            self.image
                .put_pixel(x - self.left, y - self.top, *img.get_pixel(x, y));
            img.put_pixel(x, y, image::Rgba([255, 255, 255, 255]));
        }
    }

    pub fn create_dictionary(&self) -> Dictionary {
        let obj = Dictionary::new();
        obj.insert("left", self.left);
        obj.insert("top", self.top);
        obj.insert("right", self.right);
        obj.insert("bottom", self.bottom);

        let image = gdnative::prelude::Image::new();
        image.create_from_data(
            self.image.width() as i64,
            self.image.height() as i64,
            false,
            Image::FORMAT_RGBA8,
            PoolArray::from_slice(self.image.as_bytes()),
        );
        obj.insert("image", image);

        obj.into()
    }
}

/// Contains information about each pixel in the image search.
struct ImageInfo {
    pixels: Vec<PixelInfo>,
    pub width: u32,
    pub height: u32,
}

impl ImageInfo {
    pub fn new(width: u32, height: u32) -> Self {
        ImageInfo {
            pixels: vec![PixelInfo::default(); width as usize * height as usize],
            width,
            height,
        }
    }

    pub fn pixel(&self, x: u32, y: u32) -> &PixelInfo {
        &self.pixels[self.pos_to_idx(x, y)]
    }

    pub fn pixel_mut(&mut self, x: u32, y: u32) -> &mut PixelInfo {
        let idx = self.pos_to_idx(x, y);
        &mut self.pixels[idx]
    }

    fn pos_to_idx(&self, x: u32, y: u32) -> usize {
        (x % self.width + y * self.width) as usize
    }
}

#[derive(Default, Clone)]
struct PixelInfo {
    was_checked: bool,
}
