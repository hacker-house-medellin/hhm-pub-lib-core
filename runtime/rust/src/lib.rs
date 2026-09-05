#![forbid(unsafe_code)]

pub mod generated;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidationError {
    pub code: &'static str,
}

impl ValidationError {
    const fn invalid_shape() -> Self {
        Self {
            code: "invalid_shape",
        }
    }

    const fn constraint_violation() -> Self {
        Self {
            code: "constraint_violation",
        }
    }
}

pub fn validate_json(model: &str, input: &str) -> Result<(), ValidationError> {
    match model {
        "ClientInfo" => {
            let value: generated::ClientInfo =
                serde_json::from_str(input).map_err(|_| ValidationError::invalid_shape())?;
            bounded(&value.install_id, 128)?;
            bounded(&value.app_version, 64)?;
            if let Some(locale) = value.locale.as_deref() {
                bounded(locale, 16)?;
            }
            Ok(())
        }
        "IdempotencyKey" => {
            let value: generated::IdempotencyKey =
                serde_json::from_str(input).map_err(|_| ValidationError::invalid_shape())?;
            bounded(&value.key, 128)
        }
        "PublicLocation" => {
            let value: generated::PublicLocation =
                serde_json::from_str(input).map_err(|_| ValidationError::invalid_shape())?;
            bounded(&value.slug, 64)?;
            bounded(&value.display_name, 120)?;
            bounded(&value.city, 80)?;
            bounded(&value.country_code, 2)?;
            bounded(&value.timezone, 64)
        }
        "PublicAccountContext" => {
            let value: generated::PublicAccountContext =
                serde_json::from_str(input).map_err(|_| ValidationError::invalid_shape())?;
            bounded(&value.display_name, 120)
        }
        _ => Err(ValidationError {
            code: "unknown_model",
        }),
    }
}

fn bounded(value: &str, maximum: usize) -> Result<(), ValidationError> {
    if value.len() > maximum {
        Err(ValidationError::constraint_violation())
    } else {
        Ok(())
    }
}
