#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
use parity_codec::Encode;
use support::{StorageValue, dispatch::Result, decl_module, decl_storage};
use runtime_primitives::traits::Hash;
use {balances, system::{self, ensure_signed}};

pub trait Trait: balance::Trait {}

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as Demo {
		Payment get(payment): Option<T::Balance>;
    	Pot get(pot): T::Balance;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn play(origin) -> Result{
			//ゲームをプレイするロジック
			let sender = ensure_signed(origin)?;
			//decl_strage!で宣言しているからpaymentが使える
			let payment = Self::payment().ok_or("Must have payment amount set")?;
	　		//senderの残高を減少させる
			<balances::Module<T>>::decrease_free_balance(&sender, payment)?;

			//ハッシュ関数を通してハッシュ値の最初のbyteが128以下であれば勝ち。potにあった金額がSenderに払われる
			if (<system::Module<T>>::random_seed(), &sender).using_encoded(<T as system::Trait>::Hashing::hash).using_encoded(|e| e[0] < 128) {
				<balances::Module<T>>::increase_free_balance_creating(&sender, <Pot<T>>::take());
			}
	　
			//結果どうあれ、senderが賭けに参加した金額がデポジットされる
			<Pot<T>>::mutate(|pot| *pot += payment);

			Ok(())
		}
		
		fn set_payment(_origin, value: T::Balance) -> Result{
			//イニシャルpaymentがセットされていない場合の処理
			if Self::payment().is_none() {

				<Payment<T>>::put(value);
				<Pot<T>>::put(value);
			}

			Ok(())
        }
	}
}
