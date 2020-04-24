use differential_datalog::record;
use differential_datalog::record::*;
use serde;
use std::cmp;
use std::fmt;

#[cfg(feature = "flatbuf")]
use flatbuf::{FromFlatBuffer, ToFlatBuffer, ToFlatBufferTable, ToFlatBufferVectorElement};

/* `flatc`-generated declarations re-exported by `flatbuf.rs` */
#[cfg(feature = "flatbuf")]
use flatbuf::fb;

/* FlatBuffers runtime */
#[cfg(feature = "flatbuf")]
use flatbuffers as fbrt;

#[derive(Default, Eq, PartialOrd, PartialEq, Ord, Clone, Hash)]
pub struct internment_Intern<A>
where
    A: Eq + Send + Sync + Hash + 'static,
{
    intern: arcintern::ArcIntern<A>,
}

impl<A: Eq + Hash + Send + Sync + 'static> internment_Intern<A> {
    pub fn new(x: A) -> internment_Intern<A> {
        internment_Intern {
            intern: arcintern::ArcIntern::new(x),
        }
    }
    pub fn as_ref(&self) -> &A {
        self.intern.as_ref()
    }
}

pub fn internment_intern<A: Eq + Hash + Send + Sync + Clone + 'static>(
    x: &A,
) -> internment_Intern<A> {
    internment_Intern::new(x.clone())
}

pub fn internment_ival<A: Eq + Hash + Send + Sync + Clone>(x: &internment_Intern<A>) -> &A {
    x.intern.as_ref()
}

/*pub fn intern_istring_ord(s: &intern_istring) -> u32 {
    s.x
}*/

impl<A: fmt::Display + Eq + Hash + Send + Sync + Clone> fmt::Display for internment_Intern<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_ref(), f)
        //record::format_ddlog_str(&intern_istring_str(self), f)
    }
}

impl<A: fmt::Debug + Eq + Hash + Send + Sync + Clone> fmt::Debug for internment_Intern<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_ref(), f)
        //record::format_ddlog_str(&intern_istring_str(self), f)
    }
}

impl<A: Serialize + Eq + Hash + Send + Sync + Clone> serde::Serialize for internment_Intern<A> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}

impl<'de, A: Deserialize<'de> + Eq + Hash + Send + Sync + 'static> serde::Deserialize<'de>
    for internment_Intern<A>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        A::deserialize(deserializer).map(|s| internment_Intern::new(s))
    }
}

impl<A: FromRecord + Eq + Hash + Send + Sync + 'static> FromRecord for internment_Intern<A> {
    fn from_record(val: &Record) -> Result<Self, String> {
        A::from_record(val).map(|x| internment_Intern::new(x))
    }
}

impl<A: IntoRecord + Eq + Hash + Send + Sync + Clone> IntoRecord for internment_Intern<A> {
    fn into_record(self) -> Record {
        internment_ival(&self).clone().into_record()
    }
}

impl<A> Mutator<internment_Intern<A>> for Record
where
    A: Clone + Eq + Send + Sync + Hash,
    Record: Mutator<A>,
{
    fn mutate(&self, x: &mut internment_Intern<A>) -> Result<(), String> {
        let mut v = internment_ival(x).clone();
        self.mutate(&mut v)?;
        *x = internment_intern(&v);
        Ok(())
    }
}

#[cfg(feature = "flatbuf")]
impl<A, FB> FromFlatBuffer<FB> for internment_Intern<A>
where
    A: Eq + Hash + Send + Sync + 'static,
    A: FromFlatBuffer<FB>,
{
    fn from_flatbuf(fb: FB) -> Response<Self> {
        Ok(internment_Intern::new(A::from_flatbuf(fb)?))
    }
}

