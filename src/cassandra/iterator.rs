use crate::cassandra::error::*;
use crate::cassandra::field::Field;
use crate::cassandra::schema::aggregate_meta::AggregateMeta;
use crate::cassandra::schema::column_meta::ColumnMeta;
use crate::cassandra::schema::function_meta::FunctionMeta;
use crate::cassandra::schema::keyspace_meta::KeyspaceMeta;
use crate::cassandra::schema::table_meta::TableMeta;
use crate::cassandra::util::{Protected, ProtectedInner};
use crate::cassandra::value::Value;

// use cassandra_sys::CassIteratorType as _CassIteratorType;
use crate::cassandra_sys::cass_false;
use crate::cassandra_sys::CassIterator as _CassIterator;
// use cassandra_sys::cass_iterator_type;
use crate::cassandra_sys::cass_iterator_free;
use crate::cassandra_sys::cass_iterator_get_aggregate_meta;
use crate::cassandra_sys::cass_iterator_get_column_meta;
use crate::cassandra_sys::cass_iterator_get_function_meta;
use crate::cassandra_sys::cass_iterator_get_keyspace_meta;
use crate::cassandra_sys::cass_iterator_get_map_key;
use crate::cassandra_sys::cass_iterator_get_map_value;
use crate::cassandra_sys::cass_iterator_get_meta_field_name;
use crate::cassandra_sys::cass_iterator_get_meta_field_value;
use crate::cassandra_sys::cass_iterator_get_table_meta;
use crate::cassandra_sys::cass_iterator_get_user_type_field_name;
use crate::cassandra_sys::cass_iterator_get_user_type_field_value;
use crate::cassandra_sys::cass_iterator_get_value;
use crate::cassandra_sys::cass_iterator_next;
use crate::cassandra_sys::cass_true;
use crate::cassandra_sys::CassKeyspaceMeta;
use crate::cassandra_sys::CassSchemaMeta;
use crate::cassandra_sys::CassTableMeta;
use crate::cassandra_sys::CassValue as _CassValue;

use std::marker::PhantomData;
use std::{slice, str};

/// Iterator that only allows access to a single item at a time. You must stop
/// using the returned item before getting the next.
///
/// Ultimately we will move to use a common crate for this, but to date
/// there is no good crate to follow.
/// https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html#generic-associated-types-gats
/// and https://github.com/Crazytieguy/gat-lending-iterator were references
/// for this code.
///
/// The idiomatic way to work with this trait is as follows:
///
/// ```
/// # use cassandra_cpp::*;
/// # struct MyLI;
/// # impl LendingIterator for MyLI {
/// #   type Item<'a> = ();
/// #   fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> { None }
/// #   fn size_hint(&self) -> (usize, Option<usize>) { (0, Some(0)) }
/// # }
/// # let mut lending_iterator = MyLI;
/// while let Some(row) = lending_iterator.next() {
///   // ... do something with `row` ...
/// }
/// ```
pub trait LendingIterator {
    /// The type of each item.
    type Item<'a>
    where
        Self: 'a;

    /// Retrieve the next item from the iterator; it lives only as long as the
    /// mutable reference to the iterator.
    fn next(&mut self) -> Option<Self::Item<'_>>;

    /// Minimum and optional maximum expected length of the iterator.
    /// Default implementation returns `(0, None)`.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

/// Iterator over the aggregates in the keyspace.
///
/// A Cassandra iterator is a `LendingIterator` because it borrows from some
/// underlying value, but owns a single item. Each time `next()` is invoked it
/// decodes the current item into that item, thus invalidating its previous
/// value.
#[derive(Debug)]
pub struct AggregateIterator<'a>(*mut _CassIterator, PhantomData<&'a CassKeyspaceMeta>);

// The underlying C type has no thread-local state, and forbids only concurrent
// mutation/free: https://datastax.github.io/cpp-driver/topics/#thread-safety
unsafe impl Send for AggregateIterator<'_> {}
unsafe impl Sync for AggregateIterator<'_> {}

