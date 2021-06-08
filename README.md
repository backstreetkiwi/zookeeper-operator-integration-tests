# zookeeper-operator-integration-tests

This repository bundles integration tests for the [Stackable Operator](https://github.com/stackabletech/zookeeper-operator) for Apache ZooKeeper. The integration tests should be deployed on the Stackable [test-dev-cluster](https://github.com/stackabletech/test-dev-cluster).  

## Requirements

- [test-dev-cluster](https://github.com/stackabletech/test-dev-cluster) to deploy the Stackable Agent, the Stackable Operator for Apache ZooKeeper and run the integrations tests. Please check the readme of the on how to set it up correctly.
- Although the tests are written for a dynamic amount of test nodes, we recommend using the test-dev cluster with 3 nodes / agents, which was tested extensively: `./init.sh debian zookeeper-operator --scale agent=3`

## Usage

Please refer to the [test-dev-cluster](https://github.com/stackabletech/test-dev-cluster) instructions on how to set up and run the integration tests.
Install the CRD after initializing the cluster and before `./run.sh run-operator` like this:
  `docker exec -t operator kubectl apply -f /zookeeper-operator/deploy/crd`

This avoids a bug in the operator framework, where the operator runs in a very tight loop and consumes a lot of CPU if the CRD is not available.

## Content

Currently, the integration tests cover the following cases:

- **Create** a ZooKeeper cluster and check if it is running correctly via the [four letter commands](https://zookeeper.apache.org/doc/r3.4.14/zookeeperAdmin.html#sc_zkCommands) for version 3.5.2 and below or the [admin server commands](https://zookeeper.apache.org/doc/r3.7.0/zookeeperAdmin.html#sc_adminserver) for version 3.5.3 and above. 
- **Update** a ZooKeeper cluster from version 3.4.14 to 3.5.8 and check the correctness via the four letter commands or admin server commands.
- **Scale** a ZooKeeper cluster up (e.g. from 1 to 3 nodes) and down (e.g. from 3 to 1 nodes) and check the correctness via four letter commands or admin server commands as well as checking the configmaps which are responsible for transmitting the zoo.cfg config properties.





