//!
//! rtile provides a way to work with rectangular areas of text as atomic units which can be used for code generation.
//!
//! ```
//! use rtile::*;
//! kp!(greet_one, "Welcome to rtile!     ");
//! tp!(greet_two, "Have a great day!");
//! assert_eq!(ts!("@{greet_one}@{greet_two}"), "Welcome to rtile!     Have a great day!");
//! ```
//!

#![warn(missing_docs)]

use std::any::type_name;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::iter::Iterator;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::BitOr;
use std::ops::BitOrAssign;

///
/// give a name to a tile using any string literal and persist it in tls (thread local storage)
/// ```
/// use rtile::*;
/// let tile = t!("tile value");
/// stp!(persisted_tile_name, tile);
/// assert_eq!("tile value".to_string(), t!("@{persisted_tile_name}").to_string());
/// ```
///
#[macro_export]
macro_rules! stp {
    ($i: ident, $t: expr) => {{
        set_tiles(format!("{}", stringify!($i)), $t.to_string());
        set_raw_tiles(format!("{}", stringify!($i)), $t.clone());
    }};
}

///
/// give a name to a tile using a variable containing a string value and persist it in tls (thread local storage)
/// ```
/// use rtile::*;
/// let tile = t!("tile value");
/// let name_of_the_persisted_tile = "persisted_tile_name";
/// stq!(name_of_the_persisted_tile, tile);
/// assert_eq!("tile value".to_string(), t!("@{persisted_tile_name}").to_string());
/// ```
///
#[macro_export]
macro_rules! stq {
    ($e: expr, $t: expr) => {{
        set_tiles(format!("{}", $e), $t.to_string());
        set_raw_tiles(format!("{}", $e), $t.clone());
    }};
}

///
/// get the tile which is persisted in the tls (thread local storage)
/// ```
/// use rtile::*;
/// let tile = t!("tile value");
/// stp!(persisted_tile_name, tile);
/// let result = gtp!(persisted_tile_name).unwrap();
/// assert_eq!(tile, result);
/// ```
///
#[macro_export]
macro_rules! gtp {
    ($i: ident) => {{
        get_raw_tile(&stringify!($i).to_string())
    }};
}

///
/// get the tile which is persisted in the tls (thread local storage) using a variable containing a string value
/// ```
/// use rtile::*;
/// let tile = t!("tile value");
/// stp!(persisted_tile_name, tile);
/// let name_of_the_persisted_tile = "persisted_tile_name";
/// let result = gtq!(name_of_the_persisted_tile).unwrap();
/// assert_eq!(tile, result);
/// ```
///
#[macro_export]
macro_rules! gtq {
    ($e: expr) => {{
        let target_tile_name = format!("{}", $e);
        get_raw_tile(&target_tile_name)
    }};
}

#[doc(hidden)]
///
/// Tiles with trimming
///
/// t - trim white spaces - do_trimming: true
///
pub trait MacroAttributeForT {
    #[doc(hidden)]
    fn process(&self) -> RTile;
}

impl MacroAttributeForT for &str {
    fn process(&self) -> RTile {
        RTile::construct_from_str(self)
    }
}

impl MacroAttributeForT for String {
    fn process(&self) -> RTile {
        RTile::construct_from_str(self.as_str())
    }
}

impl MacroAttributeForT for &String {
    fn process(&self) -> RTile {
        RTile::construct_from_str(self.as_str())
    }
}

impl MacroAttributeForT for RTile {
    fn process(&self) -> RTile {
        RTile::construct_from_str(self.to_string().as_str())
    }
}

impl MacroAttributeForT for &RTile {
    fn process(&self) -> RTile {
        RTile::construct_from_str(self.to_string().as_str())
    }
}

impl MacroAttributeForT for Vec<&str> {
    fn process(&self) -> RTile {
        RTile::new_str(self.clone())
    }
}

impl MacroAttributeForT for Vec<String> {
    fn process(&self) -> RTile {
        RTile::new(self.clone())
    }
}

/// tf! is used to flatten the multilines of the tile output into a single string
///
/// ```
/// use rtile::*;
/// tp!(
///     tile_2,
///     "
///             seven,
///     "
/// );
/// tp!(
///     tile_1,
///     "
///             six,
///             @{tile_2}
///             eight,
///     "
/// );
/// let input_tile = t!("
///             one,
///             two,
///             three,
///             four,
///             five,
///             @{tile_1}
///             nine,
///             ten
/// ");
/// let output = tf!(input_tile);
/// let expected_output = "one,two,three,four,five,six,seven,eight,nine,ten".to_string();
/// assert_eq!(output, expected_output);
///
/// let v1 = vec!["  one  ", "  two  ", "  three  "];
/// let v2 = vec!["  1  ", "  2  ", "  3  "];
/// let val = k!(v1) + k!(v2);
/// assert_eq!(tf!(val), "one      1two      2three    3");
/// ```
///        
#[macro_export]
macro_rules! tf {
    ($t: expr) => {{
        $t.to_string()
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|&item| item.trim())
            .collect::<Vec<&str>>()
            .join("")
    }};
}

/// t! is to expand any inner tiles and to trim the white spaces around the block of text and return a tile
///
/// ```
/// use rtile::*;
/// tp!(
///     tile_2,
///     "
///             seven,
///     "
/// );
/// tp!(
///     tile_1,
///     "
///             six,
///             @{tile_2}
///             eight,
///     "
/// );
/// let input_tile = t!("
///             one,
///             two,
///             three,
///             four,
///             five,
///             @{tile_1}
///             nine,
///             ten
/// ");
/// let output = input_tile.to_string();
/// let expected_output = "one,\ntwo,\nthree,\nfour,\nfive,\nsix,\nseven,\neight,\nnine,\nten".to_string();
/// assert_eq!(output, expected_output);
/// ```
///   
#[macro_export]
macro_rules! t {
    () => {{
        RTile::new(vec![])
    }};
    ("") => {{
        RTile {
            name: None,
            lns: vec!["".to_string()],
            do_trimming: true,
        }
    }};
    ($e:expr) => {{
        MacroAttributeForT::process(&$e)
    }};
    ($($arg:tt)*) => {{
        let val = format!($($arg)*);
        t!(val)
    }};
}