#[cfg(feature = "flatbuf")]
impl<'b, A, T> ToFlatBuffer<'b> for internment_Intern<A>
where
    T: 'b,
    A: Eq + Send + Sync + Hash + ToFlatBuffer<'b, Target = T>,
{
    type Target = T;

    fn to_flatbuf(&self, fbb: &mut fbrt::FlatBufferBuilder<'b>) -> Self::Target {
        self.as_ref().to_flatbuf(fbb)
    }
}

/*#[cfg(feature = "flatbuf")]
impl<'a> FromFlatBuffer<fb::__String<'a>> for intern_istring {
    fn from_flatbuf(v: fb::__String<'a>) -> Response<Self> {
        Ok(intern_string_intern(&String::from_flatbuf(v)?))
    }
}*/

#[cfg(feature = "flatbuf")]
impl<'b, A, T> ToFlatBufferTable<'b> for internment_Intern<A>
where
    T: 'b,
    A: Eq + Send + Sync + Hash + ToFlatBufferTable<'b, Target = T>,
{
    type Target = T;
    fn to_flatbuf_table(
        &self,
        fbb: &mut fbrt::FlatBufferBuilder<'b>,
    ) -> fbrt::WIPOffset<Self::Target> {
        self.as_ref().to_flatbuf_table(fbb)
    }
}

#[cfg(feature = "flatbuf")]
impl<'b, A, T> ToFlatBufferVectorElement<'b> for internment_Intern<A>
where
    T: 'b + fbrt::Push + Copy,
    A: Eq + Send + Sync + Hash + ToFlatBufferVectorElement<'b, Target = T>,
{
    type Target = T;

    fn to_flatbuf_vector_element(&self, fbb: &mut fbrt::FlatBufferBuilder<'b>) -> Self::Target {
        self.as_ref().to_flatbuf_vector_element(fbb)
    }
}

pub fn internment_istring_join(strings: &std_Vec<internment_istring>, sep: &String) -> String {
    strings
        .x
        .iter()
        .map(|s| s.as_ref())
        .cloned()
        .collect::<Vec<String>>()
        .join(sep.as_str())
}

pub fn internment_istring_split(s: &internment_istring, sep: &String) -> std_Vec<String> {
    std_Vec {
        x: s.as_ref().split(sep).map(|x| x.to_owned()).collect(),
    }
}

pub fn internment_istring_contains(s1: &internment_istring, s2: &String) -> bool {
    s1.as_ref().contains(s2.as_str())
}

pub fn internment_istring_substr(
    s: &internment_istring,
    start: &std_usize,
    end: &std_usize,
) -> String {
    let len = s.as_ref().len();
    let from = cmp::min(*start as usize, len);
    let to = cmp::max(from, cmp::min(*end as usize, len));
    s.as_ref()[from..to].to_string()
}

pub fn internment_istring_replace(s: &internment_istring, from: &String, to: &String) -> String {
    s.as_ref().replace(from, to)
}

pub fn internment_istring_starts_with(s: &internment_istring, prefix: &String) -> bool {
    s.as_ref().starts_with(prefix)
}

pub fn internment_istring_ends_with(s: &internment_istring, suffix: &String) -> bool {
    s.as_ref().ends_with(suffix)
}

pub fn internment_istring_trim(s: &internment_istring) -> String {
    s.as_ref().trim().to_string()
}

pub fn internment_istring_len(s: &internment_istring) -> std_usize {
    s.as_ref().len() as std_usize
}

pub fn internment_istring_to_bytes(s: &internment_istring) -> std_Vec<u8> {
    std_Vec::from(s.as_ref().as_bytes())
}

pub fn internment_istring_to_lowercase(s: &internment_istring) -> String {
    s.as_ref().to_lowercase()
}

pub fn internment_istring_to_uppercase(s: &internment_istring) -> String {
    s.as_ref().to_uppercase()
}

pub fn internment_istring_reverse(s: &internment_istring) -> String {
    s.as_ref().chars().rev().collect()
}

