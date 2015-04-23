#[macro_use]
extern crate nom;

use nom::{IResult,Needed,Producer,FileProducer,ProducerState,not_line_ending};
use nom::IResult::*;

use std::str::from_utf8;
use std::fmt::Debug;

#[test]
#[allow(unused_must_use)]
fn tag() {
  FileProducer::new("links.txt", 20).map(|producer: FileProducer| {
    let mut p = producer;
    named!(pr<&[u8], ()>, flat_map!(map_res!(tag!("https!"), from_utf8), print));
    pusher!(ps, pr);
    ps(&mut p);
    //assert!(false);
  });
}

pub fn print<T: Debug>(input: T) -> IResult<T,()> {
  println!("{:?}", input);
  Done(input, ())
}


#[test]
fn is_not() {
  //is_not!(foo b"\r\n");
  named!(foo<&[u8],&[u8]>, is_not!(&b"\r\n"[..]));
  let a = &b"ab12cd\nefgh"[..];
  assert_eq!(foo(a), Done(&b"\nefgh"[..], &b"ab12cd"[..]));
}

#[test]
fn exported_public_method_defined_by_macro() {
  let a = &b"ab12cd\nefgh"[..];
  assert_eq!(not_line_ending(a), Done(&b"\nefgh"[..], &b"ab12cd"[..]));
}

