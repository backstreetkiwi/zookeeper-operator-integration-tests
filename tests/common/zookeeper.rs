use anyhow::Result;
use indoc::formatdoc;
use integration_test_commons::operator::setup::{
    TestCluster, TestClusterOptions, TestClusterTimeouts,
};
use stackable_zookeeper_crd::{ZookeeperCluster, ZookeeperVersion};
use std::time::Duration;
use uuid::Uuid;

/// Predefined options and timeouts for the TestCluster.
pub fn build_test_cluster() -> TestCluster<ZookeeperCluster> {
    TestCluster::new(
        TestClusterOptions {
            cluster_ready_condition_type: "Upgrading".to_string(),
            pod_name_label: "zookeeper".to_string(),
        },
        TestClusterTimeouts {
            cluster_ready: Duration::from_secs(300),
            pods_terminated: Duration::from_secs(30),
        },
    )
}

/// Used to create random cluster names. The full UUID is too long when combined in the pod
/// names (63 characters). So we just use a slice here to avoid conflicts with the names.
// TODO: unify naming - https://github.com/stackabletech/issues/issues/10
pub fn append_random_characters(name: &str) -> String {
    format!("{}-{}", name, Uuid::new_v4().as_fields().0)
}

/// This returns a ZooKeeper custom resource and the expected pod count (1). We use labels
/// for host_name and assign it to the node_ids provided by test-dev-cluster.
/// This creates 1 ZooKeeper server.
pub fn build_zk_custom_resource_1_server(
    name: &str,
    version: &ZookeeperVersion,
) -> Result<(ZookeeperCluster, usize)> {
    let spec = &formatdoc!(
        "
        apiVersion: zookeeper.stackable.tech/v1alpha1
        kind: ZookeeperCluster
        metadata:
          name: {}
        spec:
          version: {}
          servers:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    node: 1
                replicas: 1
    ",
        name,
        version.to_string()
    );

    Ok((serde_yaml::from_str(spec)?, 1))
}

/// This returns a ZooKeeper custom resource and the expected pod count (1). We use labels
/// for host_name and assign it to the node_ids provided by test-dev-cluster.
/// This creates 1 ZooKeeper server with a user defined client and metrics port.
pub fn build_zk_custom_resource_1_server_metrics_and_client_port(
    name: &str,
    version: &ZookeeperVersion,
    client_port: u16,
    metrics_port: u16,
) -> Result<(ZookeeperCluster, usize)> {
    let spec = &formatdoc!(
        "
        apiVersion: zookeeper.stackable.tech/v1alpha1
        kind: ZookeeperCluster
        metadata:
          name: {}
        spec:
          version: {}
          servers:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    node: 1
                replicas: 1
                config:
                  clientPort: {}
                  metricsPort: {}
    ",
        name,
        version.to_string(),
        client_port,
        metrics_port,
    );

    Ok((serde_yaml::from_str(spec)?, 1))
}

/// This returns a ZooKeeper custom resource and the expected pod count (3). We use labels
/// for host_name and assign it to the node_ids provided by test-dev-cluster.
/// This creates 1 ZooKeeper server.
pub fn build_zk_custom_resource_3_server(
    name: &str,
    version: &ZookeeperVersion,
) -> Result<(ZookeeperCluster, usize)> {
    let spec = &formatdoc!(
        "
        apiVersion: zookeeper.stackable.tech/v1alpha1
        kind: ZookeeperCluster
        metadata:
          name: {}
        spec:
          version: {}
          servers:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    node: 1
                replicas: 1
              default2:
                selector:
                  matchLabels:
                    node: 2
                replicas: 1
              default3:
                selector:
                  matchLabels:
                    node: 3
                replicas: 1
    ",
        name,
        version.to_string()
    );

    Ok((serde_yaml::from_str(spec)?, 3))
}