/// tp! is to used to persist the tile into the tls (thread local storage), with a given name (string literal) and return a tile
///
/// ```
/// use rtile::*;
/// tp!(
///     tile_one,
///     "
///             one
///
///             two
///     "
/// );
/// tp!(
///     tile_two,
///     "
///             three
///             four
///             five
///     "
/// );
/// let input_tile = t!("
///             @{tile_one} @{tile_two}
/// ");
/// let output = input_tile.to_string();
/// let expected_output = ts!("
///                          one three
///                              four
///                          two five
///                          ");
/// assert_eq!(output, expected_output);
/// ```
///   
#[macro_export]
macro_rules! tp {
    ($i:ident) => {{
        let mut $i = t!();
        $i.name = Some(stringify!($i).to_string());
        set_tiles(format!("{}", stringify!($i)), $i.to_string());
        set_raw_tiles(format!("{}", stringify!($i)), $i.clone());
        $i
    }};
    ($i:ident, $e:expr) => {{
        let mut $i = t!($e);
        $i.name = Some(stringify!($i).to_string());
        set_tiles(format!("{}", stringify!($i)), $i.to_string());
        set_raw_tiles(format!("{}", stringify!($i)), $i.clone());
        $i
    }};
    ($i:ident, $($arg:tt)*) => {{
        let val = format!($($arg)*);
        let mut $i = t!(val);
        $i.name = Some(stringify!($i).to_string());
        set_tiles(format!("{}", stringify!($i)), $i.to_string());
        set_raw_tiles(format!("{}", stringify!($i)), $i.clone());
        $i
    }};
}

/// tq! is to used to persist the tile into the tls (thread local storage), with a variable having a string value and return a tile
///
/// ```
///         
/// use rtile::*;
/// let persisted_tile_one = "tile_one";
/// let persisted_tile_two = "tile_two";
/// tq!(
///     persisted_tile_one,
///     "
///             one
///
///             two
///     "
/// );
/// tq!(
///     persisted_tile_two,
///     "
///             three
///             four
///             five
///     "
/// );
/// let input_tile = t!("
///             @{tile_one} @{tile_two}
/// ");
/// let output = input_tile.to_string();
/// let expected_output = ts!("
///                          one three
///                              four
///                          two five
///                          ");
/// assert_eq!(output, expected_output);
/// ```
///  
#[macro_export]
macro_rules! tq {
    ($e:expr) => {{
        let mut target_tile = t!();
        let target_tile_name = format!("{}", $e);
        target_tile.name = Some(target_tile_name.clone());
        set_tiles(target_tile_name.clone(), target_tile.to_string());
        set_raw_tiles(target_tile_name, target_tile.clone());
        target_tile
    }};
    ($e:expr, $val:expr) => {{
        let mut target_tile = t!($val);
        let target_tile_name = format!("{}", $e);
        target_tile.name = Some(target_tile_name.clone());
        set_tiles(target_tile_name.clone(), target_tile.to_string());
        set_raw_tiles(target_tile_name, target_tile.clone());
        target_tile
    }};
    ($e:expr, $($arg:tt)*) => {{
        let val = format!($($arg)*);
        let mut target_tile = t!(val);
        let target_tile_name = format!("{}", $e);
        target_tile.name = Some(target_tile_name.clone());
        set_tiles(target_tile_name.clone(), target_tile.to_string());
        set_raw_tiles(target_tile_name, target_tile.clone());
        target_tile
    }};
}

/// tt! is to used to expand the inner tiles and return the expanded ouput as a trimmed tile
///
/// ```
/// use rtile::*;
///
/// tp!(numbers, "1, 2, 3, 4, 5");
/// let mut result = tt!("Numbers: @{numbers}");
/// tp!(numbers, "one, two, three, four, five");
/// result |= t!("In words: @{numbers}");
/// assert_eq!(result.to_string(), ts!("
///                                     Numbers: 1, 2, 3, 4, 5
///                                     In words: one, two, three, four, five
///                                     "));
/// ```
#[macro_export]
macro_rules! tt {
    ($e:expr) => {{
        t!(t!($e))
    }};
    ($($arg:tt)*) => {{
        let val = format!($($arg)*);
        t!(t!(val))
    }};
}

/// ttp! is to used to expand the inner tiles, persist the result to tls (thread local storage) using a string literal and return a trimmed tile
///
/// ```
/// use rtile::*;
///
/// tp!(numbers, "1, 2, 3, 4, 5");
/// ttp!(numbers, "Numbers: @{numbers}");
/// assert_eq!(t!("@{numbers}").to_string(), "Numbers: 1, 2, 3, 4, 5".to_string());
/// ```
#[macro_export]
macro_rules! ttp {
    ($i:ident, $e:expr) => {{
        let mut $i = t!(t!($e));
        $i.name = Some(stringify!($i).to_string());
        set_tiles(format!("{}", stringify!($i)), $i.to_string());
        set_raw_tiles(format!("{}", stringify!($i)), $i.clone());
        $i
    }};
    ($i:ident, $($arg:tt)*) => {{
        let val = format!($($arg)*);
        let mut $i = t!(t!(val));
        $i.name = Some(stringify!($i).to_string());
        set_tiles(format!("{}", stringify!($i)), $i.to_string());
        set_raw_tiles(format!("{}", stringify!($i)), $i.clone());
        $i
    }};
}

/// ttq! is to used to expand the inner tiles, persist the result to tls (thread local storage) using a variable name and return a trimmed tile
///
/// ```
/// use rtile::*;
///
/// tp!(numbers, "1, 2, 3, 4, 5");
/// let persisted_tile_name = "numbers";
/// ttq!(persisted_tile_name, "Numbers: @{numbers}");
/// assert_eq!(t!("@{numbers}").to_string(), "Numbers: 1, 2, 3, 4, 5".to_string());
/// ```
#[macro_export]
macro_rules! ttq {
    ($e:expr, $val:expr) => {{
        let mut target_tile = t!(t!($val));
        let target_tile_name = format!("{}", $e);
        target_tile.name = Some(target_tile_name.clone());
        set_tiles(target_tile_name.clone(), target_tile.to_string());
        set_raw_tiles(target_tile_name, target_tile.clone());
        target_tile
    }};
    ($e:expr, $($arg:tt)*) => {{
        let val = format!($($arg)*);
        let mut target_tile = t!(t!(val));
        let target_tile_name = format!("{}", $e);
        target_tile.name = Some(target_tile_name.clone());
        set_tiles(target_tile_name.clone(), target_tile.to_string());
        set_raw_tiles(target_tile_name, target_tile.clone());
        target_tile
    }};
}

