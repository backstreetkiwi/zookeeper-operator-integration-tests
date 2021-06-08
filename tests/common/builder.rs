use kube::api::ObjectMeta;
use stackable_zookeeper_crd::{
    RoleGroups, SelectorAndConfig, ZookeeperCluster, ZookeeperClusterSpec, ZookeeperConfig,
    ZookeeperVersion,
};

pub struct ZookeeperClusterBuilder {
    cluster: ZookeeperCluster,
}

impl ZookeeperClusterBuilder {
    pub fn new() -> ZookeeperClusterBuilder {
        ZookeeperClusterBuilder {
            cluster: ZookeeperCluster {
                api_version: "zookeeper.stackable.tech/v1".to_string(),
                kind: "ZookeeperCluster".to_string(),
                metadata: ObjectMeta {
                    ..ObjectMeta::default()
                },
                spec: ZookeeperClusterSpec {
                    version: ZookeeperVersion::v3_4_14,
                    servers: RoleGroups {
                        selectors: Default::default(),
                    },
                },
                status: None,
            },
        }
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.cluster.metadata.name = Some(name.to_string());
        self
    }

    pub fn with_version(&mut self, version: ZookeeperVersion) -> &mut Self {
        self.cluster.spec.version = version;
        self
    }

    pub fn add_selector(
        &mut self,
        role_group: &str,
        selector: &SelectorAndConfig<ZookeeperConfig>,
    ) -> &mut Self {
        self.cluster
            .spec
            .servers
            .selectors
            .insert(role_group.to_string(), selector.clone());
        self
    }

    /// Consumes the Builder and returns a constructed ZookeeperCluster
    pub fn build(&self) -> ZookeeperCluster {
        // We're cloning here because we can't take just `self` in this method because then
        // we couldn't chain the method with the others easily (because they return &mut self and not self)
        self.cluster.clone()
    }
}
