use crate::models::pack::PackFormat;
use crate::models::projects::{GameVersion, Loader};
use crate::validate::fabric::FabricValidator;
use crate::validate::forge::{ForgeValidator, LegacyForgeValidator};
use crate::validate::liteloader::LiteLoaderValidator;
use crate::validate::pack::PackValidator;
use crate::validate::quilt::QuiltValidator;
use std::io::Cursor;
use thiserror::Error;
use time::OffsetDateTime;
use zip::ZipArchive;

mod fabric;
mod forge;
mod liteloader;
mod pack;
mod quilt;
#[cfg(test)]
mod test_util;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Unable to read Zip Archive: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Error while validating JSON: {0}")]
    SerDe(#[from] serde_json::Error),
    #[error("Invalid Input: {0}")]
    InvalidInput(std::borrow::Cow<'static, str>),
    #[error("Error while managing threads")]
    Blocking(#[from] actix_web::error::BlockingError),
}

#[derive(Debug, Eq, PartialEq)]
pub enum ValidationResult {
    /// File should be marked as primary with pack file data
    PassWithPackData(PackFormat),
    /// File should be marked as primary
    Pass,
    /// File should not be marked primary, the reason for which is inside the String
    Warning(&'static str),
}

impl ValidationResult {
    pub fn is_passed(&self) -> bool {
        match self {
            ValidationResult::PassWithPackData(_) | ValidationResult::Pass => {
                true
            }
            ValidationResult::Warning(_) => false,
        }
    }
}

pub enum SupportedGameVersions {
    All,
    PastDate(OffsetDateTime),
    Range(OffsetDateTime, OffsetDateTime),
    #[allow(dead_code)]
    Custom(Vec<GameVersion>),
}

pub trait Validator: Sync {
    fn get_file_extensions(&self) -> &[&str];
    fn get_project_types(&self) -> &[&str];
    fn get_supported_loaders(&self) -> &[&str];
    fn get_supported_game_versions(&self) -> SupportedGameVersions;
    fn validate(
        &self,
        archive: &mut ZipArchive<Cursor<bytes::Bytes>>,
    ) -> Result<ValidationResult, ValidationError>;
}

static VALIDATORS: [&dyn Validator; 6] = [
    &PackValidator,
    &FabricValidator,
    &ForgeValidator,
    &LegacyForgeValidator,
    &QuiltValidator,
    &LiteLoaderValidator,
];

/// The return value is whether this file should be marked as primary or not, based on the analysis of the file
pub async fn validate_file(
    data: bytes::Bytes,
    file_extension: String,
    project_type: String,
    loaders: Vec<Loader>,
    game_versions: Vec<GameVersion>,
    all_game_versions: Vec<crate::database::models::categories::GameVersion>,
) -> Result<ValidationResult, ValidationError> {
    actix_web::web::block(move || {
        let reader = std::io::Cursor::new(data);
        let mut zip = zip::ZipArchive::new(reader)?;

        let mut visited = false;
        for validator in &VALIDATORS {
            if validator.get_project_types().contains(&&*project_type)
                && loaders
                    .iter()
                    .any(|x| validator.get_supported_loaders().contains(&&*x.0))
                && game_version_supported(
                    &game_versions,
                    &all_game_versions,
                    validator.get_supported_game_versions(),
                )
            {
                if validator.get_file_extensions().contains(&&*file_extension) {
                    return validator.validate(&mut zip);
                }
                visited = true;
                break;
            }
        }

        if visited {
            Err(ValidationError::InvalidInput(
                format!(
                    "File extension {} is invalid for input file",
                    file_extension
                )
                .into(),
            ))
        } else {
            Ok(ValidationResult::Pass)
        }
    })
    .await?
}

fn game_version_supported(
    game_versions: &[GameVersion],
    all_game_versions: &[crate::database::models::categories::GameVersion],
    supported_game_versions: SupportedGameVersions,
) -> bool {
    match supported_game_versions {
        SupportedGameVersions::All => true,
        SupportedGameVersions::PastDate(date) => {
            game_versions.iter().any(|x| {
                all_game_versions
                    .iter()
                    .find(|y| y.version == x.0)
                    .map_or(false, |x| x.date > date)
            })
        }
        SupportedGameVersions::Range(before, after) => {
            game_versions.iter().any(|x| {
                all_game_versions
                    .iter()
                    .find(|y| y.version == x.0)
                    .map_or(false, |x| x.date > before && x.date < after)
            })
        }
        SupportedGameVersions::Custom(versions) => {
            versions.iter().any(|x| game_versions.contains(x))
        }
    }
}

fn match_extension_ignore_case(name: &str, exts: &[&str]) -> bool {
    exts.iter().any(|ext| {
        name.chars()
            .rev()
            .zip(ext.chars().rev())
            .all(|(nc, ec)| nc.eq_ignore_ascii_case(&ec))
    })
}

#[cfg(test)]
mod tests {
    use crate::validate::match_extension_ignore_case;

    #[test]
    fn matching_extensions_works() {
        assert!(match_extension_ignore_case("quilt.mod.json", &[".json"]));
        assert!(match_extension_ignore_case(
            "quilt.mod.json",
            &[".mod.json"]
        ));
        assert!(match_extension_ignore_case(
            "quilt.mod.json",
            &["quilt.mod.json"]
        ));
    }
    #[test]
    fn wrong_extension() {
        assert!(!match_extension_ignore_case("quilt.mod.json", &[".toml"]));
        assert!(!match_extension_ignore_case("quilt.mod.json", &[".class"]));
        assert!(!match_extension_ignore_case("mods.toml", &[".yaml"]));
        assert!(!match_extension_ignore_case("example.refmap.json", &[".class"]));
    }
    #[test]
    fn multiple_extensions() {
        assert!(match_extension_ignore_case(
            "quilt.mod.json",
            &[".not.gonna.match", ".json"]
        ));
        assert!(match_extension_ignore_case(
            "Test$1$2$3.class",
            &[".refmap.json", ".class"]
        ));
    }
    #[test]
    fn multiple_wrong_extensions() {
        assert!(!match_extension_ignore_case(
            "quilt.mod.json",
            &[".neither.of", ".these.matches"]
        ));
        assert!(!match_extension_ignore_case(
            "Test.class",
            &[".java", ".kt", ".scala", ".groovy"]
        ));
    }
}
