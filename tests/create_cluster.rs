pub mod common;

use crate::common::checks::custom_checks;

use crate::common::zookeeper::append_random_characters;
use anyhow::Result;
use common::zookeeper::{build_test_cluster, build_zk_cluster};
use stackable_zookeeper_crd::ZookeeperVersion;

#[test]
fn test_create_cluster_3_4_14() -> Result<()> {
    let name = append_random_characters("simple");
    let version = ZookeeperVersion::v3_4_14;
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

    Ok(())
}

#[test]
fn test_create_cluster_3_5_8() -> Result<()> {
    let name = append_random_characters("simple");
    let version = ZookeeperVersion::v3_5_8;
    let mut cluster = build_test_cluster();

    let (zookeeper_cr, expected_pod_count) = build_zk_cluster(&name, &version, 3)?;
    cluster.create_or_update(&zookeeper_cr, expected_pod_count)?;
    let created_pods = cluster.list_pods();

    custom_checks(
        &cluster.client,
        created_pods.as_slice(),
        &version,
        8080,
        expected_pod_count,
    )?;

    Ok(())
}
