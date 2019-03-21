# port_check
A simple rust library to get a free local port or to check if a port somewhere is reachable

Example:
```rust

let free_port = port_check::free_local_port();

let free_port_in_rage = port_check::free_local_port_in_range(10000, 15000);

let is_reachable = port_check::is_port_reachable("192.0.2.0:8080");

```