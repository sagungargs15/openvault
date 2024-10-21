// This PolicyEngine implementation provides the following features:
// 1. Creation of complex spending policies using Miniscript.
// 2. Conversion of policies to Bitcoin descriptors.
// 3. Validation of spending attempts against policies.
// 4. Support for multiple named policies.
// Helper methods for common policy types (multisig, timelocked, recovery).

use std::collections::HashMap;
use std::str::FromStr;
use bitcoin::util::bip32::{ExtendedPubKey, DerivationPath};
use bitcoin::{PublicKey, Network};
use miniscript::{Miniscript, Terminal, Segwitv0};
use miniscript::policy::Concrete;
use bdk::descriptor::Descriptor;
use bdk::keys::ExtendedKey;

pub struct PolicyEngine {
    policies: HashMap<String, Policy>,
    network: Network,
}

pub struct Policy {
    name: String,
    descriptor: Descriptor<ExtendedKey>,
    concrete_policy: Concrete<PublicKey>,
}

impl PolicyEngine {
    pub fn new(network: Network) -> Self {
        PolicyEngine {
            policies: HashMap::new(),
            network,
        }
    }

    pub fn add_policy(&mut self, name: &str, policy_str: &str, xpubs: &[(&str, ExtendedPubKey)]) -> Result<(), PolicyError> {
        let concrete_policy = Concrete::<PublicKey>::from_str(policy_str)?;
        let descriptor = self.policy_to_descriptor(&concrete_policy, xpubs)?;

        let policy = Policy {
            name: name.to_string(),
            descriptor,
            concrete_policy,
        };

        self.policies.insert(name.to_string(), policy);
        Ok(())
    }

    pub fn get_descriptor(&self, policy_name: &str) -> Option<&Descriptor<ExtendedKey>> {
        self.policies.get(policy_name).map(|p| &p.descriptor)
    }

    pub fn validate_spend(&self, policy_name: &str, signers: &[PublicKey]) -> Result<bool, PolicyError> {
        let policy = self.policies.get(policy_name)
            .ok_or(PolicyError::PolicyNotFound)?;

        let mut satisfied = HashMap::new();
        for pk in signers {
            satisfied.insert(pk.clone(), ());
        }

        Ok(policy.concrete_policy.satisfaction_weight(&satisfied).is_some())
    }

    fn policy_to_descriptor(&self, policy: &Concrete<PublicKey>, xpubs: &[(&str, ExtendedPubKey)]) -> Result<Descriptor<ExtendedKey>, PolicyError> {
        let ms = Miniscript::<PublicKey, Segwitv0>::from_ast(policy.clone().into_inner())?;
        let mut key_map = HashMap::new();
        for (key_name, xpub) in xpubs {
            key_map.insert(key_name.to_string(), ExtendedKey::from(*xpub));
        }

        let descriptor = Descriptor::new_wsh(ms, key_map)?;
        Ok(descriptor)
    }

    pub fn list_policies(&self) -> Vec<String> {
        self.policies.keys().cloned().collect()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PolicyError {
    #[error("Invalid policy string: {0}")]
    InvalidPolicy(#[from] miniscript::Error),
    #[error("Error creating descriptor: {0}")]
    DescriptorError(#[from] bdk::descriptor::error::Error),
    #[error("Policy not found")]
    PolicyNotFound,
}

// Example usage and helper functions

impl PolicyEngine {
    pub fn add_multisig_policy(&mut self, name: &str, threshold: usize, keys: &[(&str, ExtendedPubKey)]) -> Result<(), PolicyError> {
        let policy_str = if keys.len() == threshold {
            format!("and({})", keys.iter().map(|(name, _)| format!("pk({})", name)).collect::<Vec<_>>().join(","))
        } else {
            format!("thresh({},{})", threshold, keys.iter().map(|(name, _)| format!("pk({})", name)).collect::<Vec<_>>().join(","))
        };
        self.add_policy(name, &policy_str, keys)
    }

    pub fn add_timelocked_policy(&mut self, name: &str, key: (&str, ExtendedPubKey), blocks: u32) -> Result<(), PolicyError> {
        let policy_str = format!("and(pk({}),older({}))", key.0, blocks);
        self.add_policy(name, &policy_str, &[key])
    }

    pub fn add_recovery_policy(&mut self, name: &str, main_key: (&str, ExtendedPubKey), recovery_key: (&str, ExtendedPubKey), blocks: u32) -> Result<(), PolicyError> {
        let policy_str = format!("or(pk({}),and(pk({}),older({})))", main_key.0, recovery_key.0, blocks);
        self.add_policy(name, &policy_str, &[main_key, recovery_key])
    }
}