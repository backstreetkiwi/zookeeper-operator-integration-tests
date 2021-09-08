pub mod common;

use crate::common::checks::custom_checks;
use crate::common::zookeeper::{append_random_characters, build_test_cluster, build_zk_cluster};

use anyhow::Result;
use stackable_zookeeper_crd::ZookeeperVersion;

// This will cause the integration tests to fail because config maps are not updated correctly. This
// can be activated once https://github.com/stackabletech/zookeeper-operator/issues/128 is fixed.
#[test]
#[ignore]
fn test_scale_cluster_up() -> Result<()> {
    let name = append_random_characters("simple");
    let version = ZookeeperVersion::v3_4_14;
    let (zookeeper_cr, expected_pod_count) = build_zk_cluster(&name, &version, 1)?;

    let mut cluster = build_test_cluster();
    cluster.create_or_update(&zookeeper_cr, expected_pod_count)?;
    let created_pods = cluster.list_pods();

    custom_checks(
        &cluster.client,
        created_pods.as_slice(),
        &version,
        2181,
        expected_pod_count,
    )?;

    let (zookeeper_cr, expected_pod_count) = build_zk_cluster(&name, &version, 3)?;
    cluster.create_or_update(&zookeeper_cr, expected_pod_count)?;
    let created_pods = cluster.list_pods();

    custom_checks(
        &cluster.client,
        created_pods.as_slice(),
        &version,
        2181,
        expected_pod_count,
    )?;

    Ok(())
}

// This will cause the integration tests to fail because config maps are not updated correctly. This
// can be activated once https://github.com/stackabletech/zookeeper-operator/issues/128 is fixed.
#[test]
#[ignore]
fn test_scale_cluster_down() -> Result<()> {
    let name = append_random_characters("simple");
    let version = ZookeeperVersion::v3_4_14;
    let mut cluster = build_test_cluster();

    let (zookeeper_cr, expected_pod_count) = build_zk_cluster(&name, &version, 3)?;
    cluster.create_or_update(&zookeeper_cr, expected_pod_count)?;
    let created_pods = cluster.list_pods();

    custom_checks(
        &cluster.client,
        created_pods.as_slice(),
        &version,
        2181,
        expected_pod_count,
    )?;

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

    Ok(())
}
