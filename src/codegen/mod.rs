pub mod typescript;

use crate::error::Result;
use std::path::Path;

#[allow(dead_code)]
pub trait CodeGenerator {
    fn generate(&self, project_root: &Path, output_dir: &str) -> Result<()>;
}
