# port_check
[![crates.io](https://img.shields.io/crates/v/port_check.svg)](https://crates.io/crates/port_check)
![Build Status](https://github.com/ufoscout/port-check-rs/actions/workflows/build_and_test.yml/badge.svg)
[![codecov](https://codecov.io/gh/ufoscout/port-check-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/ufoscout/port-check-rs)

A simple rust library to get a free local port or to check if a port somewhere is reachable

Example:
```rust no_run
use port_check::*;
use std::time::Duration;

// --------------------------------------------------------------------
// If not specified, all port checks are performed for IPv4 addresses.
// --------------------------------------------------------------------

// get a free local port
let free_port = free_local_port().unwrap();

// get a free local port between 10000 and 15000
let free_port_in_range = free_local_port_in_range(10000..=15000);

// check whether a remote port is reachable
let is_reachable = is_port_reachable("192.0.2.0:8080");
// or
let is_reachable = is_port_reachable_with_timeout("192.0.2.0:8080", Duration::from_millis(10_000));



// --------------------------------------------------------------------
// IPv6 checks are supported too
// --------------------------------------------------------------------

let free_ipv6_port = free_local_ipv6_port().unwrap();

let is_ipv6_port_free = is_local_port_free(Port::ipv6(free_ipv6_port));
// or
let is_ipv6_port_free = is_local_ipv6_port_free(free_ipv6_port);

```