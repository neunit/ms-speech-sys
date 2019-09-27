//! Macros for crate local usage of D.R.Y.

/// Export internal properties.
#[macro_export]
macro_rules! FlattenProps {
    ($name:ident) => {
        impl $crate::properties::PropertyBag for $name {
            fn get_by_id(&self, id: $crate::api::PropertyId) -> Result<String> {
                self.props.get_by_id(id)
            }

            fn get_by_name(&self, name: &str) -> $crate::Result<String> {
                self.props.get_by_name(name)
            }

            fn put_by_id<T: ToString>(&self, id: $crate::api::PropertyId, value: T) -> Result<()> {
                self.props.put_by_id(id, value)
            }

            fn put_by_name<T: ToString>(&self, name: &str, value: T) -> Result<()> {
                self.props.put_by_name(name, value)
            }
        }
    };
}

/// A quick and dirty implementation of derive Handle trait.
#[macro_export]
macro_rules! DeriveHandle {
    ( $name:ident, $t:ty ,$release:ident ) => {
        /// Derive the trait used to get underlying handle value.
        impl $crate::Handle<$t> for $name {
            fn handle(&self) -> $t {
                self.handle
            }
        }

        /// Drop the handle by underlying destructor.
        impl Drop for $name {
            fn drop(&mut self) {
                self.$release();
                self.handle = $crate::INVALID_HANDLE;
            }
        }

        /// Enable threading operation.
        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}
    };

    ( $name:ident, $t:ty ,$release:ident, $check:ident ) => {
        /// Derive the trait used to get underlying handle value.
        impl $crate::Handle<$t> for $name {
            fn handle(&self) -> $t {
                self.handle
            }
        }

        /// Drop the handle by underlying destructor.
        impl Drop for $name {
            fn drop(&mut self) {
                unsafe {
                    if !$check(self.handle) {
                        return;
                    }
                    $release(self.handle);
                }
                self.handle = $crate::INVALID_HANDLE;
            }
        }

        /// Enable threading operation.
        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}
    };
}

/// Compact version to wrap underlying handle.
#[macro_export]
macro_rules! SmartHandle {
    ( $name:ident, $t:ty, $release:ident ) => {
        $crate::DeriveHandle!($name, $t, $release);

        /// Wrap the underlying handle with common methods.
        pub struct $name {
            handle: $t,
        }

        impl $name {
            /// Constructor.
            pub fn new(handle: $t) -> Self {
                $name { handle }
            }
        }

        /// Syntactic sugar for dereference.
        impl std::ops::Deref for $name {
            type Target = $t;

            fn deref(&self) -> &$t {
                &self.handle
            }
        }

        /// Convert from underlying handle.
        impl From<$t> for $name {
            fn from(handle: $t) -> Self {
                Self::new(handle)
            }
        }

        /// For somewhere need default implementation.
        impl Default for $name {
            fn default() -> Self {
                Self::new($crate::INVALID_HANDLE as $t)
            }
        }
    };
    ( $name:ident, $t:ty, $release:ident, $check:ident ) => {
        $crate::DeriveHandle!($name, $t, $release, $check);

        /// Wrap the underlying handle with common methods.
        pub struct $name {
            handle: $t,
        }

        impl $name {
            /// Constructor.
            pub fn new(handle: $t) -> Self {
                $name { handle }
            }

            /// Validator.
            pub fn is_valid(&self) -> bool {
                unsafe { $check(self.handle) }
            }

            /// Destructor.
            pub fn release(&mut self) {
                if self.is_valid() {
                    unsafe { $release(self.handle) };
                }
                self.handle = $crate::INVALID_HANDLE as $t;
            }
        }

        /// Syntactic sugar for dereference.
        impl std::ops::Deref for $name {
            type Target = $t;

            fn deref(&self) -> &$t {
                &self.handle
            }
        }

        /// Convert from underlying handle.
        impl From<$t> for $name {
            fn from(handle: $t) -> Self {
                Self::new(handle)
            }
        }

        /// For somewhere need default implementation.
        impl Default for $name {
            fn default() -> Self {
                Self::new($crate::INVALID_HANDLE as $t)
            }
        }
    };
    ( $name:ident, $t:ty, $release:ident, $check:ident ,$props:ident ) => {
        $crate::DeriveHandle!($name, $t, $release, $check);

        /// Wrap the underlying handle with common methods.
        pub struct $name {
            handle: $t,
            props: $crate::Properties,
        }

        impl $name {
            /// Constructor.
            pub fn new(handle: $t) -> Self {
                let mut hprops = INVALID_HANDLE;
                unsafe { $props(handle, &mut hprops) };
                $name { handle, props: $crate::Properties::new(hprops) }
            }
        }

        $crate::FlattenProps!($name);
    };
}

/// Pre-alloc buffer to retrieve string of ffi.
#[macro_export]
macro_rules! ffi_get_string {
    ($f:ident, $h:expr $(, $sz:expr)?) => ({
        let _max_len = 1024;
        $(
            let _max_len = $sz;
        )?
        let mut s: Vec<u8> = vec![0; _max_len];
        let buf_ptr = s.as_mut_ptr() as *mut ::std::os::raw::c_char;
        unsafe {
            r#try!($crate::from_hr($f($h, buf_ptr, _max_len as u32)));
            let output = ::std::ffi::CStr::from_ptr(buf_ptr);
            r#try!(output.to_str().map(String::from))
        }
    })
}

/// From hresult to Result.
#[macro_export]
macro_rules! hr {
    ($ffi:expr) => {
        $crate::from_hr(unsafe { $ffi })
    };
}