/// sr! returns the trimmed raw data of a tile
///
/// ```
/// use rtile::*;
/// let tile = t!("
///                @{numbers}
///                @{alphabets}
///                ");
/// let result = sr!(tile);
/// assert_eq!(result, "@{numbers}\n@{alphabets}".to_string());
/// ```
#[macro_export]
macro_rules! sr {
    ($e:expr) => {{
        $e.raw()
    }};
    ($($arg:tt)*) => {{
        let val = format!($($arg)*);
        t!(val).raw()
    }};
}

/// ts! is to expand any inner tiles and to trim the white spaces around the block of text and return a String
/// ```
/// use rtile::*;
/// tp!(tile_one, "   one   ");
/// tp!(tile_two, "   two   ");
/// let result = ts!("
///                 @{tile_one}
///                 @{tile_two}
///                 ");
/// assert_eq!(result, "one\ntwo");
/// ```
#[macro_export]
macro_rules! ts {
    () => {{
        "".to_string()
    }};
    ($e:expr) => {{
        t!($e).to_string()
    }};
    ($($arg:tt)*) => {{
        let val = format!($($arg)*);
        t!(val).to_string()
    }};
}

#[doc(hidden)]
///
/// Tiles without trimming
///
/// k - keep white spaces - do_trimming: false
///
pub trait MacroAttributeForK {
    #[doc(hidden)]
    fn process(&self) -> RTile;
}

impl MacroAttributeForK for &str {
    fn process(&self) -> RTile {
        RTile::from_str_without_trimming(self)
    }
}

impl MacroAttributeForK for String {
    fn process(&self) -> RTile {
        RTile::from_str_without_trimming(self.as_str())
    }
}

impl MacroAttributeForK for &String {
    fn process(&self) -> RTile {
        RTile::from_str_without_trimming(self.as_str())
    }
}

impl MacroAttributeForK for RTile {
    fn process(&self) -> RTile {
        RTile::from_str_without_trimming(self.to_string().as_str())
    }
}

impl MacroAttributeForK for &RTile {
    fn process(&self) -> RTile {
        RTile::from_str_without_trimming(self.to_string().as_str())
    }
}

impl MacroAttributeForK for Vec<&str> {
    fn process(&self) -> RTile {
        RTile::new_without_trimming_str(self.clone())
    }
}

impl MacroAttributeForK for Vec<String> {
    fn process(&self) -> RTile {
        RTile::new_without_trimming(self.clone())
    }
}

/// kf! is used to flatten the multilines of the tile output into a single string, without trimming the white spaces. i.e. keep the white spaces
///
/// ```
/// use rtile::*;
/// let v1 = vec!["  one  ", "  two  ", "  three  "];
/// let val = k!(v1);
/// assert_eq!(kf!(val), "  one    two    three  ");
///
/// let v1 = vec!["  one  ", "  two  ", "  three  "];
/// let v2 = vec!["  1  ", "  2  ", "  3  "];
/// let val = k!(v1) + k!(v2);
/// assert_eq!(kf!(val), "  one      1    two      2    three    3  ");
/// ```
///        
#[macro_export]
macro_rules! kf {
    ($t: expr) => {{
        $t.to_string().split("\n").collect::<Vec<&str>>().join("")
    }};
}

/// k! is to expand any inner tiles, to keep the white spaces (i.e. do not trim any white spaces around the block) and return a tile
///
/// ```
/// use rtile::*;
///
/// let v1 = vec!["  one  ", "  two  ", "  three  "];
/// let val = k!(v1);
/// assert_eq!(val.to_string(), "  one  \n  two  \n  three  ");
/// ```
#[macro_export]
macro_rules! k {
    () => {{
        RTile::new_without_trimming(vec![])
    }};
    ("") => {{
        RTile {
            name: None,
            lns: vec!["".to_string()],
            do_trimming: false,
        }
    }};
    ($e:expr) => {{
        MacroAttributeForK::process(&$e)
    }};
    ($($arg:tt)*) => {{
        let val = format!($($arg)*);
        k!(val)
    }};
}

/// kp! is to expand any inner tiles, to keep the white spaces (i.e. do not trim any white spaces around the block), with a given name (string literal) and return a tile
///
/// ```
/// use rtile::*;
///
/// kp!(tile_1, " abc ");
/// kp!(tile_2, "@{tile_1}");
///
/// let result = k!("@{tile_2}");
/// assert_eq!(ks!(result), " abc ");
/// ```
#[macro_export]
macro_rules! kp {
    ($i:ident) => {{
        let mut $i = k!();
        $i.name = Some(stringify!($i).to_string());
        set_tiles(format!("{}", stringify!($i)), $i.to_string());
        set_raw_tiles(format!("{}", stringify!($i)), $i.clone());
        $i
    }};
    ($i:ident, $e:expr) => {{
        let mut $i = k!($e);
        $i.name = Some(stringify!($i).to_string());
        set_tiles(format!("{}", stringify!($i)), $i.to_string());
        set_raw_tiles(format!("{}", stringify!($i)), $i.clone());
        $i
    }};
    ($i:ident, $($arg:tt)*) => {{
        let val = format!($($arg)*);
        let mut $i = k!(val);
        $i.name = Some(stringify!($i).to_string());
        set_tiles(format!("{}", stringify!($i)), $i.to_string());
        set_raw_tiles(format!("{}", stringify!($i)), $i.clone());
        $i
    }};
}

