use async_graphql_test::Application;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Application::new();
    app.run_until_stopped().await?;
    Ok(())
}
