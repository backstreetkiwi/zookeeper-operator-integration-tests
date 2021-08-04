pub mod common;

use crate::common::checks::custom_pod_checks;
use crate::common::zookeeper::{
    append_random_characters, build_test_cluster, build_zk_custom_resource_1_server,
    build_zk_custom_resource_3_server,
};

use anyhow::Result;
use stackable_zookeeper_crd::ZookeeperVersion;

#[test]
fn test_scale_cluster_up() -> Result<()> {
    let name = append_random_characters("simple");
    let version = ZookeeperVersion::v3_4_14;
    let (zookeeper_cr, expected_pod_count) = build_zk_custom_resource_1_server(&name, &version)?;

    let mut cluster = build_test_cluster();
    cluster.create_or_update(&zookeeper_cr, expected_pod_count)?;
    let created_pods = cluster.get_current_pods();

    custom_pod_checks(
        &cluster.client,
        created_pods.as_slice(),
        &version,
        2181,
        expected_pod_count,
    )?;

    let (zookeeper_cr, expected_pod_count) = build_zk_custom_resource_3_server(&name, &version)?;
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
fn test_scale_cluster_down() -> Result<()> {
    let name = append_random_characters("simple");
    let version = ZookeeperVersion::v3_4_14;
    let mut cluster = build_test_cluster();

    let (zookeeper_cr, expected_pod_count) = build_zk_custom_resource_3_server(&name, &version)?;
    cluster.create_or_update(&zookeeper_cr, expected_pod_count)?;
    let created_pods = cluster.get_current_pods();

    custom_pod_checks(
        &cluster.client,
        created_pods.as_slice(),
        &version,
        2181,
        expected_pod_count,
    )?;

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
