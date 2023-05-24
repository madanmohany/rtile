# rtile

rtile provides a way to work with rectangular areas of text as atomic units which can be used for code generation

works with nightly build, as it is dependent on the feature 'local_key_cell_methods'

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

    tp!(f1, frame_tile(gtp!(main_result_tile).unwrap(), 5, 2));

    // set new values for main_tile_one and main_tile_two, to reuse the template with different values
    kp!(
        main_tile_one,
        "Reusing the template with different values.     "
    );
    tp!(main_tile_two, "{:?}", vec![1, 2, 3, 4, 5]);
    tp!(f2, frame_tile(gtp!(main_result_tile).unwrap(), 1, 1));

    // set new values for main_tile_one and main_tile_two, to reuse the template with different values
    kp!(
        main_tile_one,
        "{} ",
        t!(",").vjoin(&vec!["1", "2", "3", "4", "5"], true, Some(t!(",")))
    );
    tp!(main_tile_two, vec!["one", "two", "three", "four", "five"]);
    let result = t!(gtp!(main_result_tile).unwrap());
    tp!(f3, frame_tile(result.clone(), 0, 0));

    // set new values for main_tile_one and main_tile_two, to reuse the template with different values
    let dimensions = result.dimensions();
    kp!(main_tile_two, result);
    kp!(
        main_tile_one,
        k!(vec![23, 4, 5, 7, 10]
                .iter()
                .map(|x| if x % 2 == 0 { "Ok(true)" } else { "Err(false)" })
                .collect::<Vec<&str>>()) 
        + k!(vec!["  ==>   "; dimensions.1])
    );
    tp!(f4, frame_tile(gtp!(main_result_tile).unwrap(), 5, 0));

    let result = k!(r#"
        @{f1}
        @{f2}
        @{f3}
        @{f4}
    "#);
    println!("{}", result);
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

```html

        ===================================================
        |                                                 |
        |                                                 |
        |     Welcome to RTile!     Have a great day!     |
        |                                                 |
        |                                                 |
        ===================================================
        ===================================================================
        |                                                                 |
        | Reusing the template with different values.     [1, 2, 3, 4, 5] |
        |                                                                 |
        ===================================================================
        ==========
        |1, one  |
        |2, two  |
        |3, three|
        |4, four |
        |5, five |
        ==========
        ======================================
        |     Err(false)  ==>   1, one       |
        |     Ok(true)    ==>   2, two       |
        |     Err(false)  ==>   3, three     |
        |     Err(false)  ==>   4, four      |
        |     Ok(true)    ==>   5, five      |
        ======================================

```