/// kq! is to expand any inner tiles, to keep the white spaces (i.e. do not trim any white spaces around the block), with a variable having a string value and return a tile
///
/// ```
/// use rtile::*;
/// let t1 = "tile_1";
/// let t2 = "tile_2";
///
/// kq!(t1, " abc ");
/// kq!(t2, "@{tile_1}");
///
/// let result = k!("@{tile_2}");
/// assert_eq!(ks!(result), " abc ");
/// ```
#[macro_export]
macro_rules! kq {
    ($e:expr) => {{
        let mut target_tile = k!();
        let target_tile_name = format!("{}", $e);
        target_tile.name = Some(target_tile_name.clone());
        set_tiles(target_tile_name.clone(), target_tile.to_string());
        set_raw_tiles(target_tile_name, target_tile.clone());
        target_tile
    }};
    ($e:expr, $val:expr) => {{
        let mut target_tile = k!($val);
        let target_tile_name = format!("{}", $e);
        target_tile.name = Some(target_tile_name.clone());
        set_tiles(target_tile_name.clone(), target_tile.to_string());
        set_raw_tiles(target_tile_name, target_tile.clone());
        target_tile
    }};
    ($e:expr, $($arg:tt)*) => {{
        let val = format!($($arg)*);
        let mut target_tile = k!(val);
        let target_tile_name = format!("{}", $e);
        target_tile.name = Some(target_tile_name.clone());
        set_tiles(target_tile_name.clone(), target_tile.to_string());
        set_raw_tiles(target_tile_name, target_tile.clone());
        target_tile
    }};
}

/// kk! is to used to expand the inner tiles, by keeping the white spaces (i.e. do not trim any white spaces around the block) and return a tile
///
/// ```
/// use rtile::*;
///
/// kp!(numbers, "     1, 2, 3, 4, 5     ");
/// let mut result = kk!("  Numbers: @{numbers}  ");
/// kp!(numbers, "   one, two, three, four, five   ");
/// result |= k!("  In words: @{numbers}  ");
/// assert_eq!(result.to_string(), ks!("  Numbers:      1, 2, 3, 4, 5       \n  In words:    one, two, three, four, five     "));
/// ```
#[macro_export]
macro_rules! kk {
    ($e:expr) => {{
        k!(k!($e))
    }};
    ($($arg:tt)*) => {{
        let val = format!($($arg)*);
        k!(k!(val))
    }};
}

/// kkp! is to used to expand the inner tiles, by keeping the white spaces (i.e. do not trim any white spaces around the block), persist the result to tls (thread local storage) using a string literal and return a tile
///
/// ```
/// use rtile::*;
///
/// kp!(numbers, "     1, 2, 3, 4, 5     ");
/// kkp!(numbers, "  Numbers: @{numbers}  ");
/// assert_eq!(ks!("@{numbers}"), "  Numbers:      1, 2, 3, 4, 5       ".to_string());
/// ```
#[macro_export]
macro_rules! kkp {
    ($i:ident, $e:expr) => {{
        let mut $i = k!(k!($e));
        $i.name = Some(stringify!($i).to_string());
        set_tiles(format!("{}", stringify!($i)), $i.to_string());
        set_raw_tiles(format!("{}", stringify!($i)), $i.clone());
        $i
    }};
    ($i:ident, $($arg:tt)*) => {{
        let val = format!($($arg)*);
        let mut $i = k!(k!(val));
        $i.name = Some(stringify!($i).to_string());
        set_tiles(format!("{}", stringify!($i)), $i.to_string());
        set_raw_tiles(format!("{}", stringify!($i)), $i.clone());
        $i
    }};
}

/// kkq! is to used to expand the inner tiles, by keeping the white spaces (i.e. do not trim any white spaces around the block), persist the result to tls (thread local storage) using a variable having a string value and return a tile
///
/// ```
/// use rtile::*;
/// let tile_name = "numbers";
/// kq!(tile_name, "     1, 2, 3, 4, 5     ");
/// kkq!(tile_name, "  Numbers: @{numbers}  ");
/// assert_eq!(ks!("@{numbers}"), "  Numbers:      1, 2, 3, 4, 5       ".to_string());
/// ```
#[macro_export]
macro_rules! kkq {
    ($e:expr, $val:expr) => {{
        let mut target_tile = k!(k!($val));
        let target_tile_name = format!("{}", $e);
        target_tile.name = Some(target_tile_name.clone());
        set_tiles(target_tile_name.clone(), target_tile.to_string());
        set_raw_tiles(target_tile_name, target_tile.clone());
        target_tile
    }};
    ($e:expr, $($arg:tt)*) => {{
        let val = format!($($arg)*);
        let mut target_tile = k!(k!(val));
        let target_tile_name = format!("{}", $e);
        target_tile.name = Some(target_tile_name.clone());
        set_tiles(target_tile_name.clone(), target_tile.to_string());
        set_raw_tiles(target_tile_name, target_tile.clone());
        target_tile
    }};
}

/// ks! is to expand any inner tiles by keeping the white spaces (i.e. do not trim any white spaces around the block) and return a String
/// ```
/// use rtile::*;
/// kp!(tile_one, "   one   ");
/// kp!(tile_two, "   two   ");
/// let result = ks!("@{tile_one}, @{tile_two}");
/// assert_eq!(result, "   one   ,    two   ");
/// ```
#[macro_export]
macro_rules! ks {
    () => {{
        "".to_string()
    }};
    ($e:expr) => {{
        k!($e).to_string()
    }};
    ($($arg:tt)*) => {{
        let val = format!($($arg)*);
        k!(val).to_string()
    }};
}

thread_local! {
    static TL_PROCESSED_TILES: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
    static TL_RAW_TILES: RefCell<HashMap<String, RTile>> = RefCell::new(HashMap::new());
}

#[doc(hidden)]
pub fn set_tiles(key: String, value: String) {
    TL_PROCESSED_TILES.with_borrow_mut(|v| v.insert(key, value));
}

#[doc(hidden)]
pub fn set_raw_tiles(key: String, value: RTile) {
    TL_RAW_TILES.with_borrow_mut(|v| v.insert(key, value));
}

