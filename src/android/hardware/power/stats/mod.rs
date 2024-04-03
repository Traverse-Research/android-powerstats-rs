#![allow(
    clippy::derivable_impls,
    non_snake_case,
    clippy::needless_lifetimes,
    unused_mut,
    unused_imports,
    clippy::identity_op,
    unused_qualifications
)]

pub mod Channel;
pub mod EnergyConsumer;
pub mod EnergyConsumerAttribution;
pub mod EnergyConsumerResult;
pub mod EnergyConsumerType;
pub mod EnergyMeasurement;
pub mod IPowerStats;
pub mod PowerEntity;
pub mod State;
pub mod StateResidency;
pub mod StateResidencyResult;

pub(crate) mod mangled {
    pub(crate) use super::Channel::mangled::*;
    pub(crate) use super::EnergyConsumer::mangled::*;
    pub(crate) use super::EnergyConsumerAttribution::mangled::*;
    pub(crate) use super::EnergyConsumerResult::mangled::*;
    pub(crate) use super::EnergyConsumerType::mangled::*;
    pub(crate) use super::EnergyMeasurement::mangled::*;
    // pub(crate) use super::IPowerStats::mangled::*;
    pub(crate) use super::PowerEntity::mangled::*;
    pub(crate) use super::State::mangled::*;
    pub(crate) use super::StateResidency::mangled::*;
    pub(crate) use super::StateResidencyResult::mangled::*;
}
