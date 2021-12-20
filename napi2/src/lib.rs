#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use futures::prelude::*;
use napi::bindgen_prelude::{ Result, Buffer, Error, Status, BigInt, i64n, Undefined, Symbol,
  ToNapiValue, Null, Promise, Uint32Array, Float32Array};
use tokio::fs;
use std::env;
use napi::{ JsObject, Env, JsSymbol};


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

// class_factory
#[napi]
struct ClassWithFactory{
  pub name: String,
}

#[napi]
impl ClassWithFactory{
  #[napi(factory)]
  pub fn with_name(name: String) -> Self{
    Self { name}
  }

  #[napi]
  pub fn set_name(&mut self, name: String ) -> &Self{
    self.name = name;
    self
  }
}

// enum

#[napi]
pub enum Kind{
  Dog,
  Cat,
  Duck,
}

#[napi]
pub enum CustomNumEnum{
  One = 1,
  Two,
  Three = 3,
  Four,
  Six = 6,
  Eight = 8,
  Nine,
  Ten,
}

#[napi]
fn enum_to_i32(e: CustomNumEnum) -> i32{
  e as i32
}

// nullable
#[napi]
fn map_option(val: Option<u32>) -> Option<u32>{
  val.map(|v| v + 1)
}

#[napi]
fn return_null() -> Null{
  Null
}

#[napi]
fn return_undefined() -> Undefined{
  
}

// number
#[napi]
fn add(a: i32, b: i32) -> i32{
  a + b
}

#[napi(strict)]
fn fibonacci(n: u32)-> u32 {
  match n{
    1|2 => 1,
    _ => fibonacci(n -1 ) + fibonacci(n -2),
  }
}

#[napi]
pub async fn async_plus_100(p: Promise<u32>) -> Result<u32>{
  let v = p.await?;
  Ok(v + 100)
}

#[napi]
pub struct Animal{
  #[napi(readonly)]
  pub kind: Kind,

  name: String,
}

#[napi]
impl Animal{
  #[napi(constructor)]
  pub fn new(kind: Kind, name: String) -> Self{
    Animal { kind, name}
  }

  #[napi(factory)]
  pub fn with_kind(kind: Kind) -> Self{
    Animal{
      kind,
      name: "Default".to_owned(),
    }
  }

  #[napi(getter)]
  pub fn get_name(&self) -> &str{
    self.name.as_str()
  }

  #[napi(setter)]
  pub fn set_name(&mut self, name: String){
    self.name = name;
  }

  #[napi]
  pub fn whoami(&self)->String{
    match self.kind{
      Kind::Dog => format!("Dog: {}", self.name),
      Kind::Cat => format!("Cat: {}", self.name),
      Kind::Duck => format!("Duck: {}", self.name),

    }
  }

  #[napi]
  pub fn get_dog_kind()->Kind{
    Kind::Dog
  }
}

#[napi]
#[repr(transparent)]
pub struct Blake2bHasher(u32);

#[napi]
impl Blake2bHasher{
  #[napi(factory)]
  pub fn with_key(key: &Blake2bKey)->Self{
    Blake2bHasher(key.get_inner())
  }
}

#[napi]
pub struct Blake2bKey(u32);

impl Blake2bKey{
  fn get_inner(&self) -> u32{
    self.0
  }
}

#[napi]
pub struct Context {
  data: String,
  pub maybe_need: Option<bool>,
}

// Test for return `napi::Result` and `Result`
#[napi]
impl Context {
  #[napi(constructor)]
  pub fn new() -> napi::Result<Self> {
    Ok(Self {
      data: "not empty".into(),
      maybe_need: None,
    })
  }

  #[napi(factory)]
  pub fn with_data(data: String) -> Result<Self> {
    Ok(Self {
      data,
      maybe_need: Some(true),
    })
  }

  #[napi]
  pub fn method(&self) -> String {
    self.data.clone()
  }
}

#[napi(constructor)]
pub struct AnimalWithDefaultConstructor {
  pub name: String,
  pub kind: u32,
}

//typed_array

#[napi]
fn get_buffer() -> Buffer{
  String::from("hello world").as_bytes().into()
}

// #[napi]
// fn append_buffer(buf: Buffer)-> Buffer{
//   let mut buf = Vec::<u8>::from(buf);
//   buf.push('!' as u8);
//   buf.into()
// }

#[napi]
fn convert_u32_array(input: Uint32Array) -> Vec<u32>{
  input.to_vec()
}

#[napi]
fn create_extenal_typed_array() -> Uint32Array{
  Uint32Array::new(vec![1,2,3,4])
}

#[napi]
fn mutate_typed_array(mut input: Float32Array){
  for item in input.as_mut(){
    *item *= 2.0;
  }
}

// #[napi]
// fn convert_float_array(input: Float32Array) ->Vec<Number>{
//   input.to_vec()
// }

// symbol
#[napi]
pub fn create_symbol() -> Symbol{
  Symbol::new("a symbol".to_owned())
}

#[napi]
pub fn set_symbol_in_obj(env: Env, symbol: JsSymbol)-> Result<JsObject>{
  let mut obj = env.create_object()?;
  obj.set_property(symbol, env.create_string("a symbol")?)?;
  Ok(obj)
}