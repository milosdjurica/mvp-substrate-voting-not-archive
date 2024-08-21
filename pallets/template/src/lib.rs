// All pallets must be configured for `no_std`.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use scale_info::prelude::vec::Vec;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[derive(Clone, Encode, Decode, Default, TypeInfo)]
    pub struct Proposal<AccountId, BlockNumber> {
        pub creator: AccountId,
        pub description: Vec<u8>,
        pub end_block: BlockNumber,
    }

    #[derive(Clone, Encode, Decode, Default, TypeInfo)]
    pub struct FinishedProposal<AccountId, BlockNumber> {
        pub proposal: Proposal<AccountId, BlockNumber>,
        pub has_passed: bool,
    }

    #[derive(Clone, Encode, Decode, Default, TypeInfo)]
    pub struct Vote<AccountId> {
        pub voter: AccountId,
        pub vote_is_yes: bool,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ProposalCreated {
            proposal_id: u32,
            creator: T::AccountId,
            description: Vec<u8>,
            end_block: BlockNumberFor<T>,
        },

        UserVoted {
            proposal_id: u32,
            voter: T::AccountId,
            vote_is_yes: bool,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        ProposalDoesNotExist,
        ProposalIsNotActive,
        UserAlreadyVoted,
    }

    #[pallet::storage]
    #[pallet::getter(fn proposal_counter)]
    pub(super) type ProposalCounter<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn active_proposals)]
    pub(super) type ActiveProposals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32,
        Proposal<T::AccountId, BlockNumberFor<T>>,
        OptionQuery,
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(Weight::default())]
        #[pallet::call_index(0)]
        pub fn create_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
            Ok(())
        }
    }
}
