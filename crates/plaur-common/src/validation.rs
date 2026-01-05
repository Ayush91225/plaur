use crate::error::{PlaurdError, Result};
use validator::{Validate, ValidationErrors};

pub fn validate_request<T: Validate>(request: &T) -> Result<()> {
    request.validate().map_err(|errors| {
        let message = format_validation_errors(&errors);
        PlaurdError::Validation { message }
    })
}

fn format_validation_errors(errors: &ValidationErrors) -> String {
    let mut messages = Vec::new();
    
    for (field, field_errors) in errors.field_errors() {
        for error in field_errors {
            let message = match &error.code {
                std::borrow::Cow::Borrowed("email") => format!("{} must be a valid email", field),
                std::borrow::Cow::Borrowed("length") => {
                    if let Some(params) = &error.params {
                        if let Some(min) = params.get("min") {
                            format!("{} must be at least {} characters", field, min)
                        } else if let Some(max) = params.get("max") {
                            format!("{} must be at most {} characters", field, max)
                        } else {
                            format!("{} has invalid length", field)
                        }
                    } else {
                        format!("{} has invalid length", field)
                    }
                }
                std::borrow::Cow::Borrowed("required") => format!("{} is required", field),
                _ => format!("{} is invalid", field),
            };
            messages.push(message);
        }
    }
    
    messages.join(", ")
}