// ignore-windows: TODO clock shims are not implemented on Windows
// compile-flags: -Zmiri-disable-isolation

use std::time::SystemTime;

fn main() {
   let now1 = SystemTime::now();

    // Do some work to make time pass.
    for _ in 0..10 { drop(vec![42]); }

   let now2 = SystemTime::now();
   assert!(now2 > now1);
}