impl Drop for AggregateIterator<'_> {
    fn drop(&mut self) {
        unsafe { cass_iterator_free(self.0) }
    }
}

impl LendingIterator for AggregateIterator<'_> {
    type Item<'a> = AggregateMeta<'a> where Self: 'a;
    fn next(&mut self) -> Option<<Self as LendingIterator>::Item<'_>> {
        unsafe {
            match cass_iterator_next(self.0) {
                cass_false => None,
                cass_true => {
                    let field_value = cass_iterator_get_aggregate_meta(self.0);
                    Some(AggregateMeta::build(field_value))
                }
            }
        }
    }
}

/// Iterator over the fields of a UDT
///
/// A Cassandra iterator is a `LendingIterator` because it borrows from some
/// underlying value, but owns a single item. Each time `next()` is invoked it
/// decodes the current item into that item, thus invalidating its previous
/// value.
#[derive(Debug)]
pub struct UserTypeIterator<'a>(*mut _CassIterator, PhantomData<&'a _CassValue>);

// The underlying C type has no thread-local state, and forbids only concurrent
// mutation/free: https://datastax.github.io/cpp-driver/topics/#thread-safety
unsafe impl Send for UserTypeIterator<'_> {}
unsafe impl Sync for UserTypeIterator<'_> {}

impl Drop for UserTypeIterator<'_> {
    fn drop(&mut self) {
        unsafe { cass_iterator_free(self.0) }
    }
}

impl LendingIterator for UserTypeIterator<'_> {
    type Item<'a> = (String, Value<'a>) where Self: 'a;
    fn next(&mut self) -> Option<<Self as LendingIterator>::Item<'_>> {
        unsafe {
            match cass_iterator_next(self.0) {
                cass_false => None,
                cass_true => Some((self.get_field_name(), self.get_field_value())),
            }
        }
    }
}

impl UserTypeIterator<'_> {
    fn get_field_name(&self) -> String {
        unsafe {
            let mut name = std::ptr::null();
            let mut name_length = 0;
            cass_iterator_get_user_type_field_name(self.0, &mut name, &mut name_length)
                .to_result(())
                .and_then(|_| {
                    let slice = slice::from_raw_parts(name as *const u8, name_length);
                    let name = str::from_utf8(slice)?.to_owned();
                    Ok(name)
                })
                .expect("Cassandra error during iteration")
        }
    }

    fn get_field_value(&self) -> Value {
        unsafe { Value::build(cass_iterator_get_user_type_field_value(self.0)) }
    }
}

/// Iterator over the functions in a keyspace.
///
/// A Cassandra iterator is a `LendingIterator` because it borrows from some
/// underlying value, but owns a single item. Each time `next()` is invoked it
/// decodes the current item into that item, thus invalidating its previous
/// value.
#[derive(Debug)]
pub struct FunctionIterator<'a>(*mut _CassIterator, PhantomData<&'a CassKeyspaceMeta>);

// The underlying C type has no thread-local state, and forbids only concurrent
// mutation/free: https://datastax.github.io/cpp-driver/topics/#thread-safety
unsafe impl Send for FunctionIterator<'_> {}
unsafe impl Sync for FunctionIterator<'_> {}

impl LendingIterator for FunctionIterator<'_> {
    type Item<'a> = FunctionMeta<'a> where Self: 'a;
    fn next(&mut self) -> Option<<Self as LendingIterator>::Item<'_>> {
        unsafe {
            match cass_iterator_next(self.0) {
                cass_false => None,
                cass_true => Some(FunctionMeta::build(cass_iterator_get_function_meta(self.0))),
            }
        }
    }
}

