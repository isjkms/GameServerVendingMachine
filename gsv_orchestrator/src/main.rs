use anyhow::Result;
use k8s_openapi::api::core::v1::Pod;
use kube::{api::ListParams, Api, Client};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Resources {
    cpu: String,
    memory: String,
}

#[derive(Debug, Deserialize)]
struct GameConfig {
    game: String,
    image: String,
    service_port: u16,
    pvc_size: String,
    resources: Resources,
}

#[tokio::main]
async fn main() -> Result<()> {
    // RUST_LOG=info,kube=warn 로 제어 가능

    let yaml_path = "../games/minecraft/config.yaml";
    let yaml_text = std::fs::read_to_string(yaml_path)?;
    let cfg: GameConfig = serde_yaml::from_str(&yaml_text)?;

    println!("config loaded OK: {:?}", cfg);
    println!(
        "game={} image={} port={} pvc={} cpu={} mem={}",
        cfg.game,
        cfg.image,
        cfg.service_port,
        cfg.pvc_size,
        cfg.resources.cpu,
        cfg.resources.memory
    );

    println!("gsv_orchestrator: hello k8s start");

    // kubeconfig 기반 자동 연결 (지금은 kind-gsvc)
    let client = Client::try_default().await?;
    println!("k8s client connected");

    // 전체 네임스페이스에서 Pod 조회(확실하게 보이게)
    let pods: Api<Pod> = Api::all(client);
    let lp: ListParams = ListParams::default().limit(20);

    let pod_list = pods.list(&lp).await?;
    println!("pods found: {}", pod_list.items.len());

    for p in pod_list.items {
        let name = p.metadata.name.unwrap_or_else(|| "<no-name>".to_string());
        let ns = p.metadata.namespace.unwrap_or_else(|| "<no-namespace>".to_string());
        println!("{}/{}", ns, name);
    }

    Ok(())
}
