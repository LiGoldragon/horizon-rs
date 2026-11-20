//! Cluster-level validation of a user's two-ended synchronizations.

use super::{error::Error, node::NodeDefinitions};
use crate::UserDefinition;

pub(crate) trait SynchronizationsValidating {
    fn validate_synchronizations(&self, nodes: &NodeDefinitions) -> Result<(), Error>;
}

impl SynchronizationsValidating for [UserDefinition] {
    fn validate_synchronizations(&self, nodes: &NodeDefinitions) -> Result<(), Error> {
        for user in self {
            for synchronization in &user.synchronization_vector {
                let master = &synchronization.master;
                let replica = &synchronization.replica;
                if master == replica {
                    return Err(Error::SynchronizationSameNode {
                        user: user.user_name.clone(), node: master.clone(),
                    });
                }
                for node in [master, replica] {
                    if !nodes.contains_key(node) {
                        return Err(Error::SynchronizationUnknownNode {
                            user: user.user_name.clone(), node: node.clone(),
                        });
                    }
                    if !user.user_pub_key_vector.iter().any(|key| &key.node_name == node) {
                        return Err(Error::SynchronizationUserAbsent {
                            user: user.user_name.clone(), node: node.clone(),
                        });
                    }
                }
                let tree = &synchronization.tree;
                if tree.is_empty() || tree.starts_with('/') || tree.contains(['\0', '\n'])
                    || tree.split('/').any(|component| component.is_empty() || component == "." || component == "..")
                {
                    return Err(Error::SynchronizationInvalidTree {
                        user: user.user_name.clone(), tree: tree.clone(),
                    });
                }
            }
            for (index, first) in user.synchronization_vector.iter().enumerate() {
                for second in &user.synchronization_vector[index + 1..] {
                    for node in [&first.master, &first.replica] {
                        if node != &second.master && node != &second.replica { continue; }
                        if first.tree == second.tree
                            || first.tree.strip_prefix(&second.tree).is_some_and(|tail| tail.starts_with('/'))
                            || second.tree.strip_prefix(&first.tree).is_some_and(|tail| tail.starts_with('/'))
                        {
                            return Err(Error::SynchronizationOverlappingTrees {
                                user: user.user_name.clone(), node: node.clone(),
                                first: first.tree.clone(), second: second.tree.clone(),
                            });
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
