pub mod common;

use crate::common::checks::custom_pod_checks;

use crate::common::zookeeper::append_random_characters;
use anyhow::Result;
use common::zookeeper::{build_test_cluster, build_zk_custom_resource_1_server};
use stackable_zookeeper_crd::ZookeeperVersion;

#[test]
fn test_create_cluster_3_4_14_with_node_ids() -> Result<()> {
    let name = append_random_characters("simple");
    let version = ZookeeperVersion::v3_4_14;
    let mut cluster = build_test_cluster();

    let (zookeeper_cr, expected_pod_count) = build_zk_custom_resource_1_server(&name, &version)?;
    cluster.create_or_update(&zookeeper_cr, expected_pod_count)?;
    let created_pods = cluster.get_current_pods();

    custom_pod_checks(
        &cluster.client,
        created_pods.as_slice(),
        &version,
        2181,
        expected_pod_count,
    )?;

    Ok(())
}

#[test]
fn test_create_cluster_3_5_8_with_node_ids() -> Result<()> {
    let name = append_random_characters("simple");
    let version = ZookeeperVersion::v3_5_8;
    let mut cluster = build_test_cluster();

    let (zookeeper_cr, expected_pod_count) = build_zk_custom_resource_1_server(&name, &version)?;
    cluster.create_or_update(&zookeeper_cr, expected_pod_count)?;
    let created_pods = cluster.get_current_pods();

    custom_pod_checks(
        &cluster.client,
        created_pods.as_slice(),
        &version,
        8080,
        expected_pod_count,
    )?;

    Ok(())
}
