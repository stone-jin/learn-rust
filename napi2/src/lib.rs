#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use futures::prelude::*;
use napi::bindgen_prelude::{ Result, Buffer, Error, Status, BigInt, i64n};
use tokio::fs;
use std::env;


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

// callback

#[napi]
fn get_current_dir<T: Fn(String) -> Result<()>>(callback: T) {
  callback(env::current_dir().unwrap().to_string_lossy().to_string()).unwrap();
}

#[napi]
fn read_file<T: Fn(Result<()>, Option<String>) -> Result<()>>(callback: T) {
  match read_file_content(){
    Ok(s) => callback(Ok(()), Some(s)),
    Err(e) => callback(Err(e), None),
  }.unwrap();
}

fn read_file_content() -> Result<String>{
  Ok("hello world".to_string())
}

// error

#[napi]
fn throw_error()->Result<()>{
  Err(Error::new(Status::InvalidArg, "Manual Error".to_owned()))
}

