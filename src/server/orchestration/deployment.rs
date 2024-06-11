use k8s_openapi::api::apps::v1::{Deployment, DeploymentSpec, DeploymentStatus};
use kube::{Client, Api};
use kube::api::{PostParams, ListParams, DeleteParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;

    let deployments: Api<Deployment> = Api::namespaced(client, "default");

    let dp = Deployment {
        metadata: Default::default(),
        spec: Some(DeploymentSpec {
            replicas: Some(3),
            ..Default::default()
        }),
        status: Some(DeploymentStatus {
            ..Default::default()
        }),
    };

    deployments.create(&PostParams::default(), &dp).await?;
    println!("Deployment created!");

    Ok(())
}
