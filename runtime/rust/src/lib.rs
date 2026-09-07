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
    // Contract lengths count Unicode scalars, not UTF-8 bytes or grapheme clusters.
    // Stop at the first scalar beyond the limit; no allocation or full scan needed.
    if value.chars().nth(maximum).is_some() {
        Err(ValidationError::constraint_violation())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{bounded, ValidationError};

    #[test]
    fn ascii_boundary_is_unchanged() {
        assert_eq!(bounded("abc", 3), Ok(()));
        assert_eq!(
            bounded("abcd", 3),
            Err(ValidationError::constraint_violation())
        );
    }

    #[test]
    fn unicode_scalars_not_utf8_bytes_define_length() {
        for scalar in ["é", "中", "😀"] {
            assert_eq!(bounded(&scalar.repeat(3), 3), Ok(()));
            assert_eq!(
                bounded(&scalar.repeat(4), 3),
                Err(ValidationError::constraint_violation())
            );
        }
    }

    #[test]
    fn combining_sequences_are_not_silently_normalized() {
        assert_eq!(bounded("e\u{301}", 2), Ok(()));
        assert_eq!(
            bounded("e\u{301}", 1),
            Err(ValidationError::constraint_violation())
        );
    }

    #[test]
    fn zero_maximum_accepts_only_empty_input() {
        assert_eq!(bounded("", 0), Ok(()));
        assert_eq!(
            bounded("é", 0),
            Err(ValidationError::constraint_violation())
        );
    }
}
