use anyhow::Result;
use mdbx::prelude::*;
use mdbx_speedy::MdbxSpeedy;
use speedy::{Readable, Writable};

#[derive(PartialEq, Debug, Readable, Writable, MdbxSpeedy)]
pub struct City {
  name: String,
  lnglat: (u32, u32),
}

env_rw!(MDBX, {
  let mut db_path = std::env::current_exe().unwrap();
  db_path.set_extension("mdb");
  db_path.into()
});

mdbx! {
  MDBX
  Test
    key u16
    val City
}

fn main() -> Result<()> {
  let city = City {
    name: "BeiJing".into(),
    lnglat: (11640, 3990),
  };

  let tx = w!();
  let test = tx | Test;
  test.set(1, city)?;
  println!("{:?}", test.get(1)?);

  Ok(())
}
