#![cfg_attr(not(feature = "std"), no_std)]
#![feature(associated_type_defaults)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use frame_support::traits::{Currency, Randomness};

    // The basis which we buil
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // Allows easy access our Pallet's `Balance` type. Comes from `Currency` interface.
    type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    // The Gender type used in the `Kitty` struct
    #[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum Gender {
        Male,
        Female,
    }

    // Struct for holding kitty information
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    #[scale_info(skip_type_params(T))]
    pub struct Kitty<T: Config> {
        // Using 16 bytes to represent a kitty DNA
        pub dna: [u8; 16],
        // `None` assumes not for sale
        pub price: Option<BalanceOf<T>>,
        pub gender: Gender,
        pub owner: T::AccountId,
    }

    /// Keeps track of the number of kitties in existence.
    #[pallet::storage]
    pub(super) type CountForKitties<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// Maps the kitty struct to the kitty DNA.
    #[pallet::storage]
    pub(super) type Kitties<T: Config> = StorageMap<_, Twox64Concat, [u8; 16], Kitty<T>>;

    /// Track the kitties owned by each account.
    #[pallet::storage]
    pub(super) type KittiesOwned<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        BoundedVec<[u8; 16], T::MaxKittiesOwned>,
        ValueQuery,
    >;

    // Your Pallet's configuration trait, representing custom external types and interfaces.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The Currency handler for the kitties pallet.
        type Currency: Currency<Self::AccountId>;

        /// The maximum amount of kitties a single account can own.
        #[pallet::constant]
        type MaxKittiesOwned: Get<u32>;

        type BlockNumber = Self::BlockNumber;

        /// The type of Randomness we want to specify for this pallet.
        type KittyRandomness: Randomness<Self::Hash, Self::BlockNumber>;
    }

    // Your Pallet's events.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new kitty was successfully created.
        Created { kitty: [u8; 16], owner: T::AccountId },
    }

    // Your Pallet's error messages.
    #[pallet::error]
    pub enum Error<T> {
        /// An account may only own `MaxKittiesOwned` kitties.
        TooManyOwned,
        /// This kitty already exists!
        DuplicateKitty,
        /// An overflow has occured!
        Overflow,
    }

    // Your Pallet's callable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new unique kitty.
        ///
        /// The actual kitty creation is done in the `mint()` function.
        #[pallet::weight(0)]
        pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
            // Make sure the caller is from a signed origin
            let sender = ensure_signed(origin)?;

            // Generate unique DNA and Gender using a helper function
            let (kitty_gen_dna, gender) = Self::gen_dna();

            // Write new kitty to storage by calling helper function
            Self::mint(&sender, kitty_gen_dna, gender)?;

            Ok(())
        }
    }

    // Your Pallet's internal functions.
    impl<T: Config> Pallet<T> {
        // Generates and returns DNA and Gender
        fn gen_dna() -> ([u8; 16], Gender) {
            // Create randomness
            let random = T::KittyRandomness::random(&b"dna"[..]).0;

            // Create randomness payload. Multiple kitties can be generated in the same block,
            // retaining uniqueness.
            let unique_payload = (
                random,
                frame_system::Pallet::<T>::extrinsic_index().unwrap_or_default(),
                frame_system::Pallet::<T>::block_number(),
            );

            // Turns into a byte array
            let encoded_payload = unique_payload.encode();
            let hash = frame_support::Hashable::blake2_128(&encoded_payload);

            // Generate Gender
            if hash[0] % 2 == 0 {
                (hash, Gender::Male)
            } else {
                (hash, Gender::Female)
            }
        }

        // Helper to mint a kitty
        pub fn mint(
            owner: &T::AccountId,
            dna: [u8; 16],
            gender: Gender,
        ) -> Result<[u8; 16], DispatchError> {
            // Create a new object
            let kitty = Kitty::<T> { dna, price: None, gender, owner: owner.clone() };

            // Check if the kitty does not already exist in our storage map
            ensure!(!Kitties::<T>::contains_key(&kitty.dna), Error::<T>::DuplicateKitty);

            // Performs this operation first as it may fail
            let count = CountForKitties::<T>::get();
            let new_count = count.checked_add(1).ok_or(Error::<T>::Overflow)?;

            // Append kitty to KittiesOwned
            KittiesOwned::<T>::try_append(&owner, kitty.dna)
                .map_err(|_| Error::<T>::TooManyOwned)?;

            // Write new kitty to storage
            Kitties::<T>::insert(kitty.dna, kitty);
            CountForKitties::<T>::put(new_count);

            // Deposit our "Created" event.
            Self::deposit_event(Event::Created { kitty: dna, owner: owner.clone() });

            // Returns the DNA of the new kitty if this succeeds
            Ok(dna)
        }
    }
}
