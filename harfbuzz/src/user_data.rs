use std::marker::PhantomData;

pub struct UserDataKey<T> {
    key: sys::hb_user_data_key_t,
    _marker: PhantomData<T>,
}
