#[derive(PartialEq)]
pub enum ExistenceProcessing {
  Clear,
  Overwrite,
  Cancel,
}

impl ExistenceProcessing {
  pub fn from_choice(choice: &str) -> Self {
    match choice {
      "Clear the directory and continue" => ExistenceProcessing::Clear,
      "Ignore and continue (conflicting files will be overwritten)" => {
        ExistenceProcessing::Overwrite
      }
      "Cancel operation" => ExistenceProcessing::Cancel,
      _ => unreachable!(),
    }
  }
}
