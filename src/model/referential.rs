use crate::Handle;
use ring::digest;
use ring::digest::digest;
use std::io::Write;
use std::{env, fs};

pub trait Referential {
    fn ref_name() -> &'static str;

    fn serialize(&self, buf: impl Write);
    fn deserialize(data: &[u8]) -> Self;

    fn handle(&self) -> Handle<Self> {
        let mut data = vec![];
        self.serialize(&mut data);
        Handle::from_digest(digest(&digest::SHA256, &data))
    }

    fn commit(&self) -> Result<Handle<Self>, std::io::Error>
    where
        Self: Sized,
    {
        let mut data = vec![];
        self.serialize(&mut data);

        let digest = digest(&digest::SHA256, &data);
        let path = env::current_dir()?
            .join(Self::ref_name())
            .join(format!("{}.sch", hex::encode(digest.as_ref())));

        fs::create_dir(env::current_dir()?.join(Self::ref_name())).ok();
        fs::write(path, data)?;

        Ok(Handle::from_digest(digest))
    }

    fn delete(&self) -> Result<(), std::io::Error>
    where
        Self: Sized,
    {
        let mut data = vec![];
        self.serialize(&mut data);

        let digest = digest(&digest::SHA256, &data);
        let path = env::current_dir()?
            .join(Self::ref_name())
            .join(format!("{}.sch", hex::encode(digest.as_ref())));

        fs::remove_file(path)?;
        Ok(())
    }
}
