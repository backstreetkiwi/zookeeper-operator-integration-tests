use crate::common::builder::ZookeeperClusterBuilder;
use crate::test::prelude::{Node, Pod, TestKubeClient};

use anyhow::{anyhow, Result};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
use kube::api::ObjectList;
use stackable_operator::conditions::ConditionStatus;
use stackable_zookeeper_crd::{
    RoleGroups, SelectorAndConfig, ZookeeperCluster, ZookeeperConfig, ZookeeperVersion,
};
use std::collections::{BTreeMap, HashMap};
use std::thread;
use std::time::{Duration, Instant};

/// We use this to create different custom resources for e.g. scaling or updates.
pub struct TestClusterDefinition {
    pub name: String,
    pub version: ZookeeperVersion,
    pub role_groups: RoleGroups<ZookeeperConfig>,
}

impl TestClusterDefinition {
    /// This is for a console output during tests to see what custom resource is deployed.
    pub fn description(&self) -> String {
        format!(
            "ZooKeeper test cluster definition for name [{}] with version [{}] and role groups [{:?}]",
            self.name,
            self.version.to_string(),
            self.role_groups
        )
    }
}

/// A Wrapper to avoid passing in client or cluster everywhere.
pub struct TestCluster {
    pub client: TestKubeClient,
    cluster: Option<ZookeeperCluster>,
}

impl TestCluster {
    /// This creates a kube client and should be executed at the start of every test.
    pub fn new() -> TestCluster {
        TestCluster {
            client: TestKubeClient::new(),
            cluster: None,
        }
    }

    /// Applies a custom resource, stores the returned cluster object and sleeps for
    /// to seconds to give the operator time to react on the custom resource.
    /// Without the sleep it can happen that tests run without any pods being created
    fn apply(&mut self, def: TestClusterDefinition) {
        let cluster: ZookeeperCluster = self
            .client
            .apply(&serde_yaml::to_string(&TestCluster::build(def)).unwrap());

        self.cluster = Some(cluster);

        // we wait here to give the operator time to react to the custom resource
        thread::sleep(Duration::from_secs(2));
    }

    /// Creates or updates a custom resource and waits for the cluster to be up and running
    /// within the provided timeout. Depending on the cluster definition we hand in the number
    /// of created pods we expect manually.
    pub fn create_or_update(
        &mut self,
        def: TestClusterDefinition,
        timeout_in_seconds: u64,
        expected_pod_count: usize,
    ) -> Result<()> {
        println!(
            "\n{}\n with timeout: {} second(s)",
            def.description(),
            timeout_in_seconds
        );
        self.apply(def);
        self.wait_ready(timeout_in_seconds, expected_pod_count)?;
        Ok(())
    }

    /// Deletes the custom resource, and waits for pods to be terminated
    /// This should be executed after every single test to provide a "clean"
    /// environment for the following tests.
    pub fn delete(&mut self) -> Result<()> {
        if let Some(cluster) = self.cluster.take() {
            self.client.delete(cluster);
            self.wait_for_pods_terminated(30)?;
            self.cluster = None;
        }

        Ok(())
    }

    /// Returns all available pods in the cluster via the name label.
    pub fn get_current_pods(&self) -> Vec<Pod> {
        let current_pods: ObjectList<Pod> =
            self.client.list_labeled("app.kubernetes.io/name=zookeeper");
        current_pods.items
    }

    /// Returns all available nodes in the cluster. Can be used to determine the expected pods
    /// for tests (depending on the custom resource)
    pub fn get_available_nodes(&self) -> Vec<Node> {
        let available_nodes: ObjectList<Node> = self.client.list_labeled("");
        available_nodes.items
    }

    /// A "busy" wait for all pods to be terminated and cleaned up.
    pub fn wait_for_pods_terminated(&self, timeout_in_seconds: u64) -> Result<()> {
        let now = Instant::now();

        while now.elapsed().as_secs() < timeout_in_seconds {
            let pods: Vec<Pod> = self
                .client
                .list_labeled("app.kubernetes.io/name=zookeeper")
                .items;

            if pods.len() == 0 {
                return Ok(());
            }

            println!("Waiting for {} Pod(s) to terminate", pods.len());

            thread::sleep(Duration::from_secs(1));
        }

        Err(anyhow!(
            "Pods did not terminate within the specified timeout of {} second(s)",
            timeout_in_seconds
        ))
    }