mod arcintern {
    use lazy_static::lazy_static;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::borrow::Borrow;
    use std::cmp::Ordering;
    use std::collections::HashSet;
    use std::fmt::Display;
    use std::hash::{Hash, Hasher};
    use std::ops::Deref;
    use std::sync::Arc;
    use std::sync::Mutex;

    /// A pointer to a reference-counted interned object.
    ///
    /// The interned object will be held in memory only until its
    /// reference count reaches zero.
    ///
    /// # Example
    /// ```rust
    /// use internment::ArcIntern;
    ///
    /// let x = ArcIntern::new("hello");
    /// let y = ArcIntern::new("world");
    /// assert_ne!(x, y);
    /// assert_eq!(x, ArcIntern::new("hello"));
    /// assert_eq!(*x, "hello"); // dereference an ArcIntern like a pointer
    /// ```
    pub struct ArcIntern<T: Eq + Hash + Send + Sync + 'static> {
        pointer: Arc<T>,
    }

    unsafe impl<T: Eq + Hash + Send + Sync> Send for ArcIntern<T> {}
    unsafe impl<T: Eq + Hash + Send + Sync> Sync for ArcIntern<T> {}

    type Container<T> = Mutex<HashSet<Arc<T>>>;
    lazy_static! {
        static ref CONTAINER: state::Container = state::Container::new();
    }

    impl<T: Eq + Hash + Send + Sync + 'static> ArcIntern<T> {
        fn get_mutex() -> &'static Container<T> {
            match CONTAINER.try_get::<Container<T>>() {
                Some(m) => m,
                None => {
                    CONTAINER.set::<Container<T>>(Mutex::new(HashSet::new()));
                    CONTAINER.get::<Container<T>>()
                }
            }
        }
        /// Intern a value.  If this value has not previously been
        /// interned, then `new` will allocate a spot for the value on the
        /// heap.  Otherwise, it will return a pointer to the object
        /// previously allocated.
        ///
        /// Note that `ArcIntern::new` is a bit slow, since it needs to check
        /// a `HashMap` protected by a `Mutex`.
        pub fn new(val: T) -> ArcIntern<T> {
            let mymutex = Self::get_mutex();
            let mut m = mymutex.lock().unwrap();
            if let Some(b) = m.get(&val) {
                // First increment the count.  We can use relaxed ordering
                // here because we are holding the mutex, which has its
                // own barriers.
                //b.count.fetch_add(1, Ordering::Relaxed);
                // then return the value
                return ArcIntern { pointer: b.clone() };
            }
            let b = Arc::new(val);
            let p = ArcIntern { pointer: b.clone() };
            m.insert(b);
            p
        }
        /// See how many objects have been interned.  This may be helpful
        /// in analyzing memory use.
        pub fn num_objects_interned(&self) -> usize {
            if let Some(m) = CONTAINER.try_get::<Container<T>>() {
                return m.lock().unwrap().len();
            }
            0
        }
        /// Return the number of counts for this pointer.
        pub fn refcount(&self) -> usize {
            Arc::strong_count(&self.pointer)
        }
    }

    impl<T: Eq + Hash + Send + Sync + 'static> Clone for ArcIntern<T> {
        fn clone(&self) -> Self {
            // First increment the count.  Using a relaxed ordering is
            // alright here, as knowledge of the original reference
            // prevents other threads from erroneously deleting the
            // object.  (See `std::sync::Arc` documentation for more
            // explanation.)
            //unsafe { (*self.pointer).count.fetch_add(1, Ordering::Relaxed) };
            ArcIntern {
                pointer: self.pointer.clone(),
            }
        }
    }

    impl<T: Eq + Hash + Send + Sync> Drop for ArcIntern<T> {
        fn drop(&mut self) {
            // (Quoting from std::sync::Arc again): Because `fetch_sub` is
            // already atomic, we do not need to synchronize with other
            // threads unless we are going to delete the object. This same
            // logic applies to the below `fetch_sub` to the `weak` count.
            //let count_was = unsafe { (*self.pointer).count.fetch_sub(1, Ordering::Release) };
            //if Arc::strong_count(&self.pointer) == 2 {
            // (Quoting from std::sync::Arc again): This fence is
            // needed to prevent reordering of use of the data and
            // deletion of the data.  Because it is marked `Release`,
            // the decreasing of the reference count synchronizes with
            // this `Acquire` fence. This means that use of the data
            // happens before decreasing the reference count, which
            // happens before this fence, which happens before the
            // deletion of the data.
            //std::sync::atomic::fence(Ordering::Acquire);

            // removed is declared before m, so the mutex guard will be
            // dropped *before* the removed content is dropped, since it
            // might need to lock the mutex.
            let mut m = Self::get_mutex().lock().unwrap();
            if (Arc::strong_count(&self.pointer) == 2) {
                m.remove(&self.pointer);
            }
            //}
        }
    }

    impl<T: Send + Sync + Hash + Eq> AsRef<T> for ArcIntern<T> {
        fn as_ref(&self) -> &T {
            unsafe { self.pointer.as_ref() }
        }
    }
    impl<T: Eq + Hash + Send + Sync> Borrow<T> for ArcIntern<T> {
        fn borrow(&self) -> &T {
            self.as_ref()
        }
    }
    impl<T: Eq + Hash + Send + Sync> Deref for ArcIntern<T> {
        type Target = T;
        fn deref(&self) -> &T {
            self.as_ref()
        }
    }

    impl<T: Eq + Hash + Send + Sync + Display> Display for ArcIntern<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            self.deref().fmt(f)
        }
    }

    impl<T: Eq + Hash + Send + Sync + 'static> From<T> for ArcIntern<T> {
        fn from(t: T) -> Self {
            ArcIntern::new(t)
        }
    }
    impl<T: Eq + Hash + Send + Sync + Default + 'static> Default for ArcIntern<T> {
        fn default() -> ArcIntern<T> {
            ArcIntern::new(Default::default())
        }
    }

    /// The hash implementation returns the hash of the pointer
    /// value, not the hash of the value pointed to.  This should
    /// be irrelevant, since there is a unique pointer for every
    /// value, but it *is* observable, since you could compare the
    /// hash of the pointer with hash of the data itself.
    impl<T: Eq + Hash + Send + Sync> Hash for ArcIntern<T> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            Arc::into_raw(self.pointer.clone()).hash(state);
        }
    }

    impl<T: Eq + Hash + Send + Sync> PartialEq for ArcIntern<T> {
        fn eq(&self, other: &ArcIntern<T>) -> bool {
            Arc::ptr_eq(&self.pointer, &other.pointer)
        }
    }
    impl<T: Eq + Hash + Send + Sync> Eq for ArcIntern<T> {}

    impl<T: Eq + Hash + Send + Sync + PartialOrd> PartialOrd for ArcIntern<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.as_ref().partial_cmp(other)
        }
        fn lt(&self, other: &Self) -> bool {
            self.as_ref().lt(other)
        }
        fn le(&self, other: &Self) -> bool {
            self.as_ref().le(other)
        }
        fn gt(&self, other: &Self) -> bool {
            self.as_ref().gt(other)
        }
        fn ge(&self, other: &Self) -> bool {
            self.as_ref().ge(other)
        }
    }
    impl<T: Eq + Hash + Send + Sync + Ord> Ord for ArcIntern<T> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.as_ref().cmp(other)
        }
    }

    impl<T: Eq + Hash + Send + Sync + Serialize> Serialize for ArcIntern<T> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.as_ref().serialize(serializer)
        }
    }

    impl<'de, T: Eq + Hash + Send + Sync + 'static + Deserialize<'de>> Deserialize<'de>
        for ArcIntern<T>
    {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            T::deserialize(deserializer).map(|x: T| Self::new(x))
        }
    }
}
