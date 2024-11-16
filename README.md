# zuck

![MSRV](https://badgers.space/badge/MSRV/1.82/orange)
![License](https://badgers.space/badge/license/MIT%20OR%20Apache-2.0/blue)
![Crate](https://badgers.space/crates/info/zuck)

Convert human-readable time to `std::time::Duration` and vice versa. No dependencies needed (except `std`).

* [Documentation](https://docs.rs/zuck)
* [Crate](https://crates.io/crates/zuck)

## Stability

This library adheres to [Semantic Versioning](https://semver.org/).

## Usage

Parse the `&str` using `[0-9][alias]` format. Whitespaces between the value are allowed between elements. Repeating time units is not.

#### Aliases

* Nanoseconds: `ns`, `nsec`, `nsecs`, `nanosec`, `nanosecs`, `nanosecond`, `nanoseconds`
* Microseconds: `μs`, `us`, `usec`, `usecs`, `microsec`, `microsecs`, `microsecond`, `microseconds`
* Milliseconds: `ms`, `msec`, `msecs`, `millisecond`, `milliseconds`
* Seconds: `s`, `sec`, `secs`, `second`, `seconds`
* Minutes: `m`, `min`, `mins`, `minute`, `minutes`
* Hours: `h`, `hr`, `hrs`, `hour`, `hours`
* Days: `d`, `day`, `days`
* Weeks: `w`, `wk`, `wks`, `week`, `weeks`
* Months: `mo`, `month`, `months`
* Years: `y`, `yr`, `yrs`, `year`, `years`

## Example

```rust
let duration = zuck::Duration::from_str("1yr2mo3w4d5h6m7s8ms9microsec10ns").unwrap();

assert_eq!(
    duration,
    Duration {
        nanoseconds: 10,
        microseconds: 9,
        milliseconds: 8,
        seconds: 7,
        minutes: 6,
        hours: 5,
        days: 18,
        months: 2,
        years: 1
    }
);
```

## Miscellaneous

#### RegExp to use for string validation in other tools (ex: JSONSchema)

```
\b(\d{1,32} ?((ns)|(nsecs?)|(nanosecs?)|(nanosecs?)|(nanoseconds?)|(μs)|(us)|(usecs?)|(microsecs?)|(microseconds?)|(ms)|(msecs?)|(milliseconds?)|(s)|(secs?)|(second)|(seconds?)|(m)|(mins?)|(minutes?)|(h)|(hrs?)|(hours?)|(d)|(days?)|(w)|(wks?)|(weeks?)|(mo)|(months?)|(y)|(yrs?)|(years?)))( ?(\d{1,32} ?((ns)|(nsecs?)|(nanosecs?)|(nanosecs?)|(nanoseconds?)|(μs)|(us)|(usecs?)|(microsecs?)|(microseconds?)|(ms)|(msecs?)|(milliseconds?)|(s)|(secs?)|(second)|(seconds?)|(m)|(mins?)|(minutes?)|(h)|(hrs?)|(hours?)|(d)|(days?)|(w)|(wks?)|(weeks?)|(mo)|(months?)|(y)|(yrs?)|(years?))))*\b
```
With case-insensitive flag enabled.

Though it doesn't cover the failure in case if repeating of the same time unit occurs.


## License

This library is primarily distributed under the terms of either the *MIT license* or the *Apache License (Version 2.0)* at your option.

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.