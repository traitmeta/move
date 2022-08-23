use std::env;

use transactions::{
    accounts::account::Account,
    clients::{client::RestClient, faucet::FaucetClient, msg::MsgClient},
};

const TESTNET_URL: &str = "https://fullnode.devnet.aptoslabs.com/v1";
const FAUCET_URL: &str = "https://faucet.devnet.aptoslabs.com/";

fn main() -> () {
    msg_move_not_faucet()
}

fn msg_move_not_faucet() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    assert_eq!(
        args.len(),
        2,
        "Expecting an argument that points to the helloblockchain module"
    );

    let client = MsgClient::new(TESTNET_URL.to_string());
    // Create two accounts, Alice and Bob
    let key_bytes =
        hex::decode("633ccd9e612dbd8fad2112643e4d8b9aa30df089e1c10cc29f044fff863cdbcf").unwrap();
    println!("private:{:?}", key_bytes);
    let mut alice = Account::new(Some(key_bytes));

    println!("\n=== Addresses ===");
    println!("Alice: 0x{}", alice.address());

    println!("\n=== Balances ===");
    println!(
        "Alice: {:?}",
        client.rest_client.account_balance(&alice.address())
    );

    let module_path = args.get(1).unwrap();
    let module_hex = hex::encode(std::fs::read(module_path).unwrap());

    println!("\n=== Testing Alice ===");
    println!("Publishing...");
    let mut tx_hash = client.publish_module(&mut alice, &module_hex);
    client.rest_client.wait_for_transaction(&tx_hash);
    println!(
        "Initial value: {:?}",
        client.get_message(&alice.address(), &alice.address())
    );
    println!("Setting the message to \"Hello, Blockchain\"");
    tx_hash = client.set_message(&alice.address(), &mut alice, &"Hello, Blockchain");
    client.rest_client.wait_for_transaction(&tx_hash);
    println!(
        "New value: {:?}",
        client.get_message(&alice.address(), &alice.address())
    );
}

fn msg_move() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    assert_eq!(
        args.len(),
        2,
        "Expecting an argument that points to the helloblockchain module"
    );

    let client = MsgClient::new(TESTNET_URL.to_string());
    let faucet_client = FaucetClient::new(FAUCET_URL.to_string(), client.rest_client.clone());

    // Create two accounts, Alice and Bob
    let mut alice = Account::new(None);
    let mut bob = Account::new(None);

    println!("\n=== Addresses ===");
    println!("Alice: 0x{}", alice.address());
    println!("Bob: 0x{}", bob.address());

    faucet_client.fund_account(&alice.auth_key(), 5_000);
    faucet_client.fund_account(&bob.auth_key(), 5_000);

    println!("\n=== Initial Balances ===");
    println!(
        "Alice: {:?}",
        client.rest_client.account_balance(&alice.address())
    );
    println!(
        "Bob: {:?}",
        client.rest_client.account_balance(&bob.address())
    );

    println!("\nUpdate the module with Alice's address, build, copy to the provided path, and press enter.");
    match std::io::stdin().read_line(&mut String::new()) {
        Ok(_n) => {}
        Err(error) => println!("error: {}", error),
    }

    let module_path = args.get(1).unwrap();
    let module_hex = hex::encode(std::fs::read(module_path).unwrap());

    println!("\n=== Testing Alice ===");
    println!("Publishing...");
    let mut tx_hash = client.publish_module(&mut alice, &module_hex);
    client.rest_client.wait_for_transaction(&tx_hash);
    println!(
        "Initial value: {:?}",
        client.get_message(&alice.address(), &alice.address())
    );
    println!("Setting the message to \"Hello, Blockchain\"");
    tx_hash = client.set_message(&alice.address(), &mut alice, &"Hello, Blockchain");
    client.rest_client.wait_for_transaction(&tx_hash);
    println!(
        "New value: {:?}",
        client.get_message(&alice.address(), &alice.address())
    );

    println!("\n=== Testing Bob ===");
    println!(
        "Initial value: {:?}",
        client.get_message(&alice.address(), &bob.address())
    );
    println!(
        "Initial value: {:?}",
        client.get_message(&alice.address(), &bob.address())
    );
    println!("Setting the message to \"Hello, Blockchain\"");
    tx_hash = client.set_message(&alice.address(), &mut bob, &"Hello, Blockchain");
    client.rest_client.wait_for_transaction(&tx_hash);
    println!(
        "New value: {:?}",
        client.get_message(&alice.address(), &bob.address())
    );
}

fn first_move() {
    let rest_client = RestClient::new(TESTNET_URL.to_string());
    let faucet_client = FaucetClient::new(FAUCET_URL.to_string(), rest_client.clone());

    // Create two accounts, Alice and Bob, and fund Alice but not Bob
    let mut alice = Account::new(None);
    let bob = Account::new(None);

    println!("\n=== Addresses ===");
    println!("Alice: 0x{}", alice.address());
    println!("Bob: 0x{}", bob.address());

    faucet_client.fund_account(&alice.auth_key().as_str(), 1_000_000);
    faucet_client.fund_account(&bob.auth_key().as_str(), 0);

    println!("\n=== Initial Balances ===");
    println!("Alice: {:?}", rest_client.account_balance(&alice.address()));
    println!("Bob: {:?}", rest_client.account_balance(&bob.address()));

    // Have Alice give Bob 10 coins
    let tx_hash = rest_client.transfer(&mut alice, &bob.address(), 1_000);
    rest_client.wait_for_transaction(&tx_hash);

    println!("\n=== Final Balances ===");
    println!("Alice: {:?}", rest_client.account_balance(&alice.address()));
    println!("Bob: {:?}", rest_client.account_balance(&bob.address()));
}
