#![feature(associated_type_defaults)]
#![cfg_attr(not(feature = "std"), no_std)]

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
    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

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

    /* Placeholder for defining custom storage items. */

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

        // This line ensures that your pallet's configuration trait provides a BlockNumber type
        type BlockNumber = Self::BlockNumber;
        /// The type of Randomness we want to specify for this pallet.
        type KittyRandomness: Randomness<Self::Hash, Self::BlockNumber>;
    }

    // Your Pallet's events.
    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {}

    // Your Pallet's error messages.
    #[pallet::error]
    pub enum Error<T> {}

    // Your Pallet's callable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    // Your Pallet's internal functions.
    impl<T: Config> Pallet<T> {}
}
