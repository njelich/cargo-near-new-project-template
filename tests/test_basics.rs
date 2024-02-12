use std::path::PathBuf;

use serde_json::json;

#[tokio::test]
async fn test_contract_is_operational() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox()
        .rpc_addr("http://localhost:3030")
        .validator_key(near_workspaces::network::ValidatorKey::HomeDir(
            PathBuf::from("/home/noah/.near"),
        ))
        .await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;

    let contract = sandbox.dev_deploy(&contract_wasm).await?;

    let user1_account = sandbox.dev_create_account().await?;
    let user2_account = sandbox.dev_create_account().await?;

    let outcome = user1_account
        .call(contract.id(), "set_status")
        .args_json(json!({"message": "test status"}))
        .transact()
        .await?;

    assert!(outcome.is_success());

    let user1_message_outcome = contract
        .view("get_status")
        .args_json(json!({"account_id": user1_account.id()}))
        .await?;
    assert_eq!(user1_message_outcome.json::<String>()?, "test status");

    let user2_message_outcome = contract
        .view("get_status")
        .args_json(json!({"account_id": user2_account.id()}))
        .await?;
    assert_eq!(user2_message_outcome.result, b"null");

    Ok(())
}
