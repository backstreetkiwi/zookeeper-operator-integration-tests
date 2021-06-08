use crate::test::kube::TestKubeClient;
use crate::test::prelude::Pod;

use anyhow::{anyhow, Result};
use k8s_openapi::api::core::v1::ConfigMap;

/// Perform checks on configmaps for:
/// - server.<id> property set correctly (especially with scale up / down)
pub fn check_config_map(
    client: &TestKubeClient,
    pod: &Pod,
    expected_server_count: usize,
) -> Result<()> {
    let config_cm_name = get_config_cm(pod)?;
    let config_map: Option<ConfigMap> = client.find(&config_cm_name);

    check_for_server_id_property_count(config_map, expected_server_count)
}

/// This is a simple check for the correctness of the server property in config maps.
/// Every known server will be registered like:
/// server.1 = some_url
/// server.2 = another_url
/// If pods crash or scaling appears we have to make sure that the config maps
/// and pods are updated / restarted in order to contain the correct state of the cluster.
fn check_for_server_id_property_count(
    cm: Option<ConfigMap>,
    expected_server_count: usize,
) -> Result<()> {
    let mut server_count: usize = 0;
    if let Some(config_map) = cm {
        let data = config_map.data.unwrap();

        // TODO: This might interfere with other properties having the "server."
        //    string contained. Needs more stable solution.
        if let Some(value) = data.get("zoo.cfg") {
            server_count = value.matches("server.").count();
        }

        if server_count == expected_server_count {
            return Ok(());
        }
    }

    Err(anyhow!(
        "ConfigMap server.<id> properties [{}] do not match the expected number of server.<id> properties [{}]",
        server_count, expected_server_count
    ))
}

/// Extracts the name of the "config" configmap of a pod.
fn get_config_cm(pod: &Pod) -> Result<String> {
    let volumes = pod.spec.as_ref().unwrap().volumes.as_ref().unwrap();
    let mut cm_name: &str = "";
    let pod_name = pod.metadata.name.as_ref().unwrap();

    for volume in volumes {
        cm_name = volume.config_map.as_ref().unwrap().name.as_ref().unwrap();

        // TODO: use create_config_map_name if available in zookeeper
        if cm_name == &format!("{}-config", pod_name) {
            return Ok(cm_name.to_string());
        }
    }

    Err(anyhow!(
        "Could not find config map [{}] for pod [{}]",
        cm_name,
        pod_name
    ))
}
