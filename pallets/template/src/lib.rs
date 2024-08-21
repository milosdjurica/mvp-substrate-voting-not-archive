#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_support::sp_runtime::ArithmeticError;
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
        // TODO add max length???
        pub description: Vec<u8>,
        pub end_block: BlockNumber,
    }

    #[derive(Clone, Encode, Decode, Default, TypeInfo)]
    pub struct FinishedProposal<AccountId, BlockNumber> {
        pub proposal: Proposal<AccountId, BlockNumber>,
        pub is_approved: bool,
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

    #[pallet::storage]
    #[pallet::getter(fn finished_proposals)]
    pub(super) type FinishedProposals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32,
        FinishedProposal<T::AccountId, BlockNumberFor<T>>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn proposal_to_votes)]
    pub(super) type ProposalToVotes<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32,                     // Proposal ID
        Vec<Vote<T::AccountId>>, // List of votes
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn user_has_voted_on_proposal)]
    pub(super) type UserHasVotedOnProposal<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        (T::AccountId, u32), // AccountId, Proposal ID
        bool,                // Did vote or did not vote
        ValueQuery,
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(Weight::default())]
        #[pallet::call_index(0)]
        pub fn create_proposal(
            origin: OriginFor<T>,
            description: Vec<u8>,
            end_block: BlockNumberFor<T>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let current_id = Self::proposal_counter();
            let next_id = current_id.checked_add(1).ok_or(ArithmeticError::Overflow)?;

            let new_proposal = Proposal {
                creator: sender.clone(),
                description: description.clone(),
                end_block,
            };

            <ActiveProposals<T>>::insert(next_id, new_proposal);
            <ProposalCounter<T>>::put(next_id);

            Self::deposit_event(Event::ProposalCreated {
                proposal_id: next_id,
                creator: sender,
                description: description.clone(),
                end_block,
            });

            Ok(())
        }
    }
}
