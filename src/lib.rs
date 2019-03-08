extern crate regex;

use regex::Regex;

const S: f64 = 1000.0;
const M: f64 = S * 60.0;
const H: f64 = M * 60.0;
const D: f64 = H * 24.0;
const Y: f64 = D*365.0 + D/4.0;

#[macro_export]
macro_rules! ms {
  ( $val:expr ) => {
    {
      __to_ms__($val)
    }
  };

  ( $val:expr, $fmt:expr ) => {
    {
      __to_string__($val, $fmt)
    }
  };
}

pub fn __to_ms__(val: &str) -> Option<u64> {
  if val.len() == 0 {
    None
  } else {
    match process(val) {
      Some(result) => Some(result),
      None => None
    }
  }
}

pub fn __to_string__(val: u64, long_format: bool) -> String {
  if long_format {
    return fmt_long(val);
  }
  return fmt_short(val);
}

fn process(val: &str) -> Option<u64> {
  if val.len() > 100 {
		None
	} else {
    let re = Regex::new(r"(?i)^((?:\d+)?\.?\d+) *(milliseconds?|msecs?|ms|seconds?|secs?|s|minutes?|mins?|m|hours?|hrs?|h|days?|d|weeks?|w|years?|yrs?|y)?$").unwrap();
    match re.captures(val) {
      Some(caps) => {
        let n: f64 = caps.get(1).map_or("", |m| m.as_str()).parse().unwrap();
        let match2 = caps.get(2).map_or("ms", |m| m.as_str());
        let time_type = match2.to_lowercase();
        
        match time_type.as_str() {
          "years" | "year" | "yrs" | "yr" | "y"                    => Some((n * Y).round() as u64),
          "days" | "day" | "d"                                     => Some((n * D).round() as u64),
          "hours" | "hour" | "hrs" | "hr" | "h"                    => Some((n * H).round() as u64),
          "minutes" | "minute" | "mins" | "min" | "m"              => Some((n * M).round() as u64),
          "seconds" | "second" | "secs" | "sec" | "s"              => Some((n * S).round() as u64),
          "milliseconds" | "millisecond" | "msecs" | "msec" | "ms" => Some(n.round() as u64),
          _ => Some(0)
        }
      },
      None => None
    }
  }
}

fn fmt_long(val: u64) -> String {
  let ms = val as f64;
  if ms >= D {
		return plural(ms, D, "day");
	}
	if ms >= H {
		return plural(ms, H, "hour");
	}
	if ms >= M {
		return plural(ms, M, "minute");
	}
	if ms >= S {
		return plural(ms, S, "second");
	}
	return format!("{} ms", val);
}

fn fmt_short(val: u64) -> String {
  let result: u64;
  let ms = val as f64;
	if ms >= D {
		result = (ms / D).round() as u64;
		return format!("{}d", result);
	}
  if ms >= H {
		result = (ms / H).round() as u64;
		return format!("{}h", result);
	}
  if ms >= M {
		result = (ms / M).round() as u64;
		return format!("{}m", result);
	}
	if ms >= S {
		result = (ms / S).round() as u64;
		return format!("{}s", result);
	}
	return format!("{}ms", val);
}

fn plural(ms: f64, n: f64, name: &str) -> String {
  let result = (ms/n).round() as u64;
	if ms < n+n/2.0 {
		return format!("{} {}", result, name);
	}
  return format!("{} {}s", result, name);
}