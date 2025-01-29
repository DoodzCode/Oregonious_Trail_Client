use rand::Rng;
use serde::Serialize;
use std::fs::{File, remove_file};
use std::io::Write;
use std::path::Path;

pub fn d20() -> u8 {
  let mut rng = rand::thread_rng();
  rng.gen_range(1..=20)
}

pub fn save_to_file<T: Serialize>(data: &T, filename: &str) -> std::io::Result<()> {
  let path: std::path::PathBuf = Path::new("src/config").join(filename);
  let json: String = serde_json::to_string(data)?;
  let mut file: File = File::create(path)?;
  file.write_all(json.as_bytes())?;
  Ok(())
}


#[cfg(test)]
mod test {
  use super::*;
  
  #[derive(Serialize)]
    struct TestStruct {
        name: String,
        value: u32,
    }

  #[test]
  fn test_d20() {
    let result = d20();
    assert!(result >= 1 && result <= 20);
  }

  #[test]
  fn test_save_to_file() {
    let data: TestStruct = TestStruct {
      name: String::from("Test"),
      value: 42,
    };
    let result: Result<(), std::io::Error> = save_to_file(&data, "test_data.json");
    assert!(result.is_ok());

    let remove_result: Result<(), std::io::Error> = remove_file("src/config/test_data.json");
    assert!(remove_result.is_ok());
  }
}

