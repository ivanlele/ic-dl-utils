#[macro_export]
macro_rules! retry_until_success {
    ($func:expr) => {{
        let mut result = $func.await;
        while result.is_err() && format!("{:?}", result.as_ref().unwrap_err()).contains("The http_request resulted into error. RejectionCode: SysTransient, Error: Canister http responses were different across replicas, and no consensus was reached") {
            result = $func.await;
        }

        result
    }}
}

#[cfg(test)]
mod tests {
    use crate::retry_until_success;

    use anyhow::Result;

    async fn test_func() -> Result<()> {
        return Ok(())
    }

    #[tokio::test]
    async fn it_works() {
        let _ = retry_until_success!(test_func());
    }
}