#[doc(hidden)]
pub fn get_raw_tile(key: &str) -> Option<RTile> {
    let key = &key.to_string();

    TL_RAW_TILES.with_borrow(|v| {
        if v.contains_key(key) {
            Some(v.get(key).unwrap().clone())
        } else {
            None
        }
    })
}

/// remove_tile, used to remove a tile by name from the tls (thread local storage)
/// ```
/// use rtile::*;
///
/// tp!(tile1, "one");
/// tp!(tile2, "two");
/// assert_eq!(ts!("@{tile1}-@{tile2}"), "one-two".to_string());
/// //remove the tile
/// remove_tile("tile1");
/// //If the tile is not present, a blank tile would be created using that name
/// assert_eq!(ts!("@{tile1}-@{tile2}"), "-two".to_string());
/// remove_tile("tile2");
/// //If the tile is not present, a blank tile would be created using that name
/// assert_eq!(ts!("@{tile1}-@{tile2}"), "-".to_string());
/// ```
pub fn remove_tile(key: &str) {
    let key = &key.to_string();
    TL_RAW_TILES.with_borrow_mut(|v| v.remove(key));
    TL_PROCESSED_TILES.with_borrow_mut(|v| v.remove(key));
}

/// clear_tiles, used to remove all tiles from the tls (thread local storage)
/// ```
/// use rtile::*;
///
/// tp!(tile1, "one");
/// tp!(tile2, "two");
/// assert_eq!(ts!("@{tile1}-@{tile2}"), "one-two".to_string());
/// //remove the tile
/// clear_tiles();
/// //If the tile is not present, a blank tile would be created using that name
/// assert_eq!(ts!("@{tile1}-@{tile2}"), "-".to_string());
/// ```
pub fn clear_tiles() {
    TL_RAW_TILES.with_borrow_mut(|v| v.clear());
    TL_PROCESSED_TILES.with_borrow_mut(|v| v.clear());
}

/// get_blank_tiles, used to return blank tiles stored in the tls (thread local storage)
/// ```
/// use rtile::*;
///
/// t!("@{tile1}-@{tile2}");
///
/// let result = get_blank_tiles();
/// assert_eq!(result.contains(&"tile1".to_string()), true);
/// assert_eq!(result.contains(&"tile2".to_string()), true);
/// ```
pub fn get_blank_tiles() -> HashSet<String> {
    let mut blank_tiles = HashSet::new();
    TL_RAW_TILES.with_borrow(|v| {
        for (tile_name, tile) in v.iter() {
            if tile.lns == Vec::<String>::new() {
                assert!(!tile_name.is_empty());
                blank_tiles.insert(tile_name.clone());
            }
        }
    });
    blank_tiles
}

fn trim<I, T>(t1: I, do_trimming: bool) -> Vec<String>
where
    I: IntoIterator<Item = T> + Debug,
    T: Into<String>,
{
    let t1: Vec<String> = t1.into_iter().map(Into::into).collect();
    if !do_trimming {
        return t1;
    }

    let mut lns: Vec<String> = t1.into_iter().map(|ln| ln.trim_end().to_string()).collect();
    lns = lns.into_iter().skip_while(|ln| ln.is_empty()).collect();
    lns.reverse();
    lns = lns.into_iter().skip_while(|ln| ln.is_empty()).collect();
    lns.reverse();
    let lf: Vec<&str> = lns
        .iter()
        .filter(|ln| !ln.is_empty())
        .map(|ln| ln.as_str())
        .collect();
    let left = if lf.is_empty() {
        0
    } else {
        lf.iter()
            .map(|ln| ln.len() - ln.trim_start().len())
            .min()
            .unwrap_or(0_usize)
    };
    let result = lns
        .into_iter()
        .map(|ln| ln.chars().skip(left).collect())
        .collect();
    result
}

fn append<I, T>(t1: &mut Vec<String>, t2: I)
where
    I: IntoIterator<Item = T> + Debug,
    T: Into<String>,
{
    let t2: Vec<String> = t2.into_iter().map(Into::into).collect();

    let diff: i32 = t2.len() as i32 - t1.len() as i32;
    if diff > 0 {
        t1.extend(vec!["".to_owned(); diff as usize]);
    }
    let w = t1.iter().map(|s| s.chars().count()).max().unwrap_or(0);
    for (i, s) in t2.into_iter().enumerate() {
        t1[i] = format!("{:<w$}{}", t1[i], s, w = w);
    }
}

fn get_inner_tile_name(ln: &str, current_cursor: &mut usize, end: &mut usize) -> Option<String> {
    find_next_inner_tile_name_and_maybe_do_append_the_inbetween_text(
        ln,
        current_cursor,
        end,
        &mut Vec::<String>::new(),
        false,
    )
}

fn find_next_inner_tile_name_and_do_append_the_inbetween_text(
    ln: &str,
    current_cursor: &mut usize,
    end: &mut usize,
    curr: &mut Vec<String>,
) -> Option<String> {
    find_next_inner_tile_name_and_maybe_do_append_the_inbetween_text(
        ln,
        current_cursor,
        end,
        curr,
        true,
    )
}

fn find_next_inner_tile_name_and_maybe_do_append_the_inbetween_text(
    ln: &str,
    current_cursor: &mut usize,
    end: &mut usize,
    curr: &mut Vec<String>,
    do_append: bool,
) -> Option<String> {
    let mut start = ln[*current_cursor..].find("@{").unwrap_or(ln.len());
    if *current_cursor == ln.len() && start == ln.len() && *end == ln.len() && !ln.is_empty() {
        return None;
    }
    if start < ln.len() {
        start += *current_cursor;
    }

    if do_append {
        append(curr, vec![&ln[*end..start]]);
    }

    if start == ln.len() {
        return None;
    }
    *end = ln[start..].find('}').map_or(0, |i| i + 1);
    if *end == 0 {
        panic!("unfinished @{{}} expression");
    }
    *end += start;
    *current_cursor = *end;
    let tile_name = ln[start + 2..*end - 1].to_string();
    Some(tile_name)
}

