//! Cluster-level validation of a user's two-ended synchronizations.

use std::collections::BTreeMap;

use super::{error::Error, node::NodeDefinitions};
use crate::{Synchronization, User, UserDefinition};

/// The master identity admitted by the replica's projected account policy.
trait SynchronizationSshAuthorizing {
    fn authorize_user(
        &self,
        user: &str,
        master_users: &[User],
        replica_users: &[User],
    ) -> Result<(), Error>;
}

impl SynchronizationSshAuthorizing for Synchronization {
    fn authorize_user(
        &self,
        user: &str,
        master_users: &[User],
        replica_users: &[User],
    ) -> Result<(), Error> {
        let master_key = master_users
            .iter()
            .find(|account| account.name == user)
            .and_then(|account| account.ssh_public_key.as_ref())
            .ok_or_else(|| Error::SynchronizationUserAbsent {
                user: user.to_owned(),
                node: self.master.clone(),
            })?;
        let replica = replica_users
            .iter()
            .find(|account| account.name == user)
            .ok_or_else(|| Error::SynchronizationUserAbsent {
                user: user.to_owned(),
                node: self.replica.clone(),
            })?;
        if !replica.ssh_public_keys.contains(master_key) {
            return Err(Error::SynchronizationMasterKeyUnauthorized {
                user: user.to_owned(),
                master: self.master.clone(),
                replica: self.replica.clone(),
            });
        }
        Ok(())
    }
}

pub(crate) trait SynchronizationsValidating {
    fn validate_synchronizations(
        &self,
        nodes: &NodeDefinitions,
        users_by_node: &BTreeMap<String, Vec<User>>,
    ) -> Result<(), Error>;
}

impl SynchronizationsValidating for [UserDefinition] {
    fn validate_synchronizations(
        &self,
        nodes: &NodeDefinitions,
        users_by_node: &BTreeMap<String, Vec<User>>,
    ) -> Result<(), Error> {
        for user in self {
            for synchronization in &user.synchronization_vector {
                let master = &synchronization.master;
                let replica = &synchronization.replica;
                if master == replica {
                    return Err(Error::SynchronizationSameNode {
                        user: user.user_name.clone(),
                        node: master.clone(),
                    });
                }
                for node in [master, replica] {
                    if !nodes.contains_key(node) {
                        return Err(Error::SynchronizationUnknownNode {
                            user: user.user_name.clone(),
                            node: node.clone(),
                        });
                    }
                }
                synchronization.authorize_user(
                    &user.user_name,
                    &users_by_node[master],
                    &users_by_node[replica],
                )?;
                let tree = &synchronization.tree;
                if tree.is_empty()
                    || tree.starts_with('/')
                    || tree.contains(['\0', '\n', '\r'])
                    || tree.starts_with([' ', '\t'])
                    || tree.ends_with([' ', '\t'])
                    || tree.split('/').any(|component| {
                        component.is_empty() || component == "." || component == ".."
                    })
                {
                    return Err(Error::SynchronizationInvalidTree {
                        user: user.user_name.clone(),
                        tree: tree.clone(),
                    });
                }
            }
            for (index, first) in user.synchronization_vector.iter().enumerate() {
                for second in &user.synchronization_vector[index + 1..] {
                    for node in [&first.master, &first.replica] {
                        if node != &second.master && node != &second.replica {
                            continue;
                        }
                        if first.tree == second.tree
                            || first
                                .tree
                                .strip_prefix(&second.tree)
                                .is_some_and(|tail| tail.starts_with('/'))
                            || second
                                .tree
                                .strip_prefix(&first.tree)
                                .is_some_and(|tail| tail.starts_with('/'))
                        {
                            return Err(Error::SynchronizationOverlappingTrees {
                                user: user.user_name.clone(),
                                node: node.clone(),
                                first: first.tree.clone(),
                                second: second.tree.clone(),
                            });
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, SynchronizationSshAuthorizing};
    use crate::projection::user::{UserProjection, Viewpoint};
    use crate::{
        Keyboard, Magnitude, Style, Synchronization, UserDefinition, UserPubKey, UserRole,
    };

    #[test]
    fn replica_authorization_checks_the_projected_account_and_exact_master_key() {
        let definition = UserDefinition {
            user_name: "example".to_owned(),
            user_role: UserRole::Code,
            magnitude: Magnitude::Max,
            keyboard: Keyboard::Qwerty,
            style: Style::Emacs,
            github_id_option: None,
            boolean_option: None,
            user_pub_key_vector: vec![UserPubKey {
                node_name: "master".to_owned(),
                ssh_pub_key: "AAAA-master".to_owned(),
                keygrip: "master-keygrip".to_owned(),
                user_capability_vector: vec![],
            }],
            editor_option: None,
            text_size_option: None,
            synchronization_vector: vec![],
        };
        let master = definition.project_from(
            Magnitude::Max,
            &Viewpoint {
                node: "master",
                public_domain: "cluster.example",
                center: false,
                size: "Max",
            },
        );
        let mut replica = definition.project_from(
            Magnitude::Max,
            &Viewpoint {
                node: "replica",
                public_domain: "cluster.example",
                center: true,
                size: "Max",
            },
        );
        let synchronization = Synchronization {
            master: "master".to_owned(),
            replica: "replica".to_owned(),
            tree: "Code".to_owned(),
        };
        let master_accounts = [master.clone()];
        assert!(!replica.has_public_key);
        assert!(
            synchronization
                .authorize_user("example", &master_accounts, &[replica.clone()])
                .is_ok()
        );
        replica.ssh_public_keys = vec!["ssh-ed25519 AAAA-other".to_owned()];
        assert!(matches!(
            synchronization.authorize_user("example", &master_accounts, &[replica]),
            Err(Error::SynchronizationMasterKeyUnauthorized { .. })
        ));
        assert!(matches!(
            synchronization.authorize_user("example", &master_accounts, &[]),
            Err(Error::SynchronizationUserAbsent { node, .. }) if node == "replica"
        ));
        let mut missing_identity = master;
        missing_identity.ssh_public_key = None;
        assert!(matches!(
            synchronization.authorize_user("example", &[missing_identity], &[]),
            Err(Error::SynchronizationUserAbsent { node, .. }) if node == "master"
        ));
    }
}
