//! Implements the OpenZeppelin assets configuration for a Runtime.
//!
//! This macro sets up the necessary configurations for the following pallets:
//! - `pallet_assets`
//! - `pallet_transaction_payment`
//! - `pallet_asset_manager`
//!
//! # Parameters
//! - `$t`: A type that implements the `AssetsConfig` trait, providing the necessary associated types
//!   and configurations.
//!
//! # Important
//! Rerun benchmarks if making changes to runtime configuration, as weight calculations
//! may need to be updated.

#[macro_export]
macro_rules! impl_openzeppelin_assets {
    ($t:ty) => {
        parameter_types! {
            pub const StringLimit: u32 = 50;
            pub const MetadataDepositBase: Balance = deposit(1, 68);
            pub const MetadataDepositPerByte: Balance = deposit(0, 1);
            pub const RemoveItemsLimit: u32 = 1000;
        }

        // Required for runtime benchmarks
        pallet_assets::runtime_benchmarks_enabled! {
            pub struct BenchmarkHelper;
            impl<AssetIdParameter> pallet_assets::BenchmarkHelper<AssetIdParameter> for BenchmarkHelper
            where
                AssetIdParameter: From<<$t as AssetsConfig>::AssetId>,
            {
                fn create_asset_id_parameter(id: u32) -> AssetIdParameter {
                    (id as <$t as AssetsConfig>::AssetId).into()
                }
            }
        }

        impl pallet_assets::Config for Runtime {
            type ApprovalDeposit = <$t as AssetsConfig>::ApprovalDeposit;
            type AssetAccountDeposit = <$t as AssetsConfig>::AssetAccountDeposit;
            type AssetDeposit = <$t as AssetsConfig>::AssetDeposit;
            type AssetId = <$t as AssetsConfig>::AssetId;
            type AssetIdParameter = parity_scale_codec::Compact<<$t as AssetsConfig>::AssetId>;
            type Balance = Balance;
            #[cfg(feature = "runtime-benchmarks")]
            type BenchmarkHelper = BenchmarkHelper;
            type CallbackHandle = ();
            type CreateOrigin = <$t as AssetsConfig>::CreateOrigin;
            type Currency = Balances;
            type Extra = ();
            type ForceOrigin = <$t as AssetsConfig>::ForceOrigin;
            type Freezer = ();
            type MetadataDepositBase = MetadataDepositBase;
            type MetadataDepositPerByte = MetadataDepositPerByte;
            type RemoveItemsLimit = RemoveItemsLimit;
            type RuntimeEvent = RuntimeEvent;
            type StringLimit = StringLimit;
            type WeightInfo = <$t as AssetsWeight>::Assets;
        }

        parameter_types! {
            /// Relay Chain `TransactionByteFee` / 10
            pub const TransactionByteFee: Balance = 10 * MICROCENTS;
            pub const OperationalFeeMultiplier: u8 = 5;
        }

        impl pallet_transaction_payment::Config for Runtime {
            /// There are two possible mechanisms available: slow and fast adjusting.
            /// With slow adjusting fees stay almost constant in short periods of time, changing only in long term.
            /// It may lead to long inclusion times during spikes, therefore tipping is enabled.
            /// With fast adjusting fees change rapidly, but fixed for all users at each block (no tipping)
            type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
            type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
            type OnChargeTransaction = pallet_transaction_payment::FungibleAdapter<Balances, ()>;
            type OperationalFeeMultiplier = OperationalFeeMultiplier;
            type RuntimeEvent = RuntimeEvent;
            type WeightToFee = <$t as AssetsConfig>::WeightToFee;
        }

        impl pallet_asset_manager::Config for Runtime {
            type AssetId = AssetId;
            type AssetRegistrar = <$t as AssetsConfig>::AssetRegistrar;
            type AssetRegistrarMetadata = <$t as AssetsConfig>::AssetRegistrarMetadata;
            type Balance = Balance;
            type ForeignAssetModifierOrigin = <$t as AssetsConfig>::ForeignAssetModifierOrigin;
            type ForeignAssetType = <$t as AssetsConfig>::AssetType;
            type RuntimeEvent = RuntimeEvent;
            type WeightInfo = <$t as AssetsWeight>::AssetManager;
        }
    };
}

pub const PALLET_NAMES: [(&str, &str); 3] = [
    ("Assets", "pallet_assets"),
    ("TransactionPayment", "pallet_transaction_payment"),
    ("AssetManager", "pallet_asset_manager"),
];