fn r_format_using_processed_tiles_data(s: &str) -> Vec<String> {
    let lns: Vec<&str> = s.split('\n').collect();
    let mut res = vec![];
    for ln in lns {
        let mut curr = vec![];
        let mut current_cursor = 0_usize;
        let mut end = 0;

        while let Some(tile_name) = find_next_inner_tile_name_and_do_append_the_inbetween_text(
            ln,
            &mut current_cursor,
            &mut end,
            &mut curr,
        ) {
            TL_PROCESSED_TILES.with_borrow(|v| {
                if v.contains_key(&tile_name) {
                    let tile_value = v.get(&tile_name).unwrap();
                    let lns: Vec<&str> = tile_value.split('\n').collect();
                    append(&mut curr, lns);
                } else {
                    println!("{} tile is not found", tile_name);
                }
            });
        }
        res.append(&mut curr);
    }
    res
}

fn r_format_using_raw_tiles_data(s: &str) -> Vec<String> {
    let lns: Vec<&str> = s.split('\n').collect();
    let mut res = vec![];
    for ln in lns {
        let mut curr = vec![];
        let mut current_cursor = 0_usize;
        let mut end = 0;

        while let Some(tile_name) = find_next_inner_tile_name_and_do_append_the_inbetween_text(
            ln,
            &mut current_cursor,
            &mut end,
            &mut curr,
        ) {
            TL_RAW_TILES.with_borrow(|v_raw| {
                if v_raw.contains_key(&tile_name) {
                    let tile_value = v_raw.get(&tile_name).unwrap();
                    check_for_recursion_of_tiles(&tile_name, tile_value);
                    process_all_required_tiles_data(&tile_name, tile_value);

                    TL_PROCESSED_TILES.with_borrow(|v| {
                        if v.contains_key(&tile_name) {
                            let tile_value = v.get(&tile_name).unwrap();
                            let lns: Vec<&str> = tile_value.split('\n').collect();
                            append(&mut curr, lns);
                        } else {
                            println!("{} tile is not found", tile_name);
                        }
                    });
                } else {
                    println!("{} tile is not found", tile_name);
                }
            });
        }
        res.append(&mut curr);
    }
    res
}

fn check_for_recursion_of_tiles(tile_name: &String, tile_value: &RTile) {
    let mut inner_tiles: Vec<String> = vec![];
    let mut processed_tiles: HashSet<String> = HashSet::new();
    let mut direct_parents: HashSet<String> = HashSet::new();
    direct_parents.insert(tile_name.clone());
    check_for_recursion_in_inner_tiles(
        tile_name,
        tile_value,
        &mut processed_tiles,
        &mut inner_tiles,
        &direct_parents,
    );
}

fn process_all_required_tiles_data(tile_name: &String, tile_value: &RTile) {
    let mut inner_tiles: Vec<String> = vec![tile_name.clone()];
    let mut processed_tiles: HashSet<String> = HashSet::new();

    find_inner_tiles(
        tile_name,
        tile_value,
        &mut processed_tiles,
        &mut inner_tiles,
    );

    if !inner_tiles.is_empty() {
        for inner_tile_index in (0..inner_tiles.len()).rev() {
            let inner_tile_name = inner_tiles.get(inner_tile_index).unwrap();

            let result = TL_RAW_TILES.with_borrow(|v| {
                if v.contains_key(inner_tile_name) {
                    let inner_tile_value = v.get(inner_tile_name).unwrap();
                    inner_tile_value.reevaluate()
                } else {
                    //tile not found, so return emtpy string
                    String::new()
                }
            });

            TL_PROCESSED_TILES.with_borrow_mut(|v| v.insert(inner_tile_name.clone(), result));
        }
    }
}

fn check_for_recursion_in_inner_tiles(
    tile_name: &String,
    tile_value: &RTile,
    processed_tiles: &mut HashSet<String>,
    inner_tiles: &mut Vec<String>,
    direct_parents: &HashSet<String>,
) {
    for ln in &tile_value.lns {
        let mut curr = vec![];
        let mut current_cursor = 0_usize;
        let mut end = 0;

        while let Some(inner_tile_name) = find_next_inner_tile_name_and_do_append_the_inbetween_text(
            ln,
            &mut current_cursor,
            &mut end,
            &mut curr,
        ) {
            if processed_tiles.contains(&inner_tile_name) {
                continue;
            } else {
                TL_RAW_TILES.with_borrow(|v| {
                    if v.contains_key(&inner_tile_name) {
                        if direct_parents.contains(&inner_tile_name) {
                            panic!("detected a recursion");
                        } else {
                            let inner_tile_value = v.get(&inner_tile_name).unwrap();
                            inner_tiles.push(inner_tile_name.clone());

                            let mut all_direct_parents = direct_parents.clone();
                            all_direct_parents.insert(inner_tile_name.clone());
                            check_for_recursion_in_inner_tiles(
                                &inner_tile_name,
                                inner_tile_value,
                                processed_tiles,
                                inner_tiles,
                                &all_direct_parents,
                            );
                        }
                    } else {
                        println!("{} tile is not found", inner_tile_name);
                    }
                });
            }
        }
        processed_tiles.insert(tile_name.to_string());
    }
}

fn find_inner_tiles(
    tile_name: &String,
    tile_value: &RTile,
    processed_tiles: &mut HashSet<String>,
    inner_tiles: &mut Vec<String>,
) {
    for ln in &tile_value.lns {
        let mut curr = vec![];
        let mut current_cursor = 0_usize;
        let mut end = 0;
        while let Some(inner_tile_name) = find_next_inner_tile_name_and_do_append_the_inbetween_text(
            ln,
            &mut current_cursor,
            &mut end,
            &mut curr,
        ) {
            if processed_tiles.contains(&inner_tile_name) {
                continue;
            } else {
                TL_RAW_TILES.with_borrow(|v| {
                    if v.contains_key(&inner_tile_name) {
                        let inner_tile_value = v.get(&inner_tile_name).unwrap();
                        inner_tiles.push(inner_tile_name.clone());

                        find_inner_tiles(
                            &inner_tile_name,
                            inner_tile_value,
                            processed_tiles,
                            inner_tiles,
                        );
                    } else {
                        println!("{} tile is not found", inner_tile_name);
                    }
                });
            }
        }
        processed_tiles.insert(tile_name.to_string());
    }
}

