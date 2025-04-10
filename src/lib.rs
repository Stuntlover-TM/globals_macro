pub use once_cell;
use once_cell::sync::Lazy;
pub use parking_lot;
use parking_lot::RwLock;
use std::ops::Deref;

// Wrapper structs for clear trait resolution
pub struct GlobalVar<T>(pub Lazy<RwLock<T>>);
pub struct GlobalConst<T>(pub Lazy<T>);

#[macro_export]
macro_rules! globals {
    {$(
        $name:ident : $ty:ty $(= $expr:expr)?
    ),* $(,)?} => {
        $(
            #[allow(non_upper_case_globals)]
            static $name: $crate::GlobalVar<$ty> = $crate::GlobalVar(
                $crate::once_cell::sync::Lazy::new(|| $crate::parking_lot::RwLock::new(
                        globals!(@init_expr $ty, $($expr)?)
                    )
                ));
        )*
    };
    (@init_expr $ty:ty, $expr:expr) => { $expr };
    (@init_expr $ty:ty,) => { <$ty>::default() };
}

#[macro_export]
macro_rules! const_globals {
    {$(
        $name:ident : $ty:ty $(= $expr:expr)?
    ),* $(,)?} => {
        $(
            #[allow(non_upper_case_globals)]
            static $name: $crate::GlobalConst<$ty> = $crate::GlobalConst(
                $crate::once_cell::sync::Lazy::new(||
                    globals!(@init_expr $ty, $($expr)?)
                ));
        )*
    };
    (@init_expr $ty:ty, $expr:expr) => { $expr };
    (@init_expr $ty:ty,) => { <$ty>::default() };
}

pub trait GlobalVarExt<T> {
    fn get(&self) -> T where T: Clone;
    fn get_with<F, R>(&self, f: F) -> R where F: FnOnce(&T) -> R;
    fn set(&self, value: T);
    fn update<F>(&self, f: F) where F: FnOnce(&mut T);
}

pub trait GlobalConstExt<T> {
    fn get(&self) -> T where T: Clone;
    fn get_with<F, R>(&self, f: F) -> R where F: FnOnce(&T) -> R;
}

impl<T> GlobalVarExt<T> for GlobalVar<T> {
    fn get(&self) -> T where T: Clone {
        self.0.read().clone()
    }

    fn get_with<F, R>(&self, f: F) -> R where F: FnOnce(&T) -> R {
        f(&self.0.read())
    }

    fn set(&self, value: T) {
        *self.0.write() = value;
    }

    fn update<F>(&self, f: F) where F: FnOnce(&mut T) {
        f(&mut *self.0.write());
    }
}

impl<T> GlobalConstExt<T> for GlobalConst<T> {
    fn get(&self) -> T where T: Clone {
        self.0.deref().clone()
    }

    fn get_with<F, R>(&self, f: F) -> R where F: FnOnce(&T) -> R {
        f(self.0.deref())
    }
}