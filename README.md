# RTile

RTile is a simple Rust lib module meant to help with code generation.
It provides a way to work with rectangular areas of text as atomic units.
Can be used with nightly build only, till the feature 'local_key_cell_methods' is stabilized.

Inspired by a python module [tiles](https://github.com/sustrik/tiles), developed by [Martin Sustrik](https://github.com/sustrik).

## How to use

```rust
use rtile::*;

fn main() {
    kp!(main_tile_one, "Welcome to RTile!     ");
    tp!(main_tile_two, "     Have a great day!     ");

    kp!(main_combined_tiles, "@{main_tile_one}@{main_tile_two}");

    // apply top bottom spacing,
    kp!(
        combined_tiles_with_top_bottom_spacing,
        "@{main_top_bottom_spaces}\n@{main_combined_tiles}\n@{main_top_bottom_spaces}"
    );
    // apply left right unit spacing, now we have the main_result_tile
    kp!(
            main_result_tile,
            "@{main_left_right_spaces}@{combined_tiles_with_top_bottom_spacing}@{main_left_right_spaces}"
        );

    println!("{}", frame_tile(gtp!(main_result_tile).unwrap(), 5, 2));
}

fn set_spacing(width_spacing: usize, height_spacing: usize) {
    //set the spacing if required - maintain unit spacing, that is sufficient to create the frame
    let left_right_spaces = vec![" "; width_spacing];
    kp!(main_left_right_spaces, vec![left_right_spaces.join(""); 1]);
    let top_bottom_spaces = vec![" "; 1];
    kp!(
        main_top_bottom_spaces,
        vec![top_bottom_spaces.join(""); height_spacing]
    );
}

fn frame_tile(input: RTile, width_spacing: usize, height_spacing: usize) -> RTile {
    set_spacing(width_spacing, height_spacing);
    if height_spacing > 0 {
        // apply top bottom spacing,
        kp!(
            combined_tiles_with_top_bottom_spacing,
            "@{main_top_bottom_spaces}\n@{main_combined_tiles}\n@{main_top_bottom_spaces}"
        );
    } else {
        // remove the top bottom spacing,
        tp!(
            combined_tiles_with_top_bottom_spacing,
            "@{main_top_bottom_spaces}\n@{main_combined_tiles}\n@{main_top_bottom_spaces}"
        );
    }

    let (width, height) = kp!(frame_tile, input).dimensions();

    tp!(main_height, vec!["|"; height]);
    tp!(main_width, vec!["="; width + 2].join(""));

    t!(r#"
                @{main_width}
                @{main_height}@{frame_tile}@{main_height}
                @{main_width}
        "#)
}

```

## Output

```rust
===================================================
|                                                 |
|                                                 |
|     Welcome to RTile!     Have a great day!     |
|                                                 |
|                                                 |
===================================================
```
