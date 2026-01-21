use tilt_sdk::ClientBuilder;
use tilt_sdk_cloudengine::ComputeClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::new()
        .base_url("https://api.t1.cloud")
        .token("your-token")
        .project("your-project")
        .build()?;

    let compute = ComputeClient::new(&client);
    let instances = compute.list_instances(None, None).await?;
    
    for instance in instances {
        println!("{}: {:?}", instance.name, instance.status);
    }
    
    Ok(())
}