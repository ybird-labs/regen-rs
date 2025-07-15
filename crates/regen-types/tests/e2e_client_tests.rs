use regen_types::{
    client::{Client, ClientConfigBuilder},
    types,
};
use std::time::Duration;

// Regen Network mainnet public RPC endpoints
const MAINNET_GRPC: &str = "http://public-rpc.regen.vitwit.com:9090";
const MAINNET_CHAIN_ID: &str = "regen-1";

#[tokio::test]
#[ignore] // Run with: cargo test --ignored
async fn test_mainnet_connection() {
    let config = ClientConfigBuilder::new()
        .grpc_endpoint(MAINNET_GRPC)
        .chain_id(MAINNET_CHAIN_ID)
        .timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(5))
        .build();

    let client = Client::new(config, None).await;

    match client {
        Ok(_) => println!("Successfully connected to Regen mainnet"),
        Err(e) => panic!("Failed to connect: {e:?}"),
    }
}

#[tokio::test]
#[ignore]
async fn test_mainnet_with_test_signer() {
    use regen_types::SignerBuilder;

    // Test mnemonic - DO NOT USE WITH REAL FUNDS
    let test_mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    // Note: For address generation, use "regen" not "regen-1"
    let signer = SignerBuilder::new("regen")
        .mnemonic(test_mnemonic)
        .build()
        .expect("Failed to create signer");

    let config = ClientConfigBuilder::new()
        .grpc_endpoint(MAINNET_GRPC)
        .chain_id(MAINNET_CHAIN_ID)
        .build();

    let client = Client::new(config, Some(signer))
        .await
        .expect("Failed to create client with signer");

    assert!(client.signer.is_some());
}

#[tokio::test]
#[ignore]
async fn test_query_all_credit_classes() {
    println!("Testing query for all credit classes on Regen mainnet");

    let config = ClientConfigBuilder::new()
        .grpc_endpoint(MAINNET_GRPC)
        .chain_id(MAINNET_CHAIN_ID)
        .timeout(Duration::from_secs(10))
        .build();

    let client = Client::new(config, None)
        .await
        .expect("Failed to create client");

    // Create ecocredit v1 query client
    let mut ecocredit_client =
        types::regen::ecocredit::v1::query_client::QueryClient::new(client.channel.clone());

    // Create request to query all credit classes
    let request = types::regen::ecocredit::v1::QueryClassesRequest {
        pagination: None, // Get all classes without pagination
    };

    println!("Querying all credit classes...");
    let response = ecocredit_client
        .classes(request)
        .await
        .expect("Failed to query credit classes");

    let classes = response.into_inner().classes;

    println!("Found {} credit classes:", classes.len());

    // Print details of each credit class
    for (i, class) in classes.iter().enumerate() {
        println!("  {}. Class ID: {}", i + 1, class.id);
        println!("     Admin: {}", class.admin);
        println!("     Credit Type: {}", class.credit_type_abbrev);
        println!("     Metadata: {}", class.metadata);
        println!();
    }

    // Verify we got some classes back
    assert!(
        !classes.is_empty(),
        "Expected to find at least one credit class"
    );

    // Verify each class has required fields
    for class in &classes {
        assert!(!class.id.is_empty(), "Class ID should not be empty");
        assert!(!class.admin.is_empty(), "Admin should not be empty");
        assert!(
            !class.credit_type_abbrev.is_empty(),
            "Credit type should not be empty"
        );
    }

    println!("All credit classes validated successfully!");
}

#[tokio::test]
#[ignore]
async fn test_query_credit_classes_with_pagination() {
    println!("Testing query for credit classes with pagination");

    let config = ClientConfigBuilder::new()
        .grpc_endpoint(MAINNET_GRPC)
        .chain_id(MAINNET_CHAIN_ID)
        .timeout(Duration::from_secs(10))
        .build();

    let client = Client::new(config, None)
        .await
        .expect("Failed to create client");

    // Create ecocredit v1 query client
    let mut ecocredit_client =
        types::regen::ecocredit::v1::query_client::QueryClient::new(client.channel.clone());

    // Create request with pagination (limit to 5 classes per page)
    let request = types::regen::ecocredit::v1::QueryClassesRequest {
        pagination: Some(
            cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest {
                key: vec![],
                offset: 0,
                limit: 5,
                count_total: true,
                reverse: false,
            },
        ),
    };

    println!("Querying credit classes with pagination (limit: 5)...");
    let response = ecocredit_client
        .classes(request)
        .await
        .expect("Failed to query credit classes");

    let response = response.into_inner();
    let classes = response.classes;

    println!("Found {} credit classes on this page:", classes.len());

    if let Some(pagination) = response.pagination {
        println!("Total count: {:?}", pagination.total);
        println!("Next key: {:?}", pagination.next_key);
    }

    // Print details of each credit class
    for (i, class) in classes.iter().enumerate() {
        println!("  {}. Class ID: {}", i + 1, class.id);
        println!("     Admin: {}", class.admin);
        println!("     Credit Type: {}", class.credit_type_abbrev);
        println!();
    }

    // Verify we got some classes back (should be <= 5 due to pagination)
    assert!(
        classes.len() <= 5,
        "Should get at most 5 classes due to pagination"
    );

    println!("Paginated credit classes query completed successfully!");
}
