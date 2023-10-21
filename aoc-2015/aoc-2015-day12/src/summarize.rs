pub fn count(v: &serde_json::Value) -> Result<i64, JsonSummarizeError> {
    count_if(v, &mut |_| true)
}

// Take attention to parameter p. It takes &mut.
// It's same as if we passed &mut to a struct for example.
// It allows us to reborrow it in other functions.
pub fn count_if<F>(v: &serde_json::Value, p: &mut F) -> Result<i64, JsonSummarizeError>
where
    F: FnMut(&serde_json::Value) -> bool,
{
    if !p(v) {
        return Ok(0);
    }

    let ret = match &v {
        serde_json::Value::Number(n) => n.as_i64().ok_or(JsonSummarizeError::NumberError)?,
        serde_json::Value::Array(v) => count_values(v.iter(), p)?,
        serde_json::Value::Object(o) => count_values(o.values(), p)?,

        serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::String(_) => 0,
    };
    Ok(ret)
}

fn count_values<'a, F>(
    values: impl Iterator<Item = &'a serde_json::Value>,
    p: &mut F,
) -> Result<i64, JsonSummarizeError>
where
    F: FnMut(&serde_json::Value) -> bool,
{
    // Need to collect, because we can't call p in a chain.
    let ret = values.filter(|&v| p(v)).collect::<Vec<_>>();
    let ret = ret
        .into_iter()
        .map(|v| count_if(v, p))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum();
    Ok(ret)
}

#[derive(Debug, thiserror::Error)]
pub enum JsonSummarizeError {
    #[error("Number error")]
    NumberError,
}
