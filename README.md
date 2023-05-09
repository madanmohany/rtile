# rtile

rtile provides a way to work with rectangular areas of text as atomic units

works with nightly build, as it is dependent on the feature 'local_key_cell_methods'

## How to use

```rust
use rtile::*;
fn main() {
    kp!(greet_one, "Welcome to rtile!     ");
    tp!(greet_two, "Have a great day!");
    println!("{}", ts!("@{greet_one}@{greet_two}"));
}
```

## Output

```html
Welcome to rtile!     Have a great day!
```
