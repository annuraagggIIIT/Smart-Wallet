use ethers::prelude::*;
use ethers_contract::abigen;
use std::convert::TryInto;

abigen!(
    SmartWallet,
    r#"[
        function proposeNewOwner(address payable newOwner) public;
        function setAllowance(address _from, uint256 _amount) public;
        function denySending(address _from) public;
        function transfer(address payable _to, uint256 _amount, bytes memory payload) public returns (bytes memory);
        function owner() view returns (address);
        function allowance(address _from) view returns (uint256);
        function isAllowedToSend(address _from) view returns (bool);
    ]"#,
    event_derives(serde::Deserialize, serde::Serialize)
);

#[tokio::main]
async fn main() {

    let provider = Provider::<Http>::try_from("https://mainnet.infura.io/v3/YOUR_INFURA_API_KEY")
        .expect("Failed to create provider");

    let contract_address: Address =
        "SMARTWALLET_CONTRACT_ADDRESS".parse().expect("Failed to parse contract address");

    let smart_wallet = SmartWallet::new(contract_address, provider);

    // Example: Propose a new owner
    let new_owner: Address = "NEW_OWNER_ADDRESS".parse().expect("Failed to parse new owner address");
    let tx = smart_wallet.propose_new_owner(new_owner);
    println!("Propose new owner transaction hash: {:?}", tx.await);

    // Example: Set allowance
    let from_address: Address = "SENDER_ADDRESS".parse().expect("Failed to parse sender address");
    let allowance_amount: U256 = 100.into();
    let tx = smart_wallet.set_allowance(from_address, allowance_amount);
    println!("Set allowance transaction hash: {:?}", tx.await);

    // Example: Deny sending
    let tx = smart_wallet.deny_sending(from_address);
    println!("Deny sending transaction hash: {:?}", tx.await);

    // Example: Get owner
    let owner = smart_wallet.owner().call().await.expect("Failed to get owner");
    println!("Current owner: {:?}", owner);

    // Example: Get allowance
    let allowance = smart_wallet.allowance(from_address).call().await.expect("Failed to get allowance");
    println!("Allowance for sender: {:?}", allowance);

    // Example: Check if an address is allowed to send
    let is_allowed = smart_wallet.is_allowed_to_send(from_address).call().await.expect("Failed to check allowance");
    println!("Is sender allowed to send: {:?}", is_allowed);
}
