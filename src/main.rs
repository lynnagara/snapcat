mod image_diff;

use std::env::args;

pub fn main() {
  let (orig, new, threshold) = get_args();
  image_diff::ImageDiff::compare(orig, new, threshold)
}

fn get_args() -> (String, String, f32) {
  let DEFAULT_THRESHOLD = 0.;

  let args: Vec<_> = args().collect();
  let orig = args[1].to_string();
  let new = args[2].to_string();
  let threshold: f32 = 0.1;

  (orig, new, threshold)
}