/// Iterator over the tables in a keyspace.
///
/// A Cassandra iterator is a `LendingIterator` because it borrows from some
/// underlying value, but owns a single item. Each time `next()` is invoked it
/// decodes the current item into that item, thus invalidating its previous
/// value.
#[derive(Debug)]
pub struct TableIterator<'a>(*mut _CassIterator, PhantomData<&'a CassKeyspaceMeta>);

// The underlying C type has no thread-local state, and forbids only concurrent
// mutation/free: https://datastax.github.io/cpp-driver/topics/#thread-safety
unsafe impl Send for TableIterator<'_> {}
unsafe impl Sync for TableIterator<'_> {}

impl LendingIterator for TableIterator<'_> {
    type Item<'a> = TableMeta<'a> where Self: 'a;
    fn next(&mut self) -> Option<<Self as LendingIterator>::Item<'_>> {
        unsafe {
            match cass_iterator_next(self.0) {
                cass_false => None,
                cass_true => Some(TableMeta::build(cass_iterator_get_table_meta(self.0))),
            }
        }
    }
}

/// Iterator over the keyspaces in the schema.
///
/// A Cassandra iterator is a `LendingIterator` because it borrows from some
/// underlying value, but owns a single item. Each time `next()` is invoked it
/// decodes the current item into that item, thus invalidating its previous
/// value.
#[derive(Debug)]
pub struct KeyspaceIterator<'a>(*mut _CassIterator, PhantomData<&'a CassSchemaMeta>);

// The underlying C type has no thread-local state, and forbids only concurrent
// mutation/free: https://datastax.github.io/cpp-driver/topics/#thread-safety
unsafe impl Send for KeyspaceIterator<'_> {}
unsafe impl Sync for KeyspaceIterator<'_> {}

impl LendingIterator for KeyspaceIterator<'_> {
    type Item<'a> = KeyspaceMeta<'a> where Self: 'a;
    fn next(&mut self) -> Option<<Self as LendingIterator>::Item<'_>> {
        unsafe {
            match cass_iterator_next(self.0) {
                cass_false => None,
                cass_true => Some(KeyspaceMeta::build(cass_iterator_get_keyspace_meta(self.0))),
            }
        }
    }
}

/// Iterator over the columns in a table.
///
/// A Cassandra iterator is a `LendingIterator` because it borrows from some
/// underlying value, but owns a single item. Each time `next()` is invoked it
/// decodes the current item into that item, thus invalidating its previous
/// value.
#[derive(Debug)]
pub struct ColumnIterator<'a>(*mut _CassIterator, PhantomData<&'a CassTableMeta>);

// The underlying C type has no thread-local state, and forbids only concurrent
// mutation/free: https://datastax.github.io/cpp-driver/topics/#thread-safety
unsafe impl Send for ColumnIterator<'_> {}
unsafe impl Sync for ColumnIterator<'_> {}

impl LendingIterator for ColumnIterator<'_> {
    type Item<'a> = ColumnMeta<'a> where Self: 'a;
    fn next(&mut self) -> Option<<Self as LendingIterator>::Item<'_>> {
        unsafe {
            match cass_iterator_next(self.0) {
                cass_false => None,
                cass_true => Some(ColumnMeta::build(cass_iterator_get_column_meta(self.0))),
            }
        }
    }
}

/// Iterator over the fields in a metadata object.
///
/// A Cassandra iterator is a `LendingIterator` because it borrows from some
/// underlying value, but owns a single item. Each time `next()` is invoked it
/// decodes the current item into that item, thus invalidating its previous
/// value.
//
// Could be one of several underlying types; CassTableMeta is just a
// representative. Since it's a phantom, it doesn't matter which type we name.
#[derive(Debug)]
pub struct FieldIterator<'a>(*mut _CassIterator, PhantomData<&'a CassTableMeta>);

// The underlying C type has no thread-local state, and forbids only concurrent
// mutation/free: https://datastax.github.io/cpp-driver/topics/#thread-safety
unsafe impl Send for FieldIterator<'_> {}
unsafe impl Sync for FieldIterator<'_> {}

