use k8s_openapi::api::core::v1::{Service, ServiceSpec, ServiceStatus};
use kube::{Client, Api};
use kube::api::{PostParams, ListParams, DeleteParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;

    let services: Api<Service> = Api::namespaced(client, "default");

    let svc = Service {
        metadata: Default::default(),
        spec: Some(ServiceSpec {
            ports: Some(vec![k8s_openapi::api::core::v1::ServicePort {
                port: 80,
                ..Default::default()
            }]),
            selector: Some([("app".to_string(), "neuraserver".to_string())].iter().cloned().collect()),
            ..Default::default()
        }),
        status: Some(ServiceStatus {
            ..Default::default()
        }),
    };

    services.create(&PostParams::default(), &svc).await?;
    println!("Service created!");

    Ok(())
}
