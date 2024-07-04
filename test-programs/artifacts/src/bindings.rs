// Generated by `wit-bindgen` 0.25.0. DO NOT EDIT!
// Options used:
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_test_store_cabi<T: Guest>() {
    #[cfg(target_arch = "wasm32")]
    _rt::run_ctors_once();
    T::test_store();
}
pub trait Guest {
    fn test_store();
}
#[doc(hidden)]

macro_rules! __export_world_example_cabi{
  ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

    #[export_name = "test-store"]
    unsafe extern "C" fn export_test_store() {
      $($path_to_types)*::_export_test_store_cabi::<$ty>()
    }
  };);
}
#[doc(hidden)]
pub(crate) use __export_world_example_cabi;
#[allow(dead_code)]
pub mod component {
    #[allow(dead_code)]
    pub mod store {
        #[allow(dead_code, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[derive(Clone)]
            pub struct KeyValuePair {
                pub key: _rt::String,
                pub value: _rt::String,
            }
            impl ::core::fmt::Debug for KeyValuePair {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("KeyValuePair")
                        .field("key", &self.key)
                        .field("value", &self.value)
                        .finish()
                }
            }
            pub type Key = _rt::String;
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, PartialEq)]
            pub enum Error {
                Nae,
            }
            impl Error {
                pub fn name(&self) -> &'static str {
                    match self {
                        Error::Nae => "nae",
                    }
                }
                pub fn message(&self) -> &'static str {
                    match self {
                        Error::Nae => "",
                    }
                }
            }
            impl ::core::fmt::Debug for Error {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("Error")
                        .field("code", &(*self as i32))
                        .field("name", &self.name())
                        .field("message", &self.message())
                        .finish()
                }
            }
            impl ::core::fmt::Display for Error {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    write!(f, "{} (error {})", self.name(), *self as i32)
                }
            }

            impl std::error::Error for Error {}

            impl Error {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> Error {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }

                    match val {
                        0 => Error::Nae,

                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }

            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Store {
                handle: _rt::Resource<Store>,
            }

            impl Store {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }

                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }

                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }

            unsafe impl _rt::WasmResource for Store {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();

                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "component:store/types")]
                        extern "C" {
                            #[link_name = "[resource-drop]store"]
                            fn drop(_: u32);
                        }

                        drop(_handle);
                    }
                }
            }

            impl Store {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new() -> Self {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "component:store/types")]
                        extern "C" {
                            #[link_name = "[constructor]store"]
                            fn wit_import() -> i32;
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import() -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import();
                        Store::from_handle(ret as u32)
                    }
                }
            }
            impl Store {
                #[allow(unused_unsafe, clippy::all)]
                pub fn insert(&self, kv: &KeyValuePair) -> Result<(), Error> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let KeyValuePair {
                            key: key0,
                            value: value0,
                        } = kv;
                        let vec1 = key0;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let vec2 = value0;
                        let ptr2 = vec2.as_ptr().cast::<u8>();
                        let len2 = vec2.len();
                        let ptr3 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "component:store/types")]
                        extern "C" {
                            #[link_name = "[method]store.insert"]
                            fn wit_import(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                            );
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            ptr1.cast_mut(),
                            len1,
                            ptr2.cast_mut(),
                            len2,
                            ptr3,
                        );
                        let l4 = i32::from(*ptr3.add(0).cast::<u8>());
                        match l4 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l5 = i32::from(*ptr3.add(1).cast::<u8>());

                                    Error::_lift(l5 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Store {
                #[allow(unused_unsafe, clippy::all)]
                pub fn search(&self, key: &Key) -> Result<KeyValuePair, Error> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 20]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 20]);
                        let vec0 = key;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "component:store/types")]
                        extern "C" {
                            #[link_name = "[method]store.search"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = {
                                    let l3 = *ptr1.add(4).cast::<*mut u8>();
                                    let l4 = *ptr1.add(8).cast::<usize>();
                                    let len5 = l4;
                                    let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                                    let l6 = *ptr1.add(12).cast::<*mut u8>();
                                    let l7 = *ptr1.add(16).cast::<usize>();
                                    let len8 = l7;
                                    let bytes8 = _rt::Vec::from_raw_parts(l6.cast(), len8, len8);

                                    KeyValuePair {
                                        key: _rt::string_lift(bytes5),
                                        value: _rt::string_lift(bytes8),
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l9 = i32::from(*ptr1.add(4).cast::<u8>());

                                    Error::_lift(l9 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Store {
                #[allow(unused_unsafe, clippy::all)]
                pub fn delete(&self, key: &Key) -> Result<(), Error> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let vec0 = key;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "component:store/types")]
                        extern "C" {
                            #[link_name = "[method]store.delete"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(1).cast::<u8>());

                                    Error::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;

    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};

    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        // NB: This would ideally be `u32` but it is not. The fact that this has
        // interior mutability is not exposed in the API of this type except for the
        // `take_handle` method which is supposed to in theory be private.
        //
        // This represents, almost all the time, a valid handle value. When it's
        // invalid it's stored as `u32::MAX`.
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }

    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }

    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }

        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }

        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }

    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource")
                .field("handle", &self.handle)
                .finish()
        }
    }

    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    // If this handle was "taken" then don't do anything in the
                    // destructor.
                    u32::MAX => {}

                    // ... but otherwise do actually destroy it with the imported
                    // component model intrinsic as defined through `T`.
                    other => T::drop(other),
                }
            }
        }
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    extern crate alloc as alloc_crate;
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_example_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::__export_world_example_cabi!($ty with_types_in $($path_to_types_root)*);
  )
}
#[doc(inline)]
pub(crate) use __export_example_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.25.0:example:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 449] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xc3\x02\x01A\x02\x01\
A\x04\x01B\x13\x01r\x02\x03keys\x05values\x04\0\x0ekey-value-pair\x03\0\0\x01s\x04\
\0\x03key\x03\0\x02\x01m\x01\x03nae\x04\0\x05error\x03\0\x04\x04\0\x05store\x03\x01\
\x01i\x06\x01@\0\0\x07\x04\0\x12[constructor]store\x01\x08\x01h\x06\x01j\0\x01\x05\
\x01@\x02\x04self\x09\x02kv\x01\0\x0a\x04\0\x14[method]store.insert\x01\x0b\x01j\
\x01\x01\x01\x05\x01@\x02\x04self\x09\x03key\x03\0\x0c\x04\0\x14[method]store.se\
arch\x01\x0d\x01@\x02\x04self\x09\x03key\x03\0\x0a\x04\0\x14[method]store.delete\
\x01\x0e\x03\x01\x15component:store/types\x05\0\x01@\0\x01\0\x04\0\x0atest-store\
\x01\x01\x04\x01\x16test:artifacts/example\x04\0\x0b\x0d\x01\0\x07example\x03\0\0\
\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.208.1\x10wit-bind\
gen-rust\x060.25.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
