#![cfg_attr(not(feature = "std"), no_std)]
#![feature(associated_type_defaults)]

pub mod assets;
pub mod consensus;
pub mod evm;
pub mod governance;
pub mod system;
pub mod xcm;

use frame_support::traits::{ConstU32, Get};
use sp_version::RuntimeVersion;

pub trait SystemConfig {
    type AccountId;
    type Lookup;
    type SS58Prefix;
    type Version: Get<RuntimeVersion>;
    type ExistentialDeposit;
    type ScheduleOrigin;
    type PreimageOrigin;
    type ProxyType;
    type MaxConsumers = ConstU32<16>;
    type MaxSignatories = ConstU32<100>;
    type MaxPendingProxies = ConstU32<32>;
    type MaxProxies = ConstU32<32>;
    type MaxFreezes = ConstU32<0>;
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ConstU32<50>;
}

pub trait ConsensusConfig {
    type DisabledValidators = ();
    type MaxAuthorities = ConstU32<100_000>;
    type MaxCandidates = ConstU32<100>;
    type MaxInvulnerables = ConstU32<20>;
    type MinEligibleCollators = ConstU32<4>;
    type CollatorSelectionUpdateOrigin;
}

pub trait AssetsConfig {
    type ApprovalDeposit;
    type AssetAccountDeposit;
    type AssetDeposit;
    type AssetId;
    type AssetType;
    type BenchmarkHelper = ();
    type CreateOrigin;
    type ForceOrigin;
    type ForeignAssetModifierOrigin;
    type AssetRegistrar;
    type AssetRegistrarMetadata;
    type WeightToFee;
}

pub trait GovernanceConfig {
    type TreasuryBurn = ();
    type TreasurySpendFunds = ();
    type TreasuryBurnDestination = ();
    type TreasuryMaxApprovals = ConstU32<100>;
    type TreasuryInteriorLocation;
    type TreasuryPalletId;
    type TreasurySpendPeriod;
    type TreasuryPayoutSpendPeriod;
    type TreasuryRejectOrigin;
    type TreasurySpendOrigin;
    type ConvictionVoteLockingPeriod;
    type ConvictionMaxVotes = ConstU32<512>;
    type DispatchWhitelistedOrigin;
    type WhitelistOrigin;
    type ReferendaAlarmInterval;
    type ReferendaCancelOrigin;
    type ReferendaKillOrigin;
    type ReferendaMaxQueued = ConstU32<20>;
    type ReferendaSlash;
    type ReferendaSubmissionDeposit;
    type ReferendaSubmitOrigin;
    type ReferendaUndecidingTimeout;
}

pub trait XcmConfig {
    type LocationToAccountId;
    type LocalOriginToLocation;
    type AssetTransactors;
    type XcmOriginToTransactDispatchOrigin;
    type FeeManager;
    type Trader;
    type Reserves;
    type MessageQueueHeapSize;
    type MessageQueueMaxStale;
    type MessageQueueServiceWeight;
    type XcmpQueueControllerOrigin;
    type XcmpQueueMaxInboundSuspended;
    type XcmAdminOrigin;
    type MaxActiveOutboundChannels = ConstU32<128>;
    type MaxPageSize = ConstU32<{ 1 << 16 }>;
    type MaxAssetsForTransfer;
    type ParachainMinFee;
    type XtokensReserveProviders;
    type AccountIdToLocation;
    type BaseXcmWeight;
    type CurrencyId;
    type CurrencyIdToLocation;
    type DerivativeAddressRegistrationOrigin;
    type HrmpManipulatorOrigin;
    type HrmpOpenOrigin;
    type MaxHrmpRelayFee;
    type TransactorReserveProvider;
    type SelfLocation;
    type SovereignAccountDispatcherOrigin;
    type Transactors;
    type UniversalLocation;
    type XcmWeigher;
    type XcmSender;
    type AddSupportedAssetOrigin;
    type AssetFeesFilter;
    type EditSupportedAssetOrigin;
    type SelfReserve;
    type RelayLocation;
    type PauseSupportedAssetOrigin;
    type RemoveSupportedAssetOrigin;
    type ResumeSupportedAssetOrigin;
    type WeightToFee;
    type XcmFeesAccount;
}

pub trait EvmConfig {
    type AddressMapping;
    type FindAuthor;
    type CallOrigin;
    type WithdrawOrigin;
    type PrecompilesType;
    type PrecompilesValue;
    type Erc20XcmBridgeTransferGasLimit;
}