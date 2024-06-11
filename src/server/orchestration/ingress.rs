use k8s_openapi::api::networking::v1::{Ingress, IngressSpec, IngressStatus, HTTPIngressPath, HTTPIngressRuleValue, IngressRule, IngressBackend};
use kube::{Client, Api};
use kube::api::{PostParams, ListParams, DeleteParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;

    let ingresses: Api<Ingress> = Api::namespaced(client, "default");

    let ing = Ingress {
        metadata: Default::default(),
        spec: Some(IngressSpec {
            rules: Some(vec![IngressRule {
                host: Some("neuraserver.local".to_string()),
                http: Some(HTTPIngressRuleValue {
                    paths: vec![HTTPIngressPath {
                        path: Some("/".to_string()),
                        backend: IngressBackend {
                            service: Some(k8s_openapi::api::networking::v1::IngressServiceBackend {
                                name: "neuraserver-service".to_string(),
                                port: Some(k8s_openapi::api::networking::v1::ServiceBackendPort {
                                    number: Some(80),
                                    ..Default::default()
                                }),
                            }),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                }),
                ..Default::default()
            }]),
            ..Default::default()
        }),
        status: Some(IngressStatus {
            ..Default::default()
        }),
    };

    ingresses.create(&PostParams::default(), &ing).await?;
    println!("Ingress created!");

    Ok(())
}
