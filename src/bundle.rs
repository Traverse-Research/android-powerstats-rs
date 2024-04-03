use std::{
    any,
    collections::HashMap,
    fmt,
    sync::{OnceLock, RwLock},
};

use binder::{
    binder_impl::{BorrowedParcel, Deserialize, Serialize},
    Parcelable, StatusCode,
};

pub(crate) mod mangled {
    #[allow(non_camel_case_types)]
    pub(crate) type _7_android_2_os_6_Bundle = super::Bundle;
}

/// https://cs.android.com/android/platform/superproject/main/+/main:frameworks/base/core/java/android/os/Parcelable.java;l=212-231;drc=39d2c720cd407481ebf9d348c7dcd5bc5df6cb8e
pub trait ParcelableCreator: Send + Sync {
    fn create_from_parcel(
        &self,
        parcel: &BorrowedParcel<'_>,
    ) -> Result<Box<dyn ParcelableInstance>, StatusCode>;
}

// TODO: Can this be cleaner?
static CREATORS: OnceLock<RwLock<HashMap<&'static str, &'static dyn ParcelableCreator>>> =
    OnceLock::new();

// https://cs.android.com/android/platform/superproject/main/+/main:frameworks/base/core/java/android/os/Parcelable.java;l=212;drc=82bdcd7ff7ba4962274f1d88caac0594ae964bef
pub fn register_creator(name: &'static str, creator: &'static dyn ParcelableCreator) {
    CREATORS
        .get_or_init(|| RwLock::new(HashMap::new()))
        .write()
        .unwrap()
        .insert(name, creator);
}

/// Wrapper-"hack" to make special "parsed [`Parcelable`]s" downcastable via the [`any::Any`] trait
pub trait ParcelableInstance: Parcelable + any::Any + fmt::Debug {
    fn as_any(&self) -> &dyn any::Any;
}

impl<T: Parcelable + any::Any + fmt::Debug> ParcelableInstance for T {
    fn as_any(&self) -> &dyn any::Any {
        self
    }
}

// #[derive(Debug, PartialEq, Hash)]
#[derive(Debug)]
pub enum Object {
    Null,
    ParcelableArray(Vec<Box<dyn ParcelableInstance>>),
    BooleanArray(Vec<bool>),
    LongArray(Vec<i64>),
}

/// https://cs.android.com/android/platform/superproject/main/+/main:frameworks/base/core/java/android/os/BaseBundle.java
// #[derive(Clone, Debug, PartialEq)]
#[derive(Debug)]
pub(crate) struct Bundle(pub HashMap<String, Object>);
impl Serialize for Bundle {
    fn serialize(&self, _parcel: &mut BorrowedParcel<'_>) -> Result<(), StatusCode> {
        todo!()
    }
}

// Keep in sync with frameworks/native/include/private/binder/ParcelValTypes.h.
const VAL_NULL: i32 = -1;
const VAL_STRING: i32 = 0;
const VAL_INTEGER: i32 = 1;
const VAL_MAP: i32 = 2; // length-prefixed
const VAL_BUNDLE: i32 = 3;
const VAL_PARCELABLE: i32 = 4; // length-prefixed
const VAL_SHORT: i32 = 5;
const VAL_LONG: i32 = 6;
const VAL_FLOAT: i32 = 7;
const VAL_DOUBLE: i32 = 8;
const VAL_BOOLEAN: i32 = 9;
const VAL_CHARSEQUENCE: i32 = 10;
const VAL_LIST: i32 = 11; // length-prefixed
const VAL_SPARSEARRAY: i32 = 12; // length-prefixed
const VAL_BYTEARRAY: i32 = 13;
const VAL_STRINGARRAY: i32 = 14;
const VAL_IBINDER: i32 = 15;
const VAL_PARCELABLEARRAY: i32 = 16; // length-prefixed
const VAL_OBJECTARRAY: i32 = 17; // length-prefixed
const VAL_INTARRAY: i32 = 18;
const VAL_LONGARRAY: i32 = 19;
const VAL_BYTE: i32 = 20;
const VAL_SERIALIZABLE: i32 = 21; // length-prefixed
const VAL_SPARSEBOOLEANARRAY: i32 = 22;
const VAL_BOOLEANARRAY: i32 = 23;
const VAL_CHARSEQUENCEARRAY: i32 = 24;
const VAL_PERSISTABLEBUNDLE: i32 = 25;
const VAL_SIZE: i32 = 26;
const VAL_SIZEF: i32 = 27;
const VAL_DOUBLEARRAY: i32 = 28;
const VAL_CHAR: i32 = 29;
const VAL_SHORTARRAY: i32 = 30;
const VAL_CHARARRAY: i32 = 31;
const VAL_FLOATARRAY: i32 = 32;

