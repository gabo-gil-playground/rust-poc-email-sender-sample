use constcat::concat;

/// logging configuration file environment variable name
pub const LOGGING_CONFIG_FILE_ENV_VAR: &str = "LOG4RS_CONFIG_FILE";
/// logging configuration file default value
pub const LOGGING_CONFIG_FILE_DEFAULT: &str = "logging_config.yaml";

/// API server host environment variable name
pub const API_SERVER_HOST_ENV_VAR: &str = "API_SERVER_HOST";
/// API server port environment variable name
pub const API_SERVER_PORT_ENV_VAR: &str = "API_SERVER_PORT";
/// API server host default value
pub const API_SERVER_HOST_DEFAULT: &str = "0.0.0.0";
/// API server port default value
pub const API_SERVER_PORT_DEFAULT: &str = "9026";
/// server running status message
pub const SERVER_RUNNING_STATUS: &str = "server is running";

/// email app environment variables name
pub const APP_EMAIL_ADDRESS_ENV_VAR: &str = "APP_EMAIL_ADDRESS";
pub const APP_EMAIL_PASSWORD_ENV_VAR: &str = "APP_EMAIL_PASSWORD";
pub const APP_EMAIL_SMTP_SERVER_ENV_VAR: &str = "APP_EMAIL_SMTP_SERVER";

/// email app default values
pub const APP_EMAIL_ADDRESS_DEFAULT: &str = "youremail@gmail.com";
pub const APP_EMAIL_PASSWORD_DEFAULT: &str = "abcd1234";
pub const APP_EMAIL_SMTP_SERVER_DEFAULT: &str = "smtp.gmail.com";

/// API Health-check main path
pub const API_HEALTH_CHECK_PATH: &str = "/health";

/// API main path
pub const API_MAIN_PATH: &str = "/api/v1";

/// API email main path
pub const API_EMAIL_MAIN_PATH: &str = concat!(API_MAIN_PATH, "/email");
pub const API_EMAIL_SEND_PATH: &str = "/send";

/// Unit test cases
#[cfg(test)]
mod tests {
    use crate::constant::constants::*;

    /// Scenario:
    /// Executes constant values validation
    /// Expectation:
    /// Constant values should be validated
    #[test]
    fn when_constant_values_are_valid() {
        assert_eq!("LOG4RS_CONFIG_FILE", LOGGING_CONFIG_FILE_ENV_VAR);
        assert_eq!("logging_config.yaml", LOGGING_CONFIG_FILE_DEFAULT);

        assert_eq!("API_SERVER_HOST", API_SERVER_HOST_ENV_VAR);
        assert_eq!("API_SERVER_PORT", API_SERVER_PORT_ENV_VAR);
        assert_eq!("0.0.0.0", API_SERVER_HOST_DEFAULT);
        assert_eq!("9026", API_SERVER_PORT_DEFAULT);
        assert_eq!("server is running", SERVER_RUNNING_STATUS);

        assert_eq!("APP_EMAIL_ADDRESS", APP_EMAIL_ADDRESS_ENV_VAR);
        assert_eq!("APP_EMAIL_PASSWORD", APP_EMAIL_PASSWORD_ENV_VAR);
        assert_eq!("APP_EMAIL_SMTP_SERVER", APP_EMAIL_SMTP_SERVER_ENV_VAR);
        assert_eq!("youremail@gmail.com", APP_EMAIL_ADDRESS_DEFAULT);
        assert_eq!("abcd1234", APP_EMAIL_PASSWORD_DEFAULT);
        assert_eq!("smtp.gmail.com", APP_EMAIL_SMTP_SERVER_DEFAULT);

        assert_eq!("/health", API_HEALTH_CHECK_PATH);

        assert_eq!("/api/v1", API_MAIN_PATH);
        assert_eq!("/api/v1/email", API_EMAIL_MAIN_PATH);
        assert_eq!("/send", API_EMAIL_SEND_PATH);
    }
}
