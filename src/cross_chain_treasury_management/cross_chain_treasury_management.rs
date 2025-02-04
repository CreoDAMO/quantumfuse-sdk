use frame_support::{decl_module, decl_storage, decl_event, dispatch};
use sp_runtime::traits::Zero;
use sp_std::vec::Vec;

pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
    trait Store for Module<T: Config> as QuantumTreasury {
        Reserves get(fn reserves): u128;
        Bonds get(fn bonds): Vec<Bond>;
    }
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, Default)]
pub struct Bond {
    pub investor: T::AccountId,
    pub amount: u128,
    pub interest_rate: u8,
    pub maturity: T::BlockNumber,
}

decl_event!(
    pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
        BondIssued(AccountId, u128, u8, T::BlockNumber),
        ReservesUpdated(u128),
    }
);

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        #[weight = 10_000]
        pub fn issue_bond(origin, amount: u128, interest_rate: u8, maturity: T::BlockNumber) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            <Bonds<T>>::append(Bond { investor: sender.clone(), amount, interest_rate, maturity });
            Self::deposit_event(RawEvent::BondIssued(sender, amount, interest_rate, maturity));
            Ok(())
        }

        #[weight = 10_000]
        pub fn adjust_reserves(origin, new_amount: u128) -> dispatch::DispatchResult {
            ensure_root(origin)?;
            <Reserves>::put(new_amount);
            Self::deposit_event(RawEvent::ReservesUpdated(new_amount));
            Ok(())
        }
    }
}
