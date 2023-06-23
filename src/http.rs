#[macro_export]
macro_rules! retry_until_success {
    ($func:expr) => {{
        let mut attempts = 1;
        let mut result = $func.await;        

        while result.is_err() 
            && format!("{:?}", result.as_ref().unwrap_err()).contains("Canister http responses were different across replicas") {
            result = $func.await;
            attempts += 1;
        }        

        let (func_name, _) = stringify!($func).rsplit_once("(").unwrap();

        ic_utils::logger::log_message(format!("[{func_name}] used attempts: {attempts}"));

        result
    }}
}

#[macro_export]
macro_rules! retry_with_unhandled {
    ($func:expr) => {{
        let mut attempts = 1;
        let mut result = $func.await;        

        let mut is_unhandled_error = false;

        while result.is_err() { 
            let format_error = format!("{:?}", result.as_ref().unwrap_err());
            if format_error.contains("insufficient funds for gas * price + value") {
                is_unhandled_error = true;
                break;
            }

            if format_error.contains("Canister http responses were different across replicas") {
                result = $func.await;
                attempts += 1;
                continue;
            }

            break;
        }        

        let (func_name, _) = stringify!($func).rsplit_once("(").unwrap();

        ic_utils::logger::log_message(format!("[{func_name}] used attempts: {attempts}"));

        (result, is_unhandled_error)
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
        let (_, _) = retry_with_unhandled!(test_func());
    }
    
}