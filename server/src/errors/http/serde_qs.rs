use super::HttpError;
use serde_json::json;
use serde_qs::Error as SerdeQsError;

impl From<SerdeQsError> for HttpError {
  fn from(qs_error: SerdeQsError) -> Self {
    match qs_error {
      SerdeQsError::Custom(message) => {
        HttpError::bad_request("IVQCS", Some(json!({ "message": message })))
      }
      SerdeQsError::Parse(message, pos) => HttpError::bad_request(
        "IVQPS",
        Some(json!({
            "message": message,
            "pos": pos
        })),
      ),
      SerdeQsError::Unsupported => HttpError::bad_request("IVQUS", None),
      SerdeQsError::FromUtf8(utf8_error) => HttpError::bad_request(
        "IVQU8",
        Some(json!({
            "message": "Invalid UTF8 string",
            "validUpTo": utf8_error.utf8_error().valid_up_to(),
            "length": utf8_error.utf8_error().error_len(),
        })),
      ),
      SerdeQsError::Io(_) => HttpError::bad_request("IVQIO", None),
      SerdeQsError::ParseInt(error) => HttpError::bad_request(
        "IVQPI",
        Some(json!({
            "message": error.to_string(),
        })),
      ),
      SerdeQsError::Utf8(error) => HttpError::bad_request(
        "IVQU8",
        Some(json!({
            "message": "Invalid UTF8 string",
            "validUpTo": error.valid_up_to(),
            "length": error.error_len(),
        })),
      ),
    }
  }
}
