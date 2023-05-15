use error_stack::{IntoReport, Report, Result, ResultExt};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use extract::{ContentType, ExtractOutput, Extractor};

#[derive(Debug, Error)]
pub enum TransformError {
    #[error("[TransformError::AddAgeGroup] Error when add age group to person: {0:#?}")]
    AddAgeGroup(Person),
    #[error("[TransformError::ExceptionalAge] Invalid age: {0:#?}")]
    ExceptionalAge(u8),

    #[error("[TransformError::TransformFile] Error when transform file: {0:#?}")]
    TransformFile(String),

    #[error("[TransformError::TransformString] Error when transform string")]
    TransformString,

    #[error("[TransformError::ConvertExtractOutput] Error when convert extract output")]
    ConvertExtractOutput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    name: String,
    age: u8,
    age_group: Option<AgeGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgeGroup {
    Child,
    Teen,
    Adult,
    Senior,
}

impl AgeGroup {
    pub fn from_age(age: u8) -> Result<Self, TransformError> {
        match age {
            0..=12 => Ok(AgeGroup::Child),
            13..=19 => Ok(AgeGroup::Teen),
            20..=59 => Ok(AgeGroup::Adult),
            60..=100 => Ok(AgeGroup::Senior),
            _ => Err(TransformError::ExceptionalAge(age)).into_report(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonList(pub Vec<Person>);

impl TryFrom<ExtractOutput> for PersonList {
    type Error = Report<TransformError>;

    fn try_from(
        extract_output: ExtractOutput,
    ) -> std::result::Result<PersonList, Report<TransformError>> {
        let person_list = match extract_output {
            ExtractOutput::Json(value) => serde_json::from_value::<Vec<Person>>(value)
                .into_report()
                .change_context(TransformError::ConvertExtractOutput)?,
            ExtractOutput::Yaml(value) => serde_yaml::from_value::<Vec<Person>>(value)
                .into_report()
                .change_context(TransformError::ConvertExtractOutput)?,
        };

        Ok(PersonList(person_list))
    }
}

#[derive(Debug, Default)]
pub struct Transform {
    pub extractor: Extractor,
}

impl Transform {
    pub fn transform_file(
        &self,
        path: &str,
        content_type: ContentType,
    ) -> Result<Vec<Person>, TransformError> {
        let extract_output = self
            .extractor
            .extract_from_file(path, content_type)
            .change_context(TransformError::TransformFile(path.to_string()))?;
        let mut person_list = PersonList::try_from(extract_output)
            .change_context(TransformError::TransformFile(path.to_string()))?;

        for person in &mut person_list.0 {
            self.add_age_group(person)?;
        }

        Ok(person_list.0)
    }

    pub fn transform_string(
        &self,
        content: &str,
        content_type: ContentType,
    ) -> Result<Vec<Person>, TransformError> {
        let extract_output = self
            .extractor
            .extract_from_string(content, content_type)
            .change_context(TransformError::TransformString)?;
        let mut person_list =
            PersonList::try_from(extract_output).change_context(TransformError::TransformString)?;

        for person in &mut person_list.0 {
            self.add_age_group(person)
                .change_context(TransformError::TransformString)?;
        }

        Ok(person_list.0)
    }

    pub fn add_age_group(&self, person: &mut Person) -> Result<(), TransformError> {
        person.age_group = AgeGroup::from_age(person.age)
            .change_context(TransformError::AddAgeGroup(person.clone()))
            .map(Some)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn invalid_person_list() -> &'static str {
        r#"
        [
            {
                "name": "John",
                "age": 10
            },
            {
                "name": "Jane",
                "age": 200
            }
        ]
        "#
    }

    #[rstest]
    fn transform_file_error(invalid_person_list: &'static str) {
        let transform = Transform::default();
        let err = transform
            .transform_string(invalid_person_list, ContentType::Json)
            .unwrap_err();
        println!("{:#?}", err);
    }
}
