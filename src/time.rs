use ic_cdk::api::time;

#[inline]
pub fn time_in_seconds() -> u64 {
    time() / 1_000_000_000
}

pub fn wait(timeout: u64) {
    let end_time = time_in_seconds() + timeout;

    while time_in_seconds() < end_time {}
}