    /// A "busy" (2 second sleep) wait for the cluster to be ready. We check the "Upgrading"
    /// condition and the expected pods that should be up and running.
    pub fn wait_ready(&self, timeout_in_seconds: u64, expected_pod_count: usize) -> Result<()> {
        let now = Instant::now();

        let name = self
            .cluster
            .as_ref()
            .map(|cluster| cluster.metadata.name.as_ref().unwrap())
            .unwrap();

        while now.elapsed().as_secs() < timeout_in_seconds {
            let cluster: ZookeeperCluster = self.client.find(&name).unwrap();

            if let Some(status) = cluster.status {
                println!("Waiting for cluster to be ready...");

                for condition in &status.conditions {
                    if condition.type_ == "Upgrading"
                        && condition.status == ConditionStatus::False.to_string()
                    {
                        let created_pods: ObjectList<Pod> =
                            self.client.list_labeled("app.kubernetes.io/name=zookeeper");

                        if created_pods.items.len() != expected_pod_count {
                            break;
                        }

                        for pod in &created_pods {
                            self.client.verify_pod_condition(pod, "Ready")
                        }

                        println!("Installation finished");
                        return Ok(());
                    }
                }
            }

            thread::sleep(Duration::from_secs(2));
        }

        Err(anyhow!(
            "Cluster did not startup within the specified timeout of {} second(s)",
            timeout_in_seconds
        ))
    }

    /// Helper method to create the ZooKeeperCluster from the TestClusterDefinition.
    fn build(def: TestClusterDefinition) -> ZookeeperCluster {
        let mut builder = ZookeeperClusterBuilder::new();

        builder.name(&def.name);
        builder.with_version(def.version);

        for (role_group, selector) in def.role_groups.selectors {
            builder.add_selector(&role_group, &selector);
        }

        builder.build()
    }
}

/// A ZooKeeper custom resource based on "kubernetes.io/arch" label.
/// This is the standard cluster we use for the tests. Will deploy a pod to
/// every node / agent supplied.
pub fn build_test_cluster_definition_with_label_selector(
    name: &str,
    version: &ZookeeperVersion,
    instances: u16,
    instances_per_node: u8,
) -> TestClusterDefinition {
    let mut selectors = HashMap::new();

    let mut labels = BTreeMap::new();
    labels.insert(
        "kubernetes.io/arch".to_string(),
        "stackable-linux".to_string(),
    );

    selectors.insert(
        "default".to_string(),
        SelectorAndConfig {
            instances,
            instances_per_node,
            config: None,
            selector: Some(LabelSelector {
                match_expressions: None,
                match_labels: Some(labels),
            }),
        },
    );

    TestClusterDefinition {
        name: name.to_string(),
        version: version.clone(),
        role_groups: RoleGroups { selectors },
    }
}

/// A ZooKeeper custom resource definition based on node id labels.
/// This is required to fix the number of deployed pods in order to
/// test scale up and down.
pub fn build_test_cluster_definition_with_node_id_label_selector(
    name: &str,
    version: &ZookeeperVersion,
    instances: usize,
) -> TestClusterDefinition {
    let mut selectors = HashMap::new();

    // for each requested instance
    for node_id in 1..instances + 1 {
        let mut labels = BTreeMap::new();
        labels.insert("node".to_string(), node_id.to_string());

        selectors.insert(
            format!("node-{}", node_id),
            SelectorAndConfig {
                instances: 1,
                instances_per_node: 1,
                config: None,
                selector: Some(LabelSelector {
                    match_expressions: None,
                    match_labels: Some(labels),
                }),
            },
        );
    }

    TestClusterDefinition {
        name: name.to_string(),
        version: version.clone(),
        role_groups: RoleGroups { selectors },
    }
}
