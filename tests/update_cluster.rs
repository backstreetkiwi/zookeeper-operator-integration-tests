pub mod common;

use crate::common::checks::custom_checks;
use crate::common::zookeeper::{append_random_characters, build_test_cluster, build_zk_cluster};
use anyhow::Result;
use integration_test_commons::test::prelude::Pod;
use stackable_zookeeper_crd::ZookeeperVersion;
use std::thread;
use std::time::Duration;

#[test]
fn test_cluster_update() -> Result<()> {
    let name = append_random_characters("simple");
    let version = ZookeeperVersion::v3_4_14;
    let version_update = ZookeeperVersion::v3_5_8;
    let mut cluster = build_test_cluster();

    let (zookeeper_cr, expected_pod_count) = build_zk_cluster(&name, &version, 1)?;
    cluster.create_or_update(&zookeeper_cr, expected_pod_count)?;
    let created_pods = cluster.list_pods();

    custom_checks(
        &cluster.client,
        created_pods.as_slice(),
        &version,
        2181,
        expected_pod_count,
    )?;
    check_pod_version(&version, created_pods.as_slice());

    let (zookeeper_cr, expected_pod_count) = build_zk_cluster(&name, &version_update, 1)?;
    cluster.create_or_update(&zookeeper_cr, expected_pod_count)?;
    let created_pods = cluster.list_pods();

    custom_checks(
        &cluster.client,
        created_pods.as_slice(),
        &version_update,
        8080,
        expected_pod_count,
    )?;
    check_pod_version(&version_update, created_pods.as_slice());

    thread::sleep(Duration::from_secs(2));

    Ok(())
}

fn check_pod_version(version: &ZookeeperVersion, pods: &[Pod]) {
    for pod in pods {
        let pod_version = &pod
            .metadata
            .labels
            .get(stackable_operator::labels::APP_VERSION_LABEL);
        assert_eq!(&Some(&version.to_string()), pod_version);
    }
}
