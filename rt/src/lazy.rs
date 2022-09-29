//! Completely stolen from https://github.com/mgattozzi/whorl/blob/5e61714acf7d29e2b97cd26f6f0587060fa652cf/src/lib.rs#L306
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::Once;

/// We want to have a static value that's set at runtime and this executor will
/// only use libstd. As of 10/26/21, the lazy types in std are still only on
/// nightly and we can't use another crate, so crates like `once_cell` and
/// `lazy_static` are also out. Thus, we create our own Lazy type so that it will
/// calculate the value only once and only when we need it.
pub struct Lazy<T> {
    /// `Once` is a neat synchronization primitive that we just talked about
    /// and this is where we need it! We want to make sure we only write into
    /// the value of the Lazy type once and only once. Otherwise we'd have some
    /// really bad things happen if we let static values be mutated. It'd break
    /// thread safety!
    once: Once,
    /// The cell is where we hold our data. The use of `UnsafeCell` is what lets
    /// us sidestep Rust's guarantees, provided we actually use it correctly and
    /// still uphold those guarantees. Rust can't always validate that
    /// everything is safe, even if it is, and so the flexibility it provides
    /// with certain library types and unsafe code lets us handle those cases
    /// where the compiler cannot possibly understand it's okay. We also use the
    /// `MaybeUninit` type here to avoid undefined behavior with uninitialized
    /// data. We'll need to drop the inner value ourselves though to avoid
    /// memory leaks because data may not be initialized and so the type won't
    /// call drop when it's not needed anymore. We could get away with not doing
    /// it though since we're only using it for static values, but let's be
    /// thorough here!
    cell: UnsafeCell<MaybeUninit<T>>,
}

impl<T> Lazy<T> {
    /// We must construct the type using a const fn so that it can be used in
    /// `static` contexts. The nice thing is that all of the function calls we
    /// make here are also const and so this will just work. The compiler will
    /// figure it all out and make sure the `Lazy` static value exists in our
    /// final binary.
    pub const fn new() -> Self {
        Self {
            once: Once::new(),
            cell: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }
    /// We want a way to check if we have initialized the value so that we can
    /// get the value from cell without causing who knows what kind of bad
    /// things if we read garbage data.
    fn is_initialized(&self) -> bool {
        self.once.is_completed()
    }

    /// This function will either grab a reference to the type or creates it
    /// with a given function
    pub fn get_or_init(&self, func: fn() -> T) -> &T {
        self.once.call_once(|| {
            // /!\ SAFETY /!\: We only ever write to the cell once
            //
            // We first get a `*mut MaybeUninit` to the cell and turn it into a
            // `&mut MaybeUninit`. That's when we call `write` on `MaybeUninit`
            // to pass the value of the function into the now initialized
            // `MaybeUninit`.
            (unsafe { &mut *self.cell.get() }).write(func());
        });
        // /!\ SAFETY /!\: We already made sure `Lazy` was initialized with our call to
        // `call_once` above
        //
        // We now want to actually retrieve the value we wrote so that we can
        // use it! We get the `*mut MaybeUninit` from the cell and turn it into
        // a `&MaybeUninit` which then lets us call `assume_init_ref` to get
        // the `&T`. This function - much like `get` - is also unsafe, but since we
        // know that the value is initialized it's fine to call this!
        unsafe { &(*self.cell.get()).assume_init_ref() }
    }
}

/// We now need to implement `Drop` by hand specifically because `MaybeUninit`
/// will need us to drop the value it holds by ourselves only if it exists. We
/// check if the value exists, swap it out with an uninitialized value and then
/// change `MaybeUninit<T>` into just a `T` with a call to `assume_init` and
/// then call `drop` on `T` itself
impl<T> Drop for Lazy<T> {
    fn drop(&mut self) {
        if self.is_initialized() {
            let old = std::mem::replace(unsafe { &mut *self.cell.get() }, MaybeUninit::uninit());
            drop(unsafe { old.assume_init() });
        }
    }
}

/// Now you might be asking yourself why we are implementing these traits by
/// hand and also why it's unsafe to do so. `UnsafeCell`is the big reason here
/// and you can see this by commenting these two lines and trying to compile the
/// code. Because of how auto traits work then if any part is not `Send` and
/// `Sync` then we can't use `Lazy` for a static. Note that auto traits are a
/// compiler specific thing where if everything in a type implements a trait
/// then that type also implements it. `Send` and `Sync` are great examples of
/// this where any type becomes `Send` and/or `Sync` if all its types implement
/// them too! `UnsafeCell` specifically implements !Sync and since it is not
/// `Sync` then it can't be used in a `static`. We can override this behavior
/// though by implementing these traits for `Lazy` here though. We're saying
/// that this is okay and that we uphold the invariants to be `Send + Sync`. We
/// restrict it though and say that this is only the case if the type `T`
/// *inside* `Lazy` is `Sync` only if `T` is `Send + Sync`. We know then that
/// this is okay because the type in `UnsafeCell` can be safely referenced
/// through an `&'static` and that the type it holds is also safe to use across
/// threads. This means we can set `Lazy` as `Send + Sync` even though the
/// internal `UnsafeCell` is !Sync in a safe way since we upheld the invariants
/// for these traits.
unsafe impl<T: Send> Send for Lazy<T> {}
unsafe impl<T: Send + Sync> Sync for Lazy<T> {}
