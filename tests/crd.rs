mod test;
use test::prelude::*;

use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::{
    CustomResourceDefinition,
};
use kube::api::Meta;

#[test]
fn find_crd() {
    let client = TestKubeClient::new();

    let crd: CustomResourceDefinition = client.find_crd("zookeeperclusters.zookeeper.stackable.tech").unwrap();

    assert_eq!("zookeeperclusters.zookeeper.stackable.tech".to_string(), crd.name());
}