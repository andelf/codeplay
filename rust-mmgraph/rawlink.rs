use std::mem;
use std::ptr;

pub struct Rawlink<T> {
    p: *mut T,
}

impl<T> Copy for Rawlink<T> {}
unsafe impl<T: Send> Send for Rawlink<T> {}
unsafe impl<T: Sync> Sync for Rawlink<T> {}

/// Rawlink is a type like Option<T> but for holding a raw pointer
impl<T> Rawlink<T> {
    /// Like Option::None for Rawlink
    pub fn none() -> Rawlink<T> {
        Rawlink { p: ptr::null_mut() }
    }

    /// Like Option::Some for Rawlink
    pub fn some(n: &mut T) -> Rawlink<T> {
        Rawlink { p: n }
    }

    pub unsafe fn unsafe_some(n: &T) -> Rawlink<T> {
        Rawlink { p: n as *const T as *mut T }
    }

    /// Convert the 'Rawlink' into an Option value
    ///
    /// **unsafe** because:
    ///
    /// - Dereference of raw pointer.
    /// - Returns reference of arbitrary lifetime.
    pub unsafe fn resolve<'a>(&self) -> Option<&'a T> {
        self.p.as_ref()
    }

    /// Convert the 'Rawlink' into an Option value
    ///
    /// **unsafe** because:
    ///
    /// - Dereference of raw pointer.
    /// - Returns reference of arbitrary lifetime.
    pub unsafe fn resolve_mut<'a>(&mut self) -> Option<&'a mut T> {
        self.p.as_mut()
    }

    /// Return the 'Rawlink' and replace with 'Rawlink::none()'
    pub fn take(&mut self) -> Rawlink<T> {
        mem::replace(self, Rawlink::none())
    }
}

// impl<'a, T> From<&'a mut Link<T>> for Rawlink<Node<T>> {
//     fn from(node: &'a mut Link<T>) -> Self {
//         match node.as_mut() {
//             None => Rawlink::none(),
//             Some(ptr) => Rawlink::some(ptr),
//         }
//     }
// }

impl<T> Clone for Rawlink<T> {
    #[inline]
    fn clone(&self) -> Rawlink<T> {
        Rawlink { p: self.p }
    }
}
