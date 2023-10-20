pub trait Summarize {
    type Err;

    fn count(&self) -> Result<i64, Self::Err>;
}

#[derive(Debug, thiserror::Error)]
pub enum JsonSummarizeError {
    #[error("Number error")]
    NumberError,
}

impl Summarize for serde_json::Value {
    type Err = JsonSummarizeError;

    fn count(&self) -> Result<i64, JsonSummarizeError> {
        fn count_values<'a>(
            nums: impl Iterator<Item = &'a serde_json::Value>,
        ) -> Result<i64, JsonSummarizeError> {
            let ret = nums
                .map(|v| v.count())
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .sum();
            Ok(ret)
        }

        let ret = match self {
            serde_json::Value::Number(n) => n.as_i64().ok_or(JsonSummarizeError::NumberError)?,
            serde_json::Value::Array(v) => count_values(v.iter())?,
            serde_json::Value::Object(o) => count_values(o.values())?,

            serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::String(_) => {
                0
            }
        };
        Ok(ret)
    }
}
