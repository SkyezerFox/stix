use std::{env::current_dir, error::Error, fs, path::PathBuf};

use serde::Deserialize;

/// The name of the manifest file used by Styx.
const MANIFEST_NAME: &'static str = "package.toml";

/// A manifest dependency.
#[derive(Deserialize)]
pub struct Dependency {
    /// The name of the dependency package.
    pub name: String,
    /// The version of the package.
    pub version: String,
    /// Features to use from this package.
    pub features: Vec<String>
}


/// A package manifest.
#[derive(Deserialize)]
pub struct Manifest {
    /// The name of the package.
    pub name: String,
    /// The version of the package.
    pub version: String,
    /// The authors of the package.
    pub authors: Vec<String>,
    /// A description of the package.
    pub description: String,
    /// The license of the package.
    pub license: String,
    /// Dependencies of the package.
    pub dependencies: Vec<Dependency>
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            name: "my-package".into(),
            version: "0.0.0".into(),
            authors: vec![],
            description: "".into(),
            license: "".into(),
            dependencies: vec![]
        }
    }
}

/// Recursively resolve the path of the package manifest.
pub fn resolve_manifset() -> Result<PathBuf, Box<dyn Error>> {
    // helper function
    fn _resolve(current_dir: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
        // check current directory contents
        for entry in fs::read_dir(&current_dir)? {
            let entry = entry?;
            if entry.file_name() == MANIFEST_NAME {
                return Ok(entry.path())
            };
        };
        // recursively ascend up fs tree
        let parent = current_dir.parent();
        match parent {
            None => Err("Could not find package.toml".into()),
            Some(parent_path) => _resolve(parent_path.into())
        }
    }
    _resolve(current_dir()?)
}

/// Attempt to load the manifest for this project.
pub fn load_manifest() -> Result<Manifest, Box<dyn Error>> {
    let manifest_path = resolve_manifset()?;
    let manifest = fs::read_to_string(manifest_path)?;
    let manifest: Manifest = toml::from_str(manifest.as_str())?;
    Ok(manifest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_manifest() {
        let manifest = resolve_manifset().unwrap();
    }
}
