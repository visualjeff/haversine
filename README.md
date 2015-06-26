# haversine
haversine implemented in rust

Add haversine dependency to your project Cargo.toml file:
```json
[dependencies]
haversine = "0.2.0"
```

Example usage:
```rust
extern crate haversine;

use haversine::{distance, Location};

fn main() {
    let start1 = haversine::Location{latitude: 38.898556,longitude: -77.037852};
    let end1 = haversine::Location{latitude: 38.897147, longitude: -77.043934};
    println!("{}", haversine::distance(start1, end1, haversine::Units::Miles));

    let start2 = haversine::Location{latitude: 38.898556,longitude: -77.037852};
    let end2 = haversine::Location{latitude: 38.897147, longitude: -77.043934};
    println!("{}", haversine::distance(start2, end2, haversine::Units::Kilometers));
}
```
