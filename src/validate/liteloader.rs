use crate::validate::{
    SupportedGameVersions, ValidationError, ValidationResult,
};
use std::io::Cursor;
use zip::ZipArchive;

pub struct LiteLoaderValidator;

impl super::Validator for LiteLoaderValidator {
    fn get_file_extensions(&self) -> &[&str] {
        &["litemod"]
    }

    fn get_project_types(&self) -> &[&str] {
        &["mod"]
    }

    fn get_supported_loaders(&self) -> &[&str] {
        &["liteloader"]
    }

    fn get_supported_game_versions(&self) -> SupportedGameVersions {
        SupportedGameVersions::All
    }

    fn validate(
        &self,
        archive: &mut ZipArchive<Cursor<bytes::Bytes>>,
    ) -> Result<ValidationResult, ValidationError> {
        archive.by_name("litemod.json").map_err(|_| {
            ValidationError::InvalidInput(
                "No litemod.json present for LiteLoader file.".into(),
            )
        })?;

        Ok(ValidationResult::Pass)
    }
}

#[cfg(test)]
mod tests {
    use crate::validate::{
        test_util::make_dummy_zip, ValidationError, ValidationResult, Validator,
    };

    use super::LiteLoaderValidator;

    #[test]
    fn all_clear() {
        let mut zip = make_dummy_zip(&["litemod.json"]).unwrap();

        assert_eq!(
            LiteLoaderValidator.validate(&mut zip).unwrap(),
            ValidationResult::Pass
        );
    }
    #[test]
    fn missing_litemod_json() {
        let mut zip = make_dummy_zip(&[]).unwrap();

        assert!(matches!(
            LiteLoaderValidator.validate(&mut zip).unwrap_err(),
            ValidationError::InvalidInput(error)
            if error == "No litemod.json present for LiteLoader file."
        ));
    }
}
