use std::{
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
};

use serde::Serialize;

#[derive(Serialize)]
struct Manifest<'a> {
    package: Package<'a>,
}

#[derive(Serialize)]
struct Package<'a> {
    name: &'a str,
    version: &'static str,
    edition: &'a str,
}

pub fn write_manifest(
    path: impl AsRef<Path>,
    contest_id: &str,
    edition: &str,
) -> Result<(), ManifestError> {
    let path = path.as_ref();

    if !is_valid_package_name(contest_id) {
        return Err(ManifestError::InvalidPackageName {
            path: path.to_path_buf(),
            name: contest_id.to_owned(),
        });
    }

    if !matches!(edition, "2015" | "2018" | "2021" | "2024") {
        return Err(ManifestError::InvalidEdition {
            path: path.to_path_buf(),
            edition: edition.to_owned(),
        });
    }

    let manifest = Manifest {
        package: Package {
            name: contest_id,
            version: "0.1.0",
            edition,
        },
    };
    let contents =
        toml::to_string_pretty(&manifest).map_err(|source| ManifestError::Serialize {
            path: path.to_path_buf(),
            source,
        })?;

    fs::write(path, contents).map_err(|source| ManifestError::Write {
        path: path.to_path_buf(),
        source,
    })
}

pub(crate) fn is_valid_package_name(name: &str) -> bool {
    let mut characters = name.chars();
    let Some(first) = characters.next() else {
        return false;
    };

    first.is_ascii_alphabetic()
        && characters
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_'))
}

#[derive(Debug)]
pub enum ManifestError {
    InvalidPackageName {
        path: PathBuf,
        name: String,
    },
    InvalidEdition {
        path: PathBuf,
        edition: String,
    },
    Serialize {
        path: PathBuf,
        source: toml::ser::Error,
    },
    Write {
        path: PathBuf,
        source: io::Error,
    },
}

impl ManifestError {
    pub fn path(&self) -> &Path {
        match self {
            Self::InvalidPackageName { path, .. }
            | Self::InvalidEdition { path, .. }
            | Self::Serialize { path, .. }
            | Self::Write { path, .. } => path,
        }
    }
}

impl fmt::Display for ManifestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPackageName { name, .. } => {
                write!(formatter, "invalid Cargo package name `{name}`")
            }
            Self::InvalidEdition { edition, .. } => {
                write!(formatter, "invalid Rust edition `{edition}`")
            }
            Self::Serialize { path, .. } => {
                write!(
                    formatter,
                    "failed to serialize manifest `{}`",
                    path.display()
                )
            }
            Self::Write { path, .. } => {
                write!(formatter, "failed to write manifest `{}`", path.display())
            }
        }
    }
}

impl Error for ManifestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Serialize { source, .. } => Some(source),
            Self::Write { source, .. } => Some(source),
            Self::InvalidPackageName { .. } | Self::InvalidEdition { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, process::Command};

    use tempfile::tempdir;

    use super::{write_manifest, ManifestError};

    #[test]
    fn writes_minimal_valid_cargo_manifest() {
        let directory = tempdir().expect("temporary directory should be created");
        let manifest_path = directory.path().join("Cargo.toml");
        let source_directory = directory.path().join("src");
        fs::create_dir(&source_directory).expect("source directory should be created");
        fs::write(source_directory.join("main.rs"), "fn main() {}")
            .expect("test source should be written");

        write_manifest(&manifest_path, "abc400", "2021").expect("manifest should be written");

        let contents = fs::read_to_string(&manifest_path).expect("manifest should be readable");
        let manifest: toml::Value = toml::from_str(&contents).expect("manifest should be TOML");
        assert_eq!(manifest["package"]["name"].as_str(), Some("abc400"));
        assert_eq!(manifest["package"]["version"].as_str(), Some("0.1.0"));
        assert_eq!(manifest["package"]["edition"].as_str(), Some("2021"));
        assert!(manifest.get("dependencies").is_none());

        let output = Command::new(env!("CARGO"))
            .args([
                "metadata",
                "--no-deps",
                "--format-version=1",
                "--manifest-path",
            ])
            .arg(&manifest_path)
            .output()
            .expect("cargo metadata should run");
        assert!(
            output.status.success(),
            "cargo metadata failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn rejects_invalid_package_names_without_writing_manifest() {
        let directory = tempdir().expect("temporary directory should be created");

        for name in ["", "-abc400", "abc.400", "abc/400"] {
            let manifest_path = directory.path().join(format!("{name:?}.toml"));

            let error = write_manifest(&manifest_path, name, "2021")
                .expect_err("invalid package name should be rejected");

            assert_eq!(error.path(), manifest_path);
            assert!(matches!(error, ManifestError::InvalidPackageName { .. }));
            assert!(!manifest_path.exists());
        }
    }

    #[test]
    fn rejects_invalid_edition_without_writing_manifest() {
        let directory = tempdir().expect("temporary directory should be created");
        let manifest_path = directory.path().join("Cargo.toml");

        let error = write_manifest(&manifest_path, "abc400", "invalid")
            .expect_err("invalid edition should be rejected");

        assert_eq!(error.path(), manifest_path);
        assert!(matches!(error, ManifestError::InvalidEdition { .. }));
        assert!(!manifest_path.exists());
    }
}
