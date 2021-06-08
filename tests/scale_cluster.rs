pub mod common;
mod test;

use crate::common::four_letter_commands::{send_four_letter_word, ARE_YOU_OK, I_AM_OK};
use crate::common::setup::{
    build_test_cluster_definition_with_node_id_label_selector, TestCluster,
};

use anyhow::Result;
use kube::api::ObjectList;
use stackable_zookeeper_crd::ZookeeperVersion;
use test::prelude::*;

#[test]
fn test_scale_cluster_up() -> Result<()> {
    let mut cluster = TestCluster::new();
    let version = ZookeeperVersion::v3_4_14;
    let name = "simple";

    let available_nodes: ObjectList<Node> = cluster.client.list_labeled("");
    let available_node_count = available_nodes.items.len();
    let expected_start_pod_count: usize = 1;

    // we assume at least 3 nodes here and want to scale from 1 to 3 pods. Needs improvement.
    let test_def = build_test_cluster_definition_with_node_id_label_selector(
        name,
        &version,
        expected_start_pod_count,
    );

    cluster.create_or_update(test_def, 100, expected_start_pod_count)?;

    let created_pods: ObjectList<Pod> = cluster
        .client
        .list_labeled("app.kubernetes.io/name=zookeeper");

    assert_eq!(expected_start_pod_count, created_pods.items.len());

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

        common::checks::check_config_map(&cluster.client, pod, expected_start_pod_count)?;
    }

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

        common::checks::check_config_map(&cluster.client, pod, available_node_count)?;
    }

    cluster.delete()
}

#[test]
fn test_scale_cluster_down() -> Result<()> {
    let mut cluster = TestCluster::new();
    let version = ZookeeperVersion::v3_4_14;
    let name = "simple";

    let available_nodes: ObjectList<Node> = cluster.client.list_labeled("");
    let available_node_count = available_nodes.items.len();
    let expected_scale_down_pod_count: usize = 1;

    // we assume at least 3 nodes here and want to scale from 1 to 3 pods. Needs improvement.
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

        common::checks::check_config_map(&cluster.client, pod, available_node_count)?;
    }

    let test_def = build_test_cluster_definition_with_node_id_label_selector(
        name,
        &version,
        expected_scale_down_pod_count,
    );

    cluster.create_or_update(test_def, 100, expected_scale_down_pod_count)?;

    let created_pods: ObjectList<Pod> = cluster
        .client
        .list_labeled("app.kubernetes.io/name=zookeeper");

    assert_eq!(expected_scale_down_pod_count, created_pods.items.len());

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

        common::checks::check_config_map(&cluster.client, pod, expected_scale_down_pod_count)?;
    }

    cluster.delete()
}
