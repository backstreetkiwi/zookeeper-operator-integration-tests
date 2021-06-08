pub mod common;
mod test;

use crate::common::four_letter_commands::{send_four_letter_word, ARE_YOU_OK, I_AM_OK};
use crate::common::setup::{
    build_test_cluster_definition_with_label_selector,
    build_test_cluster_definition_with_node_id_label_selector, TestCluster,
};

use anyhow::Result;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::api::ObjectList;
use kube::Resource;
use stackable_zookeeper_crd::ZookeeperVersion;
use test::prelude::*;

#[test]
fn test_crd_created() {
    let client = TestKubeClient::new();

    let crd: CustomResourceDefinition = client
        .find_crd("zookeeperclusters.zookeeper.stackable.tech")
        .unwrap();

    assert_eq!(
        "zookeeperclusters.zookeeper.stackable.tech".to_string(),
        crd.name()
    );
}

#[test]
fn test_create_cluster_with_label_selector() -> Result<()> {
    let mut cluster = TestCluster::new();
    let version = ZookeeperVersion::v3_4_14;
    let name = "simple";

    let available_nodes: ObjectList<Node> = cluster.client.list_labeled("");
    let available_node_count = available_nodes.items.len();

    let test_def = build_test_cluster_definition_with_label_selector(
        name,
        &version,
        available_node_count as u16,
        1,
    );

    cluster.create_or_update(test_def, 100, available_node_count)?;

    let created_pods: ObjectList<Pod> = cluster
        .client
        .list_labeled("app.kubernetes.io/name=zookeeper");

    assert_eq!(available_node_count, created_pods.items.len());

    for pod in &created_pods {
        cluster.client.verify_pod_condition(pod, "Ready");

        assert_eq!(
            send_four_letter_word(
                &version,
                ARE_YOU_OK,
                &format!(
                    "{}:{}",
                    pod.spec.as_ref().unwrap().node_name.as_ref().unwrap(),
                    2181
                ),
            )
            .unwrap(),
            I_AM_OK.to_string()
        );
    }

    cluster.delete()
}

#[test]
fn test_create_cluster_with_node_ids() -> Result<()> {
    let mut cluster = TestCluster::new();
    let version = ZookeeperVersion::v3_4_14;
    let name = "simple";

    let available_nodes: ObjectList<Node> = cluster.client.list_labeled("");
    let available_node_count = available_nodes.items.len();

    let test_def = build_test_cluster_definition_with_node_id_label_selector(
        name,
        &version,
        available_node_count,
    );

    cluster.create_or_update(test_def, 100, available_node_count)?;

    let created_pods: ObjectList<Pod> = cluster
        .client
        .list_labeled("app.kubernetes.io/name=zookeeper");

    assert_eq!(available_node_count, created_pods.items.len());

    for pod in &created_pods {
        cluster.client.verify_pod_condition(pod, "Ready");

        assert_eq!(
            send_four_letter_word(
                &version,
                ARE_YOU_OK,
                &format!(
                    "{}:{}",
                    pod.spec.as_ref().unwrap().node_name.as_ref().unwrap(),
                    2181
                ),
            )
            .unwrap(),
            I_AM_OK.to_string()
        );
    }

    cluster.delete()
}
