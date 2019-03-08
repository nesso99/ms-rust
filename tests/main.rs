extern crate ms;
use ms::*;

#[test]
fn test() {
  assert_eq!(ms!("100").unwrap(), 100);
  assert_eq!(ms!("1m").unwrap(), 60000);
  assert_eq!(ms!("1h").unwrap(), 3600000);
  assert_eq!(ms!("2d").unwrap(), 172800000);
  assert_eq!(ms!("1s").unwrap(), 1000);
  assert_eq!(ms!("100ms").unwrap(), 100);
  assert_eq!(ms!("1.5h").unwrap(), 5400000);
  assert_eq!(ms!("1   s").unwrap(), 1000);
  assert_eq!(ms!("1.5H").unwrap(), 5400000);
  assert_eq!(ms!(".5ms").unwrap(), 1);
  assert_eq!(ms!("53 milliseconds").unwrap(), 53);
  assert_eq!(ms!("17 msecs").unwrap(), 17);
  assert_eq!(ms!("1 sec").unwrap(), 1000);
  assert_eq!(ms!("1 min").unwrap(), 60000);
  assert_eq!(ms!("1 hr").unwrap(), 3600000);
  assert_eq!(ms!("2 days").unwrap(), 172800000);
  assert_eq!(ms!("1.5 hours").unwrap(), 5400000);

  assert_eq!(ms!(500, true), "500 ms");
  assert_eq!(ms!(1000, true), "1 second");
  assert_eq!(ms!(1200, true), "1 second");
  assert_eq!(ms!(10000, true), "10 seconds");
  assert_eq!(ms!(60*1000, true), "1 minute");
  assert_eq!(ms!(60*1200, true), "1 minute");
  assert_eq!(ms!(60*10000, true), "10 minutes");
  assert_eq!(ms!(60*60*1000, true), "1 hour");
  assert_eq!(ms!(60*60*1200, true), "1 hour");
  assert_eq!(ms!(60*60*10000, true), "10 hours");
  assert_eq!(ms!(24*60*60*1000, true), "1 day");
  assert_eq!(ms!(24*60*60*1200, true), "1 day");
  assert_eq!(ms!(24*60*60*10000, true), "10 days");

  assert_eq!(ms!(234234234, true), "3 days");
  assert_eq!(ms!(500, false), "500ms");
  assert_eq!(ms!(1000, false), "1s");
  assert_eq!(ms!(10000, false), "10s");
  assert_eq!(ms!(60*1000, false), "1m");
  assert_eq!(ms!(60*10000, false), "10m");
  assert_eq!(ms!(60*60*1000, false), "1h");
  assert_eq!(ms!(60*60*10000, false), "10h");
  assert_eq!(ms!(24*60*60*1000, false), "1d");
  assert_eq!(ms!(24*60*60*10000, false), "10d");
  assert_eq!(ms!(234234234, false), "3d");
}