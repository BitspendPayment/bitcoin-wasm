// Generated by `wit-bindgen` 0.25.0. DO NOT EDIT!
// Options used:
#[allow(dead_code)]
pub mod exports {
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
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
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

                type _StoreRep<T> = Option<T>;

                impl Store {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `Store`.
                    pub fn new<T: GuestStore>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _StoreRep<T> = Some(val);
                        let ptr: *mut _StoreRep<T> = _rt::Box::into_raw(_rt::Box::new(val));
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }

                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestStore>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }

                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestStore>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }

                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestStore>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }

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

                    // It's theoretically possible to implement the `GuestStore` trait twice
                    // so guard against using it with two different types here.
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(!cfg!(target_feature = "threads"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => assert!(
                                    ty == id,
                                    "cannot use two types with this resource type"
                                ),
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }

                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _StoreRep<T>);
                    }

                    fn as_ptr<T: GuestStore>(&self) -> *mut _StoreRep<T> {
                        Store::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }

                /// A borrowed version of [`Store`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct StoreBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a Store>,
                }

                impl<'a> StoreBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }

                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestStore>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }

                    // NB: mutable access is not allowed due to the component model allowing
                    // multiple borrows of the same resource.

                    fn as_ptr<T: 'static>(&self) -> *mut _StoreRep<T> {
                        Store::type_guard::<T>();
                        self.rep.cast()
                    }
                }

                unsafe impl _rt::WasmResource for Store {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();

                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]component:store/types@0.1.0")]
                            extern "C" {
                                #[link_name = "[resource-drop]store"]
                                fn drop(_: u32);
                            }

                            drop(_handle);
                        }
                    }
                }

                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_constructor_store_cabi<T: GuestStore>() -> i32 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let result0 = Store::new(T::new());
                    (result0).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_store_insert_cabi<T: GuestStore>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: *mut u8,
                    arg4: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let len1 = arg4;
                    let bytes1 = _rt::Vec::from_raw_parts(arg3.cast(), len1, len1);
                    let result2 = T::insert(
                        StoreBorrow::lift(arg0 as u32 as usize).get(),
                        KeyValuePair {
                            key: _rt::string_lift(bytes0),
                            value: _rt::string_lift(bytes1),
                        },
                    );
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result2 {
                        Ok(_) => {
                            *ptr3.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr3.add(0).cast::<u8>() = (1i32) as u8;
                            *ptr3.add(1).cast::<u8>() = (e.clone() as i32) as u8;
                        }
                    };
                    ptr3
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_store_search_cabi<T: GuestStore>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let result1 = T::search(
                        StoreBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let KeyValuePair {
                                key: key3,
                                value: value3,
                            } = e;
                            let vec4 = (key3.into_bytes()).into_boxed_slice();
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            ::core::mem::forget(vec4);
                            *ptr2.add(8).cast::<usize>() = len4;
                            *ptr2.add(4).cast::<*mut u8>() = ptr4.cast_mut();
                            let vec5 = (value3.into_bytes()).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *ptr2.add(16).cast::<usize>() = len5;
                            *ptr2.add(12).cast::<*mut u8>() = ptr5.cast_mut();
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            *ptr2.add(4).cast::<u8>() = (e.clone() as i32) as u8;
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_store_search<T: GuestStore>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                            let l3 = *arg0.add(12).cast::<*mut u8>();
                            let l4 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l3, l4, 1);
                        }
                        _ => (),
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_store_delete_cabi<T: GuestStore>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let result1 = T::delete(
                        StoreBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(_) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            *ptr2.add(1).cast::<u8>() = (e.clone() as i32) as u8;
                        }
                    };
                    ptr2
                }
                pub trait Guest {
                    type Store: GuestStore;
                }
                pub trait GuestStore: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }

                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]component:store/types@0.1.0")]
                            extern "C" {
                                #[link_name = "[resource-new]store"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }

                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }

                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]component:store/types@0.1.0")]
                            extern "C" {
                                #[link_name = "[resource-rep]store"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }

                    fn new() -> Self;
                    fn insert(&self, kv: KeyValuePair) -> Result<(), Error>;
                    fn search(&self, key: Key) -> Result<KeyValuePair, Error>;
                    fn delete(&self, key: Key) -> Result<(), Error>;
                }
                #[doc(hidden)]

                macro_rules! __export_component_store_types_0_1_0_cabi{
  ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

    #[export_name = "component:store/types@0.1.0#[constructor]store"]
    unsafe extern "C" fn export_constructor_store() -> i32 {
      $($path_to_types)*::_export_constructor_store_cabi::<<$ty as $($path_to_types)*::Guest>::Store>()
    }
    #[export_name = "component:store/types@0.1.0#[method]store.insert"]
    unsafe extern "C" fn export_method_store_insert(arg0: *mut u8,arg1: *mut u8,arg2: usize,arg3: *mut u8,arg4: usize,) -> *mut u8 {
      $($path_to_types)*::_export_method_store_insert_cabi::<<$ty as $($path_to_types)*::Guest>::Store>(arg0, arg1, arg2, arg3, arg4)
    }
    #[export_name = "component:store/types@0.1.0#[method]store.search"]
    unsafe extern "C" fn export_method_store_search(arg0: *mut u8,arg1: *mut u8,arg2: usize,) -> *mut u8 {
      $($path_to_types)*::_export_method_store_search_cabi::<<$ty as $($path_to_types)*::Guest>::Store>(arg0, arg1, arg2)
    }
    #[export_name = "cabi_post_component:store/types@0.1.0#[method]store.search"]
    unsafe extern "C" fn _post_return_method_store_search(arg0: *mut u8,) {
      $($path_to_types)*::__post_return_method_store_search::<<$ty as $($path_to_types)*::Guest>::Store>(arg0)
    }
    #[export_name = "component:store/types@0.1.0#[method]store.delete"]
    unsafe extern "C" fn export_method_store_delete(arg0: *mut u8,arg1: *mut u8,arg2: usize,) -> *mut u8 {
      $($path_to_types)*::_export_method_store_delete_cabi::<<$ty as $($path_to_types)*::Guest>::Store>(arg0, arg1, arg2)
    }

    const _: () = {
      #[doc(hidden)]
      #[export_name = "component:store/types@0.1.0#[dtor]store"]
      #[allow(non_snake_case)]
      unsafe extern "C" fn dtor(rep: *mut u8) {
        $($path_to_types)*::Store::dtor::<
        <$ty as $($path_to_types)*::Guest>::Store
        >(rep)
      }
    };

  };);
}
                #[doc(hidden)]
                pub(crate) use __export_component_store_types_0_1_0_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 20]);
                static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 20]);
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
    pub use alloc_crate::boxed::Box;

    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr as *mut u8, layout);
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
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

