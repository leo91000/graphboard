use crate::errors::HttpError;
use deadpool_postgres::PoolError;
use serde_json::json;

impl From<PoolError> for HttpError {
  fn from(pool_error: PoolError) -> Self {
    match pool_error {
      PoolError::Timeout(_) => HttpError::internal_server_error("POLTM", None),
      PoolError::Backend(ref error) => {
        #[cfg(not(debug_assertions))]
        return HttpError::internal_server_error("POLBK", None);
        #[cfg(debug_assertions)]
        HttpError::internal_server_error(
          "POLBK",
          Some(json!({
            "raw": error.to_string()
          })),
        )
      }
      PoolError::Closed => HttpError::internal_server_error("POLCL", None),
      PoolError::NoRuntimeSpecified => HttpError::internal_server_error("POLNR", None),
      PoolError::PostCreateHook(_) => HttpError::internal_server_error("POLCH", None),
      PoolError::PreRecycleHook(_) => HttpError::internal_server_error("POLEH", None),
      PoolError::PostRecycleHook(_) => HttpError::internal_server_error("POLOH", None),
    }
  }
}