impl LendingIterator for FieldIterator<'_> {
    type Item<'a> = Field<'a> where Self: 'a;

    fn next(&mut self) -> Option<<Self as LendingIterator>::Item<'_>> {
        unsafe {
            match cass_iterator_next(self.0) {
                cass_false => None,
                cass_true => {
                    let mut name = std::ptr::null();
                    let mut name_length = 0;
                    cass_iterator_get_meta_field_name(self.0, &mut name, &mut name_length)
                        .to_result(())
                        .and_then(|_| {
                            let slice = slice::from_raw_parts(name as *const u8, name_length);
                            let name = str::from_utf8(slice)?.to_owned();
                            let value = Value::build(cass_iterator_get_meta_field_value(self.0));
                            Ok(Some(Field { name, value }))
                        })
                }
                .expect("Cassandra error during iteration"),
            }
        }
    }
}

impl ProtectedInner<*mut _CassIterator> for UserTypeIterator<'_> {
    fn inner(&self) -> *mut _CassIterator {
        self.0
    }
}

impl Protected<*mut _CassIterator> for UserTypeIterator<'_> {
    fn build(inner: *mut _CassIterator) -> Self {
        if inner.is_null() {
            panic!("Unexpected null pointer")
        };
        UserTypeIterator(inner, PhantomData)
    }
}

impl ProtectedInner<*mut _CassIterator> for AggregateIterator<'_> {
    fn inner(&self) -> *mut _CassIterator {
        self.0
    }
}

impl Protected<*mut _CassIterator> for AggregateIterator<'_> {
    fn build(inner: *mut _CassIterator) -> Self {
        if inner.is_null() {
            panic!("Unexpected null pointer")
        };
        AggregateIterator(inner, PhantomData)
    }
}

impl ProtectedInner<*mut _CassIterator> for FunctionIterator<'_> {
    fn inner(&self) -> *mut _CassIterator {
        self.0
    }
}

impl Protected<*mut _CassIterator> for FunctionIterator<'_> {
    fn build(inner: *mut _CassIterator) -> Self {
        if inner.is_null() {
            panic!("Unexpected null pointer")
        };
        FunctionIterator(inner, PhantomData)
    }
}

impl ProtectedInner<*mut _CassIterator> for KeyspaceIterator<'_> {
    fn inner(&self) -> *mut _CassIterator {
        self.0
    }
}

impl Protected<*mut _CassIterator> for KeyspaceIterator<'_> {
    fn build(inner: *mut _CassIterator) -> Self {
        if inner.is_null() {
            panic!("Unexpected null pointer")
        };
        KeyspaceIterator(inner, PhantomData)
    }
}

impl ProtectedInner<*mut _CassIterator> for FieldIterator<'_> {
    fn inner(&self) -> *mut _CassIterator {
        self.0
    }
}
impl Protected<*mut _CassIterator> for FieldIterator<'_> {
    fn build(inner: *mut _CassIterator) -> Self {
        if inner.is_null() {
            panic!("Unexpected null pointer")
        };
        FieldIterator(inner, PhantomData)
    }
}

impl ProtectedInner<*mut _CassIterator> for ColumnIterator<'_> {
    fn inner(&self) -> *mut _CassIterator {
        self.0
    }
}

impl Protected<*mut _CassIterator> for ColumnIterator<'_> {
    fn build(inner: *mut _CassIterator) -> Self {
        if inner.is_null() {
            panic!("Unexpected null pointer")
        };
        ColumnIterator(inner, PhantomData)
    }
}

impl ProtectedInner<*mut _CassIterator> for TableIterator<'_> {
    fn inner(&self) -> *mut _CassIterator {
        self.0
    }
}

impl Protected<*mut _CassIterator> for TableIterator<'_> {
    fn build(inner: *mut _CassIterator) -> Self {
        if inner.is_null() {
            panic!("Unexpected null pointer")
        };
        TableIterator(inner, PhantomData)
    }
}

