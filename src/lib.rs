#![deny(clippy::all)]

use thumbcache::{get_bitmap_bits, get_hbitmap};
use napi::bindgen_prelude::Uint8Array;

#[macro_use]
extern crate napi_derive;

#[napi(ts_args_type = "path: string, size: 16 | 32 | 48 | 96 | 256 | 768 | 1280 | 1920 | 2560 | (number & {})")]
/// Will find thumbnail for provided file
/// @example
/// ```ts
/// const thumb = getThumb("C:\\Users\\User\\Downloads\\picture.jpg", 96);
/// 
/// if (thumb) {
///   console.log(thumb) // Uint8Array
/// }
/// ```
pub fn get_thumb(path: String, size: i32) -> Option<Uint8Array> {
  if let Ok(hbitmap) = get_hbitmap(&path, size, size, 0x08) {
    let bitmap = get_bitmap_bits(hbitmap, true);

    return Some(Uint8Array::from(bitmap));
  }

  return None;
}