fn is_length_prefixed(r#type: i32) -> bool {
    // In general, we want custom types and containers of custom types to be length-prefixed,
    // this allows clients (eg. Bundle) to skip their content during deserialization. The
    // exception to this is Bundle, since Bundle is already length-prefixed and already copies
    // the correspondent section of the parcel internally.
    matches!(
        r#type,
        VAL_MAP
            | VAL_PARCELABLE
            | VAL_LIST
            | VAL_SPARSEARRAY
            | VAL_PARCELABLEARRAY
            | VAL_OBJECTARRAY
            | VAL_SERIALIZABLE
    )
}

fn parcel_read_value(parcel: &BorrowedParcel<'_>, r#type: i32) -> Result<Object, StatusCode> {
    match r#type {
        VAL_NULL => todo!("VAL_NULL"),
        VAL_STRING => todo!("VAL_STRING"),
        VAL_INTEGER => todo!("VAL_INTEGER"),
        VAL_MAP => todo!("VAL_MAP"),
        VAL_BUNDLE => todo!("VAL_BUNDLE"),
        VAL_PARCELABLE => todo!("VAL_PARCELABLE"),
        VAL_SHORT => todo!("VAL_SHORT"),
        VAL_LONG => todo!("VAL_LONG"),
        VAL_FLOAT => todo!("VAL_FLOAT"),
        VAL_DOUBLE => todo!("VAL_DOUBLE"),
        VAL_BOOLEAN => todo!("VAL_BOOLEAN"),
        VAL_CHARSEQUENCE => todo!("VAL_CHARSEQUENCE"),
        VAL_LIST => todo!("VAL_LIST"),
        VAL_SPARSEARRAY => todo!("VAL_SPARSEARRAY"),
        VAL_BYTEARRAY => todo!("VAL_BYTEARRAY"),
        VAL_STRINGARRAY => todo!("VAL_STRINGARRAY"),
        VAL_IBINDER => todo!("VAL_IBINDER"),
        VAL_PARCELABLEARRAY => {
            // readParcelableArrayInternal()
            let n: i32 = parcel.read()?;
            let mut vec = Vec::<Box<dyn ParcelableInstance>>::with_capacity(n as usize);
            for _ in 0..n {
                // readParcelableInternal
                let creator: String = parcel.read()?;
                let creators = CREATORS
                    .get()
                    .expect("No CREATORs were ever registered")
                    .read()
                    .unwrap();
                let creator = creators
                    .get(creator.as_str())
                    .ok_or(StatusCode::NAME_NOT_FOUND)
                    .inspect_err(|_e| eprintln!("No CREATOR registered for `{creator}`"))?;
                let object = creator.create_from_parcel(parcel)?;
                vec.push(object);
            }
            Ok(Object::ParcelableArray(vec))
        }
        VAL_OBJECTARRAY => todo!("VAL_OBJECTARRAY"),
        VAL_INTARRAY => todo!("VAL_INTARRAY"),
        VAL_LONGARRAY => {
            // createLongArray()
            let n: i32 = parcel.read()?;
            let mut vec = Vec::with_capacity(n as usize);
            for _ in 0..n {
                vec.push(parcel.read()?);
            }
            Ok(Object::LongArray(vec))
        }
        VAL_BYTE => todo!("VAL_BYTE"),
        VAL_SERIALIZABLE => todo!("VAL_SERIALIZABLE"),
        VAL_SPARSEBOOLEANARRAY => todo!("VAL_SPARSEBOOLEANARRAY"),
        VAL_BOOLEANARRAY => {
            // createBooleanArray()
            let n: i32 = parcel.read()?;
            // dbg!(n);
            let avail = parcel.get_data_size() - parcel.get_data_position();
            Ok(
                if n >= 0 && n <= (avail / std::mem::size_of::<i32>() as i32) {
                    let mut vec = Vec::with_capacity(n as usize);
                    for _ in 0..n {
                        let b: i32 = parcel.read()?;
                        vec.push(b != 0);
                    }
                    Object::BooleanArray(vec)
                } else {
                    Object::Null
                },
            )
        }
        VAL_CHARSEQUENCEARRAY => todo!("VAL_CHARSEQUENCEARRAY"),
        VAL_PERSISTABLEBUNDLE => todo!("VAL_PERSISTABLEBUNDLE"),
        VAL_SIZE => todo!("VAL_SIZE"),
        VAL_SIZEF => todo!("VAL_SIZEF"),
        VAL_DOUBLEARRAY => todo!("VAL_DOUBLEARRAY"),
        VAL_CHAR => todo!("VAL_CHAR"),
        VAL_SHORTARRAY => todo!("VAL_SHORTARRAY"),
        VAL_CHARARRAY => todo!("VAL_CHARARRAY"),
        VAL_FLOATARRAY => todo!("VAL_FLOATARRAY"),
        t => todo!("Unknown Parcel value type {t}"),
    }
}

