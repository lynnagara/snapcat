mod image_diff;

use std::env::args;

pub fn main() {
  let (orig, new, threshold) = get_args();
  image_diff::ImageDiff::compare(orig, new, threshold)
}

fn get_args() -> (String, String, f32) {
  let default_threshold = 0.;

  let args: Vec<_> = args().collect();

  let orig = args[1].to_string();
  let new = args[2].to_string();

  let threshold = if args.len() >= 4 {
    (&args[3]).parse::<f32>().unwrap()
  } else {
    default_threshold
  };

  (orig, new, threshold)
}
