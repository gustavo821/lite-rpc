use bench::helpers::BenchHelper;
use lite_rpc::DEFAULT_LITE_RPC_ADDR;
use log::info;
use solana_rpc_client::{nonblocking::rpc_client::RpcClient, rpc_client::SerializableTransaction};
use solana_sdk::commitment_config::CommitmentConfig;

#[tokio::test]
async fn send_and_confirm_txs_get_signature_statuses() {
    tracing_subscriber::fmt::init();

    let rpc_client = RpcClient::new(DEFAULT_LITE_RPC_ADDR.to_string());

    let funded_payer = BenchHelper::get_payer().await.unwrap();
    let blockhash = rpc_client.get_latest_blockhash().await.unwrap();

    let tx = &BenchHelper::generate_txs(1, &funded_payer, blockhash)[0];
    let sig = tx.get_signature();

    rpc_client.send_transaction(tx).await.unwrap();
    info!("{sig}");

    BenchHelper::wait_till_signature_status(&rpc_client, sig, CommitmentConfig::confirmed())
        .await
        .unwrap();
}

#[tokio::test]
async fn send_and_confirm_tx_rpc_client() {
    let rpc_client = RpcClient::new(DEFAULT_LITE_RPC_ADDR.to_string());

    let funded_payer = BenchHelper::get_payer().await.unwrap();
    let blockhash = rpc_client.get_latest_blockhash().await.unwrap();

    let tx = &BenchHelper::generate_txs(1, &funded_payer, blockhash)[0];
    let sig = tx.get_signature();

    rpc_client.send_and_confirm_transaction(tx).await.unwrap();

    info!("Sent and Confirmed {sig}");
}