macro_rules! __export_store_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::component::store::types::__export_component_store_types_0_1_0_cabi!($ty with_types_in $($path_to_types_root)*::exports::component::store::types);
  )
}
#[doc(inline)]
pub(crate) use __export_store_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.25.0:store:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 438] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xba\x02\x01A\x02\x01\
A\x02\x01B\x13\x01r\x02\x03keys\x05values\x04\0\x0ekey-value-pair\x03\0\0\x01s\x04\
\0\x03key\x03\0\x02\x01m\x01\x03nae\x04\0\x05error\x03\0\x04\x04\0\x05store\x03\x01\
\x01i\x06\x01@\0\0\x07\x04\0\x12[constructor]store\x01\x08\x01h\x06\x01j\0\x01\x05\
\x01@\x02\x04self\x09\x02kv\x01\0\x0a\x04\0\x14[method]store.insert\x01\x0b\x01j\
\x01\x01\x01\x05\x01@\x02\x04self\x09\x03key\x03\0\x0c\x04\0\x14[method]store.se\
arch\x01\x0d\x01@\x02\x04self\x09\x03key\x03\0\x0a\x04\0\x14[method]store.delete\
\x01\x0e\x04\x01\x1bcomponent:store/types@0.1.0\x05\0\x04\x01\x1bcomponent:store\
/store@0.1.0\x04\0\x0b\x0b\x01\0\x05store\x03\0\0\0G\x09producers\x01\x0cprocess\
ed-by\x02\x0dwit-component\x070.208.1\x10wit-bindgen-rust\x060.25.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}