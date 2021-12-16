use anyhow::Result;
use mdbx::prelude::*;
use speedy::{Readable, Writable};

#[derive(PartialEq, Debug, Readable, Writable)]
pub struct City {
  name: String,
  lnglat: (u32, u32),
}

impl FromMdbx for City {
  fn from_mdbx(_: PtrTx, val: MDBX_val) -> Self {
    City::read_from_buffer(val_bytes!(val)).unwrap()
  }
}

impl ToAsRef<City, Vec<u8>> for City {
  fn to_as_ref(&self) -> Vec<u8> {
    self.write_to_vec().unwrap()
  }
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