/// https://cs.android.com/android/platform/superproject/main/+/main:frameworks/base/core/java/android/os/Parcel.java;l=4528;drc=190beaa49a35da1d9dcf66be9cfccfd23b0eb467
fn parcel_read_value_type(parcel: &BorrowedParcel<'_>) -> Result<Object, StatusCode> {
    let t: i32 = parcel.read()?;
    // dbg!(&t);
    if is_length_prefixed(t) {
        let length: i32 = parcel.read()?;
        // dbg!(length);
        let start = parcel.get_data_position();
        let r = parcel_read_value(parcel, t)?;
        assert_eq!(parcel.get_data_position(), start + length);
        Ok(r)
    } else {
        parcel_read_value(parcel, t)
    }
}

/// https://cs.android.com/android/platform/superproject/main/+/main:frameworks/native/libs/binder/Parcel.cpp;l=2261;drc=82bdcd7ff7ba4962274f1d88caac0594ae964bef
pub fn parcel_read_string8(parcel: &BorrowedParcel<'_>) -> Result<String, StatusCode> {
    // TODO: This is wrong, we must also parse a trailing \0 _before_ padding the parcel to 4 bytes again
    let len: u32 = parcel.read()?;
    let len_with_nul = len + 1;
    // TODO: Reading contiguous arrays of predetermined (rounded up for strings) size is hard with current API
    // Read the as UTF-8 characters with a terminating null, while keeping the parcel 4-byte aligned
    let words = (0..len_with_nul.div_ceil(4) as usize)
        .map(|_| parcel.read())
        .collect::<Result<Vec<u32>, StatusCode>>()?;
    let chars = bytemuck::cast_slice(&words);
    assert_eq!(chars[len as usize], b'\0');
    // TODO: Need to move
    let str = std::str::from_utf8(&chars[..len as usize]).unwrap();
    Ok(str.to_owned())
}

impl Deserialize for Bundle {
    fn deserialize(parcel: &BorrowedParcel<'_>) -> Result<Self, StatusCode> {
        // dbg!(parcel.get_data_size());

        // Parse nullability because of writeTypedObject
        // https://cs.android.com/android/platform/superproject/main/+/main:out/soong/.intermediates/frameworks/base/framework-minus-apex-intdefs/android_common/e18b8e8d84cb9f664aa09a397b08c165/xref50/srcjars.xref/com/android/internal/os/IResultReceiver.java;l=118;drc=190beaa49a35da1d9dcf66be9cfccfd23b0eb467
        let is_set: i32 = parcel.read()?;
        assert!(is_set == 1);

        let length: i32 = parcel.read()?;
        // dbg!(length);
        assert!(length >= 0, "Bad length {length}");
        if length == 0 {
            return Ok(Self(HashMap::new())); // Empty
        }

        // https://cs.android.com/android/platform/superproject/main/+/main:frameworks/base/core/java/android/os/BaseBundle.java;l=1877-1879;drc=190beaa49a35da1d9dcf66be9cfccfd23b0eb467
        // const BUNDLE_MAGIC: i32 = 0x4C444E42; // 'B' 'N' 'D' 'L'
        // const BUNDLE_MAGIC_NATIVE: i32 = 0x4C444E44; // 'B' 'N' 'D' 'N'
        let _magic: i32 = parcel.read()?;
        // let is_java_bundle = magic == BUNDLE_MAGIC;
        // let is_native_bundle = magic == BUNDLE_MAGIC_NATIVE;
        // dbg!(is_java_bundle, is_native_bundle);

        // https://cs.android.com/android/platform/superproject/main/+/main:frameworks/base/core/java/android/os/BaseBundle.java;l=459;drc=190beaa49a35da1d9dcf66be9cfccfd23b0eb467
        let count: i32 = parcel.read()?;
        // dbg!(count);
        let mut map = HashMap::new();
        for _ in 0..count {
            let str: String = parcel.read()?;
            // dbg!(&str);

            // TODO: optimization for sorted parcels!
            map.insert(str, parcel_read_value_type(parcel)?);
        }

        Ok(Self(map))
    }
}