impl ProtectedInner<*mut _CassIterator> for MapIterator<'_> {
    fn inner(&self) -> *mut _CassIterator {
        self.0
    }
}

impl Protected<*mut _CassIterator> for MapIterator<'_> {
    fn build(inner: *mut _CassIterator) -> Self {
        if inner.is_null() {
            panic!("Unexpected null pointer")
        };
        MapIterator(inner, PhantomData)
    }
}

impl ProtectedInner<*mut _CassIterator> for SetIterator<'_> {
    fn inner(&self) -> *mut _CassIterator {
        self.0
    }
}

impl Protected<*mut _CassIterator> for SetIterator<'_> {
    fn build(inner: *mut _CassIterator) -> Self {
        if inner.is_null() {
            panic!("Unexpected null pointer")
        };
        SetIterator(inner, PhantomData)
    }
}

/// Iterator over the values in a set.
///
/// A Cassandra iterator is a `LendingIterator` because it borrows from some
/// underlying value, but owns a single item. Each time `next()` is invoked it
/// decodes the current item into that item, thus invalidating its previous
/// value.
#[derive(Debug)]
pub struct SetIterator<'a>(*mut _CassIterator, PhantomData<&'a _CassValue>);

// The underlying C type has no thread-local state, and forbids only concurrent
// mutation/free: https://datastax.github.io/cpp-driver/topics/#thread-safety
unsafe impl Send for SetIterator<'_> {}
unsafe impl Sync for SetIterator<'_> {}

impl Drop for SetIterator<'_> {
    fn drop(&mut self) {
        unsafe { cass_iterator_free(self.0) }
    }
}

impl LendingIterator for SetIterator<'_> {
    type Item<'a> = Value<'a> where Self: 'a;

    fn next(&mut self) -> Option<<Self as LendingIterator>::Item<'_>> {
        unsafe {
            match cass_iterator_next(self.0) {
                cass_false => None,
                cass_true => Some(self.get_value()),
            }
        }
    }
}

impl SetIterator<'_> {
    fn get_value(&self) -> Value {
        unsafe { Value::build(cass_iterator_get_value(self.0)) }
    }
}

/// An iterator over the k/v pairs in a map.
#[derive(Debug)]
///
/// A Cassandra iterator is a `LendingIterator` because it borrows from some
/// underlying value, but owns a single item. Each time `next()` is invoked it
/// decodes the current item into that item, thus invalidating its previous
/// value.
pub struct MapIterator<'a>(*mut _CassIterator, PhantomData<&'a _CassValue>);

// The underlying C type has no thread-local state, and forbids only concurrent
// mutation/free: https://datastax.github.io/cpp-driver/topics/#thread-safety
unsafe impl Send for MapIterator<'_> {}
unsafe impl Sync for MapIterator<'_> {}

impl MapIterator<'_> {
    fn get_key(&self) -> Value {
        unsafe { Value::build(cass_iterator_get_map_key(self.0)) }
    }
    fn get_value(&self) -> Value {
        unsafe { Value::build(cass_iterator_get_map_value(self.0)) }
    }

    /// Gets the next k/v pair in the map
    pub fn get_pair(&self) -> (Value, Value) {
        (self.get_key(), self.get_value())
    }
}

impl Drop for MapIterator<'_> {
    fn drop(&mut self) {
        unsafe { cass_iterator_free(self.0) }
    }
}

impl LendingIterator for MapIterator<'_> {
    type Item<'a> = (Value<'a>, Value<'a>) where Self: 'a;
    fn next(&mut self) -> Option<<Self as LendingIterator>::Item<'_>> {
        unsafe {
            match cass_iterator_next(self.0) {
                cass_false => None,
                cass_true => Some(self.get_pair()),
            }
        }
    }
}
