use anyhow::{anyhow, Result, Context};

use ic_cdk::{api::management_canister::http_request::{TransformContext, TransformFunc}, export::candid};
use ic_web3_rs::{Transport, types::{H256, TransactionReceipt}, Web3, transports::ic_http_client::CallOptionsBuilder};

use crate::{time::time_in_seconds, retry_until_success};

const TX_SUCCESS_STATUS: u64 = 1;

pub async fn wait_for_success_confirmation<T: Transport>(
    w3: &Web3<T>,
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

pub async fn wait_for_confirmation<T: Transport>(
    w3: &Web3<T>,
    tx_hash: &H256,
    timeout: u64,
) -> Result<TransactionReceipt> {
    let call_opts = CallOptionsBuilder::default()
    .transform(Some(TransformContext {
        function: TransformFunc(candid::Func {
            principal: ic_cdk::api::id(),
            method: "transform".into(),
        }),
        context: vec![],
    }))
    .cycles(None)
    .max_resp(None)

    .build()
    .expect("failed to build call options");
    
    let end_time =  time_in_seconds() + timeout;
    while time_in_seconds() < end_time {
        let tx_receipt = retry_until_success!(
            w3.eth().transaction_receipt(*tx_hash, call_opts.clone())
        )
            .context("failed to get a tx receipt")?;

        if let Some(tx_receipt) = tx_receipt {
            if let Some(_) = tx_receipt.status {
                return Ok(tx_receipt);
            }
        }
    }

    Err(anyhow!("tx timeout"))
}