use serde::{Deserialize, Serialize};

/// Email struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Email {
    /// email target address
    #[serde(alias = "targetAddress")]
    pub target_address: String,
    /// email subject
    #[serde(alias = "emailSubject")]
    pub email_subject: Option<String>,
    /// email body
    #[serde(alias = "emailBody")]
    pub email_body: String,
}

/// Unit test cases
#[cfg(test)]
mod tests {
    use crate::dto::email::Email;

    /// Scenario:
    /// Creates a Email struct with valid values
    /// Expectation:
    /// A Email with proper values should be created
    #[test]
    fn when_create_email_with_proper_values_should_return_set_values() {
        let expected_target_address = "target";
        let expected_email_subject = "subject";
        let expected_email_body = "body";

        let email = Email {
            target_address: String::from(expected_target_address),
            email_subject: Some(String::from(expected_email_subject)),
            email_body: String::from(expected_email_body),
        };

        assert_eq!(expected_target_address, email.target_address);
        assert_eq!(
            expected_email_subject,
            email.email_subject.unwrap_or_default()
        );
        assert_eq!(expected_email_body, email.email_body);
    }
}
