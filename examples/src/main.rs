//! This is a simple example of how to use the `sum` and `subtract` functions
//!
//! Run with
//!
//! ```not_rust
//! cargo run -p workspace-examples
//! ```

use subtract::subtract;
use sum::sum;

fn main() {
    println!("2 + 2 = {}", sum(2, 2));
    println!("2 - 2 = {}", subtract(2, 2));
}
