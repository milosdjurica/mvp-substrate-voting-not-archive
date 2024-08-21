// All pallets must be configured for `no_std`.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ClaimCreated { who: T::AccountId, claim: T::Hash },
        ClaimRevoked { who: T::AccountId, claim: T::Hash },
    }

    #[pallet::error]
    pub enum Error<T> {
        AlreadyClaimed,

        NoSuchClaim,

        NotClaimOwner,
    }

    #[pallet::storage]
    pub(super) type Claims<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, (T::AccountId, BlockNumberFor<T>)>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(Weight::default())]
        #[pallet::call_index(0)]
        pub fn create_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
            Ok(())
        }
    }
}
