use anyhow::{anyhow, Result, Context};

use ic_web3::{types::{H256, TransactionReceipt}, Web3, transports::ICHttp};
use ic_cdk::api::time;

const TX_SUCCESS_STATUS: u64 = 1;

#[inline]
pub fn time_in_seconds() -> u64 {
    time() / 1_000_000_000
}

pub async fn wait_for_success_confirmation(
    w3: &Web3<ICHttp>,
    tx_hash: &H256,
    timeout: u64,
) -> Result<TransactionReceipt> {
    let receipt = wait_for_confirmation(w3, tx_hash, timeout)
        .await?;

    let tx_status = receipt
        .status
        .expect("tx should be confirmed")
        .as_u64();

    if tx_status != TX_SUCCESS_STATUS {
        return Err(anyhow!("tx has failed"));
    } 

    Ok(receipt)
}

pub async fn wait_for_confirmation(
    w3: &Web3<ICHttp>,
    tx_hash: &H256,
    timeout: u64,
) -> Result<TransactionReceipt> {
    let end_time =  time_in_seconds() + timeout;

    while time_in_seconds() < end_time {
        let tx_receipt = w3
            .eth()
            .transaction_receipt(*tx_hash)
            .await
            .context("failed to get a tx receipt")?;

        if let Some(tx_receipt) = tx_receipt {
            if let Some(_) = tx_receipt.status {
                return Ok(tx_receipt);
            }
        }
    }

    Err(anyhow!("tx timeout"))
}