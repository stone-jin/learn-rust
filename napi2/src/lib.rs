#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use futures::prelude::*;
use napi::bindgen_prelude::{ Result, Buffer, Error, Status, BigInt, i64n};
use tokio::fs;


#[napi]
fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
fn get_str() -> Vec<&'static str>{
  vec!["foo", "bar"]
}

#[napi]
fn get_nums() -> Vec<u32> {
  vec![1, 1, 2, 3, 5, 8]
}

#[napi]
fn sum_nums(nums: Vec<u32>) -> u32{
  nums.iter().sum()
}


// async


#[napi]
async fn read_file_async(path: String) -> Result<Buffer> {
  fs::read(path)
    .map(|r| match r {
      Ok(content) => Ok(content.into()),
      Err(e) => Err(Error::new(
        Status::GenericFailure,
        format!("failed to read file, {}", e),
      )),
    })
    .await
}

#[napi]
async fn async_multi_two(num: u32) -> Result<u32>{
  tokio::task::spawn(async move { Ok(num * 2) })
  .await
    .unwrap()
}


// bigint

#[napi]
fn big_add(a: BigInt, b: BigInt) -> u128{
  a.get_u128().1 + b.get_u128().1
}

#[napi]
fn create_big_int_i64() -> i64n{
  i64n(1000)
}

#[napi]
fn create_big_int() -> BigInt {
  BigInt {
    words: vec![100u64, 200u64],
    sign_bit: true,
  }
}