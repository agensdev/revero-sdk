use revero::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_oauth_get_oauth_authorization_server_metadata_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        username: Some("<username>".to_string()),
        password: Some("<password>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = GeneratedReveroClient::new(config).expect("Failed to build client");

    let result = client
        .oauth
        .get_oauth_authorization_server_metadata(None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/.well-known/oauth-authorization-server/oauth/partner",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_oauth_exchange_client_credentials_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        username: Some("<username>".to_string()),
        password: Some("<password>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = GeneratedReveroClient::new(config).expect("Failed to build client");

    let result = client
        .oauth
        .exchange_client_credentials(
            &ExchangeClientCredentialsRequest {
                grant_type: ExchangeClientCredentialsRequestGrantType::ClientCredentials,
                resource: "https://referat-api.flyt.test/public/v1".to_string(),
                scope: "meetings:read meetings:write recordings:write".to_string(),
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/oauth/partner/token", None, 1)
        .await
        .unwrap();
}
