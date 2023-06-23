#[macro_export]
macro_rules! retry_until_success {
    ($func:expr) => {{
        let mut attempts = 1;
        let mut result = $func.await;

        let (func_name, _) = stringify!($func).rsplit_once("(").unwrap();

        while result.is_err() 
            && (format!("{:?}", result.as_ref().unwrap_err()).contains("The http_request resulted into error. RejectionCode: SysTransient, Error: Canister http responses were different across replicas, and no consensus was reached")
            || format!("{:?}", result.as_ref().unwrap_err()).contains("insufficient funds for gas * price + value")) {
            result = $func.await;
            ic_cdk::println!("[{func_name}] use attempts: {attempts}");
            attempts += 1;
        }        

        ic_utils::logger::log_message(format!("[{func_name}] used attempts: {attempts}"));
        ic_cdk::println!("[{func_name}] used attempts: {attempts}");

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