fn identify_any_missing_inner_tiles(
    tile_name: Option<String>,
    tile_lns: &Vec<String>,
    processed_tiles: &mut HashSet<String>,
    missing_inner_tiles: &mut HashSet<String>,
) {
    for ln in tile_lns {
        let mut curr = vec![];
        let mut current_cursor = 0_usize;
        let mut end = 0;
        while let Some(inner_tile_name) = find_next_inner_tile_name_and_do_append_the_inbetween_text(
            ln,
            &mut current_cursor,
            &mut end,
            &mut curr,
        ) {
            if processed_tiles.contains(&inner_tile_name) {
                continue;
            } else {
                TL_RAW_TILES.with_borrow(|v| {
                    if v.contains_key(&inner_tile_name) {
                        let inner_tile_value = v.get(&inner_tile_name).unwrap();

                        identify_any_missing_inner_tiles(
                            Some(inner_tile_name.clone()),
                            &inner_tile_value.lns,
                            processed_tiles,
                            missing_inner_tiles,
                        );
                    } else if missing_inner_tiles.contains(&inner_tile_name) {
                    } else {
                        missing_inner_tiles.insert(inner_tile_name.clone());
                    }
                });
            }
        }
        if tile_name.is_some() {
            processed_tiles.insert(tile_name.clone().unwrap());
        }
    }
}

fn get_blank_inner_tiles_names(
    tile_name: Option<String>,
    tile_lns: &Vec<String>,
    processed_tiles: &mut HashSet<String>,
    blank_inner_tiles: &mut Vec<String>,
) {
    for ln in tile_lns {
        let mut curr = vec![];
        let mut current_cursor = 0_usize;
        let mut end = 0;
        while let Some(inner_tile_name) = find_next_inner_tile_name_and_do_append_the_inbetween_text(
            ln,
            &mut current_cursor,
            &mut end,
            &mut curr,
        ) {
            if processed_tiles.contains(&inner_tile_name) {
                continue;
            } else {
                TL_RAW_TILES.with_borrow(|v| {
                    if v.contains_key(&inner_tile_name) {
                        let inner_tile_value = v.get(&inner_tile_name).unwrap();
                        if inner_tile_value.lns == Vec::<String>::new() {
                            blank_inner_tiles.push(inner_tile_name.clone());
                        }

                        get_blank_inner_tiles_names(
                            Some(inner_tile_name),
                            &inner_tile_value.lns,
                            processed_tiles,
                            blank_inner_tiles,
                        );
                    } else {
                        println!("{} tile is not found", inner_tile_name);
                    }
                });
            }
        }
        if tile_name.is_some() {
            processed_tiles.insert(tile_name.clone().unwrap());
        }
    }
}

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq)]
pub struct RTile {
    pub name: Option<String>,
    pub lns: Vec<String>,
    pub do_trimming: bool,
}

impl RTile {
    pub fn new_str(lns: Vec<&str>) -> Self {
        let lns: Vec<String> = lns.iter().map(|&item| item.to_string()).collect();
        create_blank_tiles_of_any_missing_inner_tiles(None, &lns);
        Self {
            name: None,
            lns: trim(lns, true),
            do_trimming: true,
        }
    }

    pub fn new(lns: Vec<String>) -> Self {
        create_blank_tiles_of_any_missing_inner_tiles(None, &lns);
        Self {
            name: None,
            lns: trim(lns, true),
            do_trimming: true,
        }
    }

    pub fn construct_from_str(val: &str) -> Self {
        let lns: Vec<&str> = val.split('\n').collect();
        let lns: Vec<String> = lns.iter().map(|&item| item.to_string()).collect();
        create_blank_tiles_of_any_missing_inner_tiles(None, &lns);
        Self {
            name: None,
            lns: trim(lns, true),
            do_trimming: true,
        }
    }

    pub fn new_without_trimming_str(lns: Vec<&str>) -> Self {
        let lns: Vec<String> = lns.iter().map(|&item| item.to_string()).collect();
        create_blank_tiles_of_any_missing_inner_tiles(None, &lns);
        Self {
            name: None,
            lns: trim(lns, false),
            do_trimming: false,
        }
    }

    pub fn new_without_trimming(lns: Vec<String>) -> Self {
        create_blank_tiles_of_any_missing_inner_tiles(None, &lns);
        Self {
            name: None,
            lns: trim(lns, false),
            do_trimming: false,
        }
    }

    pub fn from_str_without_trimming(val: &str) -> Self {
        let lns: Vec<&str> = val.split('\n').collect();
        let lns: Vec<String> = lns.iter().map(|&item| item.to_string()).collect();
        create_blank_tiles_of_any_missing_inner_tiles(None, &lns);
        Self {
            name: None,
            lns: trim(lns, false),
            do_trimming: false,
        }
    }

    pub fn get_names_of_blank_inner_tiles(&self) -> Vec<String> {
        let mut processed_tiles: HashSet<String> = HashSet::new();
        let mut blank_inner_tiles = vec![];
        get_blank_inner_tiles_names(
            self.name.clone(),
            &self.lns,
            &mut processed_tiles,
            &mut blank_inner_tiles,
        );
        blank_inner_tiles
    }

    pub fn reevaluate(&self) -> String {
        // calling r_format_using_processed_tiles_data, as all the inner tiles are supposed to be reevaluated / processed by now
        trim(
            r_format_using_processed_tiles_data(self.lns.join("\n").as_str()),
            self.do_trimming,
        )
        .join("\n")
    }

