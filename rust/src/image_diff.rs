extern crate image;

use image_diff::image::{DynamicImage, GenericImage, ImageBuffer, Rgba, Pixel};
use std::path::Path;
use std::cmp::max;

pub struct ImageDiff {}

impl ImageDiff {
  pub fn compare(base_path: String, new_path: String, threshold: f32) {
    assert!(threshold <= 1. && threshold >= 0.);

    let base_image = get_image_from_path(&base_path);
    let new_image = get_image_from_path(&new_path);
    
    calculate_diff(base_image, new_image, threshold);
  }
}

// TODO: reshape images so sizes will be the same
fn calculate_diff(base_image: DynamicImage, new_image: DynamicImage, threshold: f32) {
  let (width, height) = get_dimensions(&base_image, &new_image);
  let mut image = get_empty_image(width, height);

  for w in 0..width-1 {
    for h in 0..height-1 {
      let pixel = base_image.get_pixel(w, h);
      let pixel2 = new_image.get_pixel(w, h);
      
      let is_equal = is_equal(pixel2.data, pixel.data, threshold);

      let red_pixel =  Pixel::from_channels(255, 0, 0, 255);


      if !is_equal {
        image.put_pixel(w, h, red_pixel);
      }
    }
  }
  let path = "output.png";
  save(image, path);
}

fn is_equal(a: [u8; 4], b: [u8; 4], threshold: f32) -> bool {
  let threshold_value = (threshold * 255.) as u8;

  for idx in 0..4 {
    let diff = get_diff(a[idx], b[idx]);
    if diff > threshold_value {
      return false
    }
  }
  true
}

fn get_diff(a: u8, b: u8) -> u8 {
  let larger = max(a, b);
  let smaller = if larger == a { b } else { a };
  larger - smaller
}

fn get_dimensions(base_image: &DynamicImage, new_image: &DynamicImage) -> (u32, u32) {
  // Assume both the same for now
  new_image.dimensions()
  // let width = max(base_image.dimensions().0, new_image.dimensions().0);
  // let height = max(base_image.dimensions().1, new_image.dimensions().1);
  // (height, width)
}

  
fn get_image_from_path(path: &str) -> DynamicImage {
  image::open(&Path::new(&path)).expect("Could not open image")
}

fn get_empty_image(width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8> > {
  let image = ImageBuffer::<Rgba<u8>, Vec<u8> >::new(width, height);
  image
}

fn save(image: ImageBuffer<Rgba<u8>, Vec<u8> >, path: &str) {
  image.save(path).unwrap();
}
