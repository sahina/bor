use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let base_url = "http://localhost:3000";
    let client = httpc_test::new_client(base_url)?;

    // no cookies in response
    client.do_get("/hello2/marge").await?.print().await?;

    // static file
    client.do_get("/src/main.rs").await?.print().await?;

    // sets cookie in success
    let req_login = client.do_post(
        "/api/login",
        json!(
            {"username": "demo", "pwd": "demo"}
        ),
    );
    req_login.await?.print().await?;

    client.do_delete("/api/tickets/1").await?.print().await?;

    let req_create = client.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket AAA"
        }),
    );
    req_create.await?.print().await?;

    client.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