    pub fn join<T: Display + Debug>(&self, x: &Vec<T>, last: Option<RTile>) -> Self {
        let mut res = RTile::new(vec![]);
        for (idx, item) in x.iter().enumerate() {
            match item {
                i if type_name::<T>() == "&String" => {
                    res += RTile::construct_from_str(i.to_string().as_str())
                }
                i if type_name::<T>() == "&RTile" => {
                    let lns = i
                        .to_string()
                        .split('\n')
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|&item| item.to_string())
                        .collect();
                    res += RTile::new(lns);
                }
                i => {
                    res += RTile::construct_from_str(i.to_string().as_str());
                }
            };
            if idx < x.len() - 1 {
                res += self.clone();
            }
        }
        match last {
            Some(t) => res + t,
            None => res,
        }
    }

    pub fn vjoin<T: Display + Debug>(&self, x: &Vec<T>, inline: bool, last: Option<RTile>) -> Self {
        let last = last.unwrap_or_else(|| RTile::new(vec![]));
        let mut res = RTile::new(vec![]);
        for (idx, item) in x.iter().enumerate() {
            match item {
                i if type_name::<T>() == "&String" => {
                    res |= RTile::construct_from_str(i.to_string().as_str())
                        + if inline {
                            if idx < x.len() - 1 {
                                self.clone()
                            } else {
                                last.clone()
                            }
                        } else {
                            RTile::construct_from_str("")
                        }
                }
                i if type_name::<T>() == "&RTile" => {
                    let lns = i
                        .to_string()
                        .split('\n')
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|&item| item.to_string())
                        .collect();
                    res |= RTile::new(lns)
                        + if inline {
                            if idx < x.len() - 1 {
                                self.clone()
                            } else {
                                last.clone()
                            }
                        } else {
                            RTile::construct_from_str("")
                        }
                }
                i => {
                    res |= RTile::construct_from_str(i.to_string().as_str())
                        + if inline {
                            if idx < x.len() - 1 {
                                self.clone()
                            } else {
                                last.clone()
                            }
                        } else {
                            RTile::construct_from_str("")
                        }
                }
            };
            if !inline {
                let tile = if idx < x.len() - 1 {
                    self.clone()
                } else {
                    last.clone()
                };
                res |= tile;
            }
        }
        res
    }

    ///
    /// This function returns the trimmed raw data of the tile
    ///
    /// ex:
    /// let tile = t!("   @{another_tile_one}             @{another_tile_two}       ");
    /// calling tile.raw(); would return a trimmed, non-expanded raw data of the tile
    /// so tile.raw() == "@{another_tile_one}             @{another_tile_two}".to_string();
    pub fn raw(&self) -> String {
        trim(self.lns.clone(), self.do_trimming)
            .join("\n")
            .to_string()
    }

    pub fn has_inner_tiles_in_raw_data(&self) -> bool {
        for ln in &self.lns {
            let start = ln[..].find("@{").unwrap_or(ln.len());
            if start == ln.len() {
                continue;
            }
            let end = ln[start..].find('}').map_or(0, |i| i + 1);
            if end == 0 {
                panic!("unfinished @{{}} expression");
            }
            return true;
        }
        false
    }

    pub fn inner_tiles_in_raw_data(&self) -> Vec<Vec<String>> {
        let mut result = vec![];
        for ln in &self.lns {
            let mut tiles_on_line = vec![];
            let mut current_cursor = 0_usize;
            let mut end = 0;
            while let Some(tile_name) = get_inner_tile_name(ln, &mut current_cursor, &mut end) {
                tiles_on_line.push(tile_name.clone());
                current_cursor = end;
            }
            result.push(tiles_on_line);
        }
        result
    }

    pub fn inner_tiles(&self) -> HashSet<String> {
        let mut inner_tiles: Vec<String> = vec![];
        let mut processed_tiles: HashSet<String> = HashSet::new();

        find_inner_tiles(&String::new(), self, &mut processed_tiles, &mut inner_tiles);

        inner_tiles.into_iter().collect()
    }

    pub fn flatten(&self) -> String {
        self.lns
            .iter()
            .map(|item| item.trim())
            .collect::<Vec<&str>>()
            .join("")
    }

    pub fn dimensions(&self) -> (usize, usize) {
        let width = self
            .lns
            .iter()
            .map(|s| s.chars().count())
            .max()
            .unwrap_or(0);
        let height = self.lns.len();
        (width, height)
    }
}

fn create_blank_tiles_of_any_missing_inner_tiles(name: Option<String>, lns: &Vec<String>) {
    let mut processed_tiles: HashSet<String> = HashSet::new();
    let mut missing_inner_tiles: HashSet<String> = HashSet::new();
    identify_any_missing_inner_tiles(name, lns, &mut processed_tiles, &mut missing_inner_tiles);
    if !missing_inner_tiles.is_empty() {
        for missing_inner_tile_name in missing_inner_tiles {
            TL_RAW_TILES.with_borrow_mut(|v| {
                v.insert(
                    missing_inner_tile_name.clone(),
                    RTile {
                        name: Some(missing_inner_tile_name.clone()),
                        lns: vec![],
                        do_trimming: true,
                    },
                )
            });
            TL_PROCESSED_TILES
                .with_borrow_mut(|v| v.insert(missing_inner_tile_name.clone(), String::new()));
        }
    }
}

impl Add for RTile {
    type Output = Self;

    fn add(self, other: RTile) -> Self::Output {
        let mut lns = self.lns.clone();
        append(&mut lns, other.lns);

        create_blank_tiles_of_any_missing_inner_tiles(None, &lns);

        Self {
            name: None,
            lns,
            do_trimming: self.do_trimming,
        }
    }
}

impl AddAssign for RTile {
    fn add_assign(&mut self, other: Self) {
        append(&mut self.lns, other.lns);
    }
}

impl BitOr for RTile {
    type Output = Self;

    fn bitor(self, other: RTile) -> Self::Output {
        let lns = [&self.lns[..], &other.lns[..]].concat();

        create_blank_tiles_of_any_missing_inner_tiles(None, &lns);

        Self {
            name: None,
            lns,
            do_trimming: self.do_trimming,
        }
    }
}

impl BitOrAssign for RTile {
    fn bitor_assign(&mut self, other: Self) {
        self.lns = [&self.lns[..], &other.lns[..]].concat();
    }
}

impl Display for RTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.do_trimming {
            write!(
                f,
                "{}",
                trim(
                    r_format_using_raw_tiles_data(self.lns.join("\n").as_str()),
                    true,
                )
                .join("\n")
            )
        } else {
            write!(
                f,
                "{}",
                r_format_using_raw_tiles_data(self.lns.join("\n").as_str()).join("\n")
            )
        }
    }
}
