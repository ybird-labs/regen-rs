use regen_types::client::{Client, ClientConfig};
use regen_types::regen::data::v2::QueryAnchorByIriRequest;
use regen_types::regen::ecocredit::v1::{QueryClassRequest, QueryClassesRequest};
use std::time::Duration;

const TEST_GRPC_ENDPOINT: &str = "http://public-rpc.regen.vitwit.com:9090";

#[tokio::test]
#[ignore]
async fn test_data_client_query_anchor_by_iri() {
    let config = ClientConfig {
        grpc_endpoint: TEST_GRPC_ENDPOINT.to_string(),
        timeout: Duration::from_secs(10),
        connect_timeout: Duration::from_secs(5),
        ..Default::default()
    };
    let client = Client::new(config, None)
        .await
        .expect("Failed to create client");

    // Test anchor_by_iri with a known IRI
    let request = QueryAnchorByIriRequest {
        iri: "regen:13toVfvC2YxrrfSXWB5h2BGHiXZURsKxWUz72uDRDSPMCrYPguGUXSC.rdf".to_string(),
    };

    let response = client.data().query().anchor_by_iri(request).await;
    // This might fail if the IRI doesn't exist, but tests the connection
    println!("Anchor by IRI response: {:?}", response);
}

#[tokio::test]
#[ignore]
async fn test_ecocredit_client_query_classes() {
    let config = ClientConfig {
        grpc_endpoint: TEST_GRPC_ENDPOINT.to_string(),
        timeout: Duration::from_secs(10),
        connect_timeout: Duration::from_secs(5),
        ..Default::default()
    };
    let client = Client::new(config, None)
        .await
        .expect("Failed to create client");

    // Test query classes without pagination
    let request = QueryClassesRequest { pagination: None };

    let response = client.eco_credit().query().classes(request).await;
    assert!(
        response.is_ok(),
        "Query classes failed: {:?}",
        response.err()
    );

    let classes = response.unwrap().into_inner();
    println!("Found {} credit classes", classes.classes.len());
    assert!(
        !classes.classes.is_empty(),
        "Should have at least one credit class"
    );
}

#[tokio::test]
#[ignore]
async fn test_ecocredit_client_query_class() {
    let config = ClientConfig {
        grpc_endpoint: TEST_GRPC_ENDPOINT.to_string(),
        timeout: Duration::from_secs(10),
        connect_timeout: Duration::from_secs(5),
        ..Default::default()
    };
    let client = Client::new(config, None)
        .await
        .expect("Failed to create client");

    // First get some classes to have a valid class_id
    let classes_response = client
        .eco_credit()
        .query()
        .classes(QueryClassesRequest { pagination: None })
        .await;

    if let Ok(classes) = classes_response {
        let classes = classes.into_inner();
        if let Some(first_class) = classes.classes.first() {
            // Query specific class info
            let request = QueryClassRequest {
                class_id: first_class.id.clone(),
            };

            let response = client.eco_credit().query().class(request).await;
            assert!(response.is_ok(), "Query class failed: {:?}", response.err());

            let class_info = response.unwrap().into_inner();
            assert!(class_info.class.is_some(), "Class should exist");

            let class = class_info.class.unwrap();
            assert_eq!(class.id, first_class.id, "Class ID should match");
            println!("Class: {} - Admin: {}", class.id, class.admin);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_multiple_clients_concurrent() {
    // Test that multiple clients can work concurrently
    let data_future = async {
        let config = ClientConfig {
            grpc_endpoint: TEST_GRPC_ENDPOINT.to_string(),
            ..Default::default()
        };
        let client = Client::new(config, None)
            .await
            .expect("Failed to create client");
        let request = QueryAnchorByIriRequest {
            iri: "regen:13toVfvC2YxrrfSXWB5h2BGHiXZURsKxWUz72uDRDSPMCrYPguGUXSC.rdf".to_string(),
        };
        client.data().query().anchor_by_iri(request).await
    };

    let ecocredit_future = async {
        let config = ClientConfig {
            grpc_endpoint: TEST_GRPC_ENDPOINT.to_string(),
            ..Default::default()
        };
        let client = Client::new(config, None)
            .await
            .expect("Failed to create client");
        client
            .eco_credit()
            .query()
            .classes(QueryClassesRequest { pagination: None })
            .await
    };

    let (data_result, ecocredit_result) = tokio::join!(data_future, ecocredit_future);

    // Both should at least connect successfully
    println!("Data result: {:?}", data_result);
    println!("Ecocredit result: {:?}", ecocredit_result);
}
