pub use once_cell;
pub use parking_lot;
use once_cell::sync::Lazy;
use parking_lot::RwLock;

#[macro_export]
macro_rules! globals {
    {$(
        $name:ident : $ty:ty $(= $expr:expr)?
    ),* $(,)?} => {
        $(
            #[allow(non_upper_case_globals)]
            static $name: $crate::once_cell::sync::Lazy<$crate::parking_lot::RwLock<$ty>> = 
                $crate::once_cell::sync::Lazy::new(|| $crate::parking_lot::RwLock::new(
                    globals!(@init_expr $ty, $($expr)?)
                ));
        )*
    };
    
    (@init_expr $ty:ty, $expr:expr) => { $expr };
    (@init_expr $ty:ty,) => { <$ty>::default() };
}

pub trait GlobalVar<T> {
    fn set(&self, data: T);
    
    fn get(&self) -> T where T: Clone;
    
    fn get_with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R;
    
    fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T);
}

impl<T> GlobalVar<T> for Lazy<RwLock<T>> {
    #[inline(always)]
    fn set(&self, data: T) {
        let mut var = self.write();
        *var = data;
    }
    
    fn get(&self) -> T 
    where
        T: Clone,
    {
        self.read().clone()
    }
    
    fn get_with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        f(&self.read())
    }
    
    fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        let mut var = self.write();
        f(&mut *var);
    }
}