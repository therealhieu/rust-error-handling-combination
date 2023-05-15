use error_stack::{IntoReport, Result, ResultExt};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExtractError {
    #[error("[ExtractorError::FromFile] Error when extract content from file: {path:#?}, func: {func:#?}")]
    FromFile { path: String, func: &'static str },

    #[error("[ExtractorError::FromString] Error when extract content from string: {0:#?}")]
    FromString(String),
}

#[derive(Debug)]
pub enum ExtractOutput {
    Json(serde_json::Value),
    Yaml(serde_yaml::Value),
}

impl From<serde_json::Value> for ExtractOutput {
    fn from(value: serde_json::Value) -> Self {
        ExtractOutput::Json(value)
    }
}

impl From<serde_yaml::Value> for ExtractOutput {
    fn from(value: serde_yaml::Value) -> Self {
        ExtractOutput::Yaml(value)
    }
}

#[derive(Debug)]
pub enum ContentType {
    Json,
    Yaml,
}

#[derive(Debug, Default)]
pub struct Extractor;

impl Extractor {
    pub fn extract_from_file(
        &self,
        path: &str,
        content_type: ContentType,
    ) -> Result<ExtractOutput, ExtractError> {
        let content =
            std::fs::read_to_string(path)
                .into_report()
                .change_context(ExtractError::FromFile {
                    path: path.to_string(),
                    func: "std::fs::read_to_string",
                })?;

        self.extract_from_string(&content, content_type)
            .change_context(ExtractError::FromFile {
                path: path.to_string(),
                func: "self.extract_from_string",
            })
    }

    fn extract_from_json(&self, content: &str) -> Result<serde_json::Value, ExtractError> {
        serde_json::from_str(content)
            .into_report()
            .attach_printable("serde_json::from_str")
            .change_context(ExtractError::FromString(content.to_string()))
    }

    fn extract_from_yaml(&self, content: &str) -> Result<serde_yaml::Value, ExtractError> {
        serde_yaml::from_str(content)
            .into_report()
            .change_context(ExtractError::FromString(content.to_string()))
    }

    pub fn extract_from_string(
        &self,
        content: &str,
        content_type: ContentType,
    ) -> Result<ExtractOutput, ExtractError> {
        match content_type {
            ContentType::Json => self.extract_from_json(content).map(ExtractOutput::from),
            ContentType::Yaml => self.extract_from_yaml(content).map(ExtractOutput::from),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn invalid_json_content() -> &'static str {
        r#"
        [
            {,}
        ]
        "#
    }

    #[rstest]
    fn extract_from_string_error(invalid_json_content: &'static str) {
        let extractor = Extractor {};
        let err = extractor
            .extract_from_string(invalid_json_content, ContentType::Json)
            .unwrap_err();
        println!("{:#?}", err);
    }

    #[rstest]
    fn extract_from_file_error(invalid_json_content: &'static str) {
        let mut tmp_file = tempfile::NamedTempFile::new().unwrap();
        tmp_file.write_all(invalid_json_content.as_bytes()).unwrap();
        let tmp_file_path = tmp_file.path().to_str().unwrap();

        let extractor = Extractor {};
        let err = extractor
            .extract_from_file(tmp_file_path, ContentType::Json)
            .unwrap_err();
        println!("{:#?}", err);
    }
}
