mod image_diff;

use std::env::args;
use std::fs::{File, read_dir};
use std::io::Read;
use std::str;

#[no_mangle]
pub unsafe extern "C" fn generate_diffs(threshold: f32) {
  let base = "./.snapshots/";
  let orig = "refs-heads-snapcat";
  let new = get_current_ref();

  // bail if branches are the same
  if orig == new {
    panic!("you are on the base branch!!!")
  }

  let new_files = get_directory_files(format!("{}{}", base, orig));


  for new_file in new_files {
    let orig_file = str::replace(&new_file.as_str(), &orig, &new);
    let diff_path = str::replace(&new_file.as_str(), &orig, "diffs");
    image_diff::ImageDiff::compare(orig_file, new_file, diff_path, threshold);
  }
}

fn main() {
  let (orig, new, threshold) = get_args();
  let diff_path = "output.png".to_string();
  image_diff::ImageDiff::compare(new, orig, diff_path, threshold)
}


fn get_directory_files(path: String) -> Vec<String> {
  let paths = read_dir(path).unwrap();
  let mut files = Vec::new();
  for path in paths {
    let unwrapped = path.unwrap().path();
    if unwrapped.is_dir() {
     let path_as_str = unwrapped.into_os_string().into_string().unwrap();
      files.extend(get_directory_files(path_as_str));
    } else {
      let path_as_str = unwrapped.into_os_string().into_string().unwrap();
      files.push(path_as_str);
    }
  }
  files
    .into_iter()
    .filter(|file| file.ends_with(".png"))
    .collect()
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

fn get_current_ref() -> String {
  let mut file = File::open(".git/HEAD").expect("File not found");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Error reading file");
  let current_ref = contents.trim().split(" ").last().unwrap().to_string();

  sanitize_file_path(&current_ref)
}

fn sanitize_file_path(path: &str) -> String {
  str::replace(path, "/", "-")
}