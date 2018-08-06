mod image_diff;

pub fn main() {
  let orig = "./images/pikachu1.png";
  let new = "./images/pikachu2.png";

  let threshold = 0.1;

  image_diff::ImageDiff::compare(orig, new, threshold);
}
