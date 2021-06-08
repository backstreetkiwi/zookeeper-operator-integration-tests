pub mod common;
mod test;

use crate::common::four_letter_commands::{send_four_letter_word, ARE_YOU_OK, I_AM_OK};
use crate::common::setup::{build_test_cluster_definition_with_label_selector, TestCluster};

use anyhow::Result;
use kube::api::ObjectList;
use stackable_zookeeper_crd::ZookeeperVersion;
use std::thread;
use std::time::Duration;
use test::prelude::*;

#[test]
fn test_cluster_update() -> Result<()> {
    let mut cluster = TestCluster::new();
    let name = "simple";
    let version_3_4_14 = ZookeeperVersion::v3_4_14;
    let version_3_5_8 = ZookeeperVersion::v3_5_8;

    let available_nodes: ObjectList<Node> = cluster.client.list_labeled("");
    let available_node_count = available_nodes.items.len();

    let test_def_3_4_14 = build_test_cluster_definition_with_label_selector(
        name,
        &version_3_4_14,
        available_node_count as u16,
        1,
    );

    cluster.create_or_update(test_def_3_4_14, 100, available_node_count)?;

    let pods = cluster.get_current_pods();
    assert_eq!(available_node_count, pods.len());
    check_pod_version(&version_3_4_14, pods.as_slice());

    for pod in pods {
        assert_eq!(
            send_four_letter_word(
                &version_3_4_14,
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

    // start update to version 3.5.8
    let test_def_3_5_8 = build_test_cluster_definition_with_label_selector(
        name,
        &version_3_5_8,
        available_node_count as u16,
        1,
    );

    cluster.create_or_update(test_def_3_5_8, 100, available_node_count)?;

    let pods = cluster.get_current_pods();
    assert_eq!(available_node_count, pods.len());
    check_pod_version(&version_3_5_8, pods.as_slice());

    // give some time for the webserver to be initialized
    // otherwise we receive
    thread::sleep(Duration::from_secs(2));

    for pod in pods {
        // Starting from version 3.5.3 we query the admin server for the 4 letter word
        // It is strange, but the correct answer for "ruok" will be "ruok" not "imok".
        assert_eq!(
            send_four_letter_word(
                &version_3_5_8,
                ARE_YOU_OK,
                &format!(
                    "{}:{}",
                    pod.spec.as_ref().unwrap().node_name.as_ref().unwrap(),
                    8080
                ),
            )
            .unwrap(),
            ARE_YOU_OK.to_string()
        );
    }

    cluster.delete()
}

fn check_pod_version(version: &ZookeeperVersion, pods: &[Pod]) {
    for pod in pods {
        if let Some(labels) = &pod.metadata.labels {
            let pod_version = labels.get(stackable_operator::labels::APP_VERSION_LABEL);
            assert_eq!(Some(&version.to_string()), pod_version);
        }
    }
}
