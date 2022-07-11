#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
  use codec::{Codec, FullEncode};
  use frame_support::{pallet_prelude::*};
  use frame_system::pallet_prelude::*;

  // Every pallet has a `Config` trait. It contains everything configuration-related. A
  // non-exhaustive list of examples below. In general, anything that you can do with a trait, you
  // can do in your config.
  //
  // Note that we inherit from everything system!
  #[pallet::config]
  pub trait Config: frame_system::Config {
    // A hook about what to do.
    fn on_value_update(new_value: Self::ValueType);
    // An example of receiving an input type.
    type ValueType: From<u32> + Codec + Default + TypeInfo + FullEncode + sp_std::ops::AddAssign + PartialOrd + Copy;
    // An example of receiving a const.
    const MAX_VALUE: u32;
  }

  // T::AccountId comes from system, T::ValueType comes form us.
  #[pallet::storage]
  pub type Accumulators<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, T::ValueType, ValueQuery>;
  // A simpler storage item.
  #[pallet::storage]
  pub type Counter<T: Config> = StorageValue<_, u32, ValueQuery>;

  #[pallet::pallet]
  #[pallet::without_storage_info]
  pub struct Pallet<T>(PhantomData<T>);

  #[pallet::call]
  impl<T: Config> Pallet<T> {
      // this a dummy transaction that allows any user to submit a number (that is converted to
      // `ValueType`) exactly once. It already increments a counter every time someone submits something
      // new. A maximum of `Config::MAX_VALUE` is allowed, and the associated hooks are called.
      #[pallet::weight(0)]
      pub fn inc_user_counter(origin: OriginFor<T>, inc: u32) -> DispatchResult {
        // checks the origin to be signed -- more on this later.
        let who = ensure_signed(origin)?;

        // check that this user has not submitted already.
        if !<Accumulators<T>>::contains_key(&who) {
          // increment the counter .
          Counter::<T>::mutate(|x| *x += 1);

          // get the accumulator associated with this user.
          let mut current = Accumulators::<T>::get(&who);
          // convert `inc` to `ValueType`.
          current += inc.into();
          // check reaching .
          if current > T::MAX_VALUE.into() {
            return Err("failed".into())
          }

          // all good! call hook..
          T::on_value_update(current);
          // ..and write to storage.
          Accumulators::<T>::insert(who, current);
        } else {
          return Err("already submitted".into())
        }

        Ok(())
      }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
      fn on_initialize(n: T::BlockNumber) -> Weight {
        if n % 10u32.into() == sp_runtime::traits::Zero::zero() {
          log::info!("count of users is {}", Counter ::<T>::get());
        }
        0
      }

      fn on_finalize(_n: T::BlockNumber) {
        // other stuff..
      }
    }
  }
