#[allow(dead_code)]
const CONSENCUS_ERROR: &str = "The http_request resulted into error. RejectionCode: SysTransient, Error: Canister http responses were different across replicas, and no consensus was reached";

#[macro_export]
macro_rules! retry_until_success {
    ($func:expr, $($arg:expr),*) => {{
        let mut result = $func($($args),*);
        while result.is_err() && format!("{:?}", result.unwrap_err()).contains(CONSENCUS_ERROR) {
            let mut result = $func($($args),*);
        }

        result
    }};
}