use crate::validate::{
    SupportedGameVersions, ValidationError, ValidationResult,
};
use std::io::Cursor;
use time::OffsetDateTime;
use zip::ZipArchive;

use super::match_extension_ignore_case;

pub struct QuiltValidator;

impl super::Validator for QuiltValidator {
    fn get_file_extensions(&self) -> &[&str] {
        &["jar", "zip"]
    }

    fn get_project_types(&self) -> &[&str] {
        &["mod"]
    }

    fn get_supported_loaders(&self) -> &[&str] {
        &["fabric"]
    }

    fn get_supported_game_versions(&self) -> SupportedGameVersions {
        // Time since release of 18w49a, the first fabric version
        SupportedGameVersions::PastDate(OffsetDateTime::from_unix_timestamp(
            1646070100,
        ))
    }

    fn validate(
        &self,
        archive: &mut ZipArchive<Cursor<bytes::Bytes>>,
    ) -> Result<ValidationResult, ValidationError> {
        archive.by_name("quilt.mod.json").map_err(|_| {
            ValidationError::InvalidInput(
                "No quilt.mod.json present for Quilt file.".into(),
            )
        })?;

        if !archive.file_names().any(|name| {
            match_extension_ignore_case(name, &[".refmap.json", ".class"])
        }) {
            return Ok(ValidationResult::Warning(
                "Quilt mod file is a source file!",
            ));
        }

        Ok(ValidationResult::Pass)
    }
}

#[cfg(test)]
mod tests {
    use crate::validate::{
        test_util::make_dummy_zip, ValidationError, ValidationResult, Validator,
    };

    use super::QuiltValidator;

    #[test]
    fn all_clear() {
        let mut zip = make_dummy_zip(&[
            "quilt.mod.json",
            "Test.class",
            "example.refmap.json",
        ])
        .unwrap();

        assert_eq!(
            QuiltValidator.validate(&mut zip).unwrap(),
            ValidationResult::Pass
        );
    }
    #[test]
    fn weird_extensions() {
        let mut zip = make_dummy_zip(&[
            "quilt.mod.json",
            "TEST.CLASS",
            "EXAMPLE.ReFMaP.JSon",
        ])
        .unwrap();

        assert_eq!(
            QuiltValidator.validate(&mut zip).unwrap(),
            ValidationResult::Pass
        );
    }
    #[test]
    fn missing_qmj() {
        let mut zip =
            make_dummy_zip(&["Test.class", "example.refmap.json"]).unwrap();

        assert!(matches!(
            QuiltValidator.validate(&mut zip).unwrap_err(),
            ValidationError::InvalidInput(error)
            if error == "No quilt.mod.json present for Quilt file."
        ));
    }
    #[test]
    fn missing_refmap_and_class_files() {
        let mut zip =
            make_dummy_zip(&["quilt.mod.json"]).unwrap();

        assert_eq!(
            QuiltValidator.validate(&mut zip).unwrap(),
            ValidationResult::Warning("Quilt mod file is a source file!")
        );
    }
}
