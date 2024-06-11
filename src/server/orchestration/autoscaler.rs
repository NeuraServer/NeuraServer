use k8s_openapi::api::autoscaling::v1::{HorizontalPodAutoscaler, HorizontalPodAutoscalerSpec, HorizontalPodAutoscalerStatus};
use kube::{Client, Api};
use kube::api::{PostParams, ListParams, DeleteParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;

    let hpas: Api<HorizontalPodAutoscaler> = Api::namespaced(client, "default");

    let hpa = HorizontalPodAutoscaler {
        metadata: Default::default(),
        spec: Some(HorizontalPodAutoscalerSpec {
            scale_target_ref: k8s_openapi::api::autoscaling::v1::CrossVersionObjectReference {
                api_version: Some("apps/v1".to_string()),
                kind: "Deployment".to_string(),
                name: "neuraserver-deployment".to_string(),
            },
            min_replicas: Some(1),
            max_replicas: 10,
            target_cpu_utilization_percentage: Some(50),
        }),
        status: Some(HorizontalPodAutoscalerStatus {
            ..Default::default()
        }),
    };

    hpas.create(&PostParams::default(), &hpa).await?;
    println!("Horizontal Pod Autoscaler created!");

    Ok(())
}
