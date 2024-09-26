use crate::Referential;
use ring::digest::Digest;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::{env, fs};

#[derive(Hash)]
pub struct Handle<T: Referential + ?Sized> {
    phantom: PhantomData<T>,
    digest: [u8; 32],
}

impl<T: Referential> Copy for Handle<T> {}

impl<T: Referential> Clone for Handle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Referential + ?Sized> Debug for Handle<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}/{}", T::ref_name(), hex::encode(self.digest))
    }
}

impl<T: Referential + ?Sized> Display for Handle<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.digest))
    }
}

impl<T: Referential + ?Sized> Eq for Handle<T> {}

impl<T: Referential + ?Sized> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.digest.eq(&other.digest)
    }
}

impl<T: Referential + ?Sized> Ord for Handle<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.digest.cmp(&other.digest)
    }
}

impl<T: Referential + ?Sized> PartialOrd for Handle<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Referential + ?Sized> Handle<T> {
    pub fn new(data: [u8; 32]) -> Self {
        Self {
            phantom: PhantomData,
            digest: data,
        }
    }

    pub fn parse(hash: &str) -> Self {
        Self::new(
            hex::decode(hash)
                .expect(&format!("could not decode hash: {}", hash))
                .try_into()
                .expect(&format!("could not decode hash: {}", hash)),
        )
    }

    pub fn from_digest(digest: Digest) -> Self {
        Self::new(digest.as_ref().try_into().expect("digest could not fit"))
    }
}

impl<T: Referential> Handle<T> {
    pub fn resolve(&self) -> Result<T, std::io::Error> {
        Ok(T::deserialize(&fs::read(
            env::current_dir()
                .unwrap()
                .join(T::ref_name())
                .join(format!("{}.sch", hex::encode(&self.digest))),
        )?))
    }
}

impl<T: Referential> Serialize for Handle<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.digest)
    }
}

impl<'de, T: Referential> Deserialize<'de> for Handle<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(HandleVisitor::<T>(PhantomData))
    }
}

struct HandleVisitor<T: Referential>(PhantomData<T>);

impl<'de, T: Referential> Visitor<'de> for HandleVisitor<T> {
    type Value = Handle<T>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "a string or raw byes")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Handle::parse(v))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        v.try_into()
            .map(|v| Handle::new(v))
            .or_else(|_| std::str::from_utf8(v).map(|v| Handle::parse(v)))
            .map_err(|e| Error::custom(e))
    }
}
