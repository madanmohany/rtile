// Copyright (c) 2023 Madan Mohan Yenuganti  All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom
// the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

#![feature(local_key_cell_methods)]

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
use std::panic::RefUnwindSafe;

#[macro_export]
macro_rules! stp {
    ($i: ident, $t: expr) => {{
        set_tiles(format!("{}", stringify!($i)), $t.to_string());
        set_raw_tiles(format!("{}", stringify!($i)), $t.clone());
    }};
}

#[macro_export]
macro_rules! stq {
    ($e: expr, $t: expr) => {{
        set_tiles(format!("{}", $e), $t.to_string());
        set_raw_tiles(format!("{}", $e), $t.clone());
    }};
}

#[macro_export]
macro_rules! gtp {
    ($i: ident) => {{
        get_raw_tile(&stringify!($i).to_string())
    }};
}

#[macro_export]
macro_rules! gtq {
    ($e: expr) => {{
        let target_tile_name = format!("{}", $e);
        get_raw_tile(&target_tile_name)
    }};
}

///
/// Tiles with trimming
///
/// t - trim white spaces - do_trimming: true
///

pub trait MacroAttributeForT {
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

#[macro_export]
macro_rules! sr {
    ($e:expr) => {{
        t!($e).raw()
    }};
    ($($arg:tt)*) => {{
        let val = format!($($arg)*);
        t!(val).raw()
    }};
}

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

///
/// Tiles without trimming
///
/// k - keep white spaces - do_trimming: false
///

pub trait MacroAttributeForK {
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

#[macro_export]
macro_rules! kf {
    ($t: expr) => {{
        $t.to_string().split("\n").collect::<Vec<&str>>().join("")
    }};
}

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

pub fn set_tiles(key: String, value: String) {
    TL_PROCESSED_TILES.with_borrow_mut(|v| v.insert(key, value));
}

pub fn set_raw_tiles(key: String, value: RTile) {
    TL_RAW_TILES.with_borrow_mut(|v| v.insert(key, value));
}

pub fn get_raw_tile(key: &str) -> Option<RTile> {
    let key = &key.to_string();

    TL_RAW_TILES.with_borrow(|v| {
        if v.contains_key(key) {
            Some(v.get(key).unwrap().clone())
        } else {
            //panic!("no tile exists with name {}", key);
            None
        }
    })
}

pub fn remove_tile(key: &str) {
    let key = &key.to_string();
    TL_RAW_TILES.with_borrow_mut(|v| v.remove(key));
    TL_PROCESSED_TILES.with_borrow_mut(|v| v.remove(key));
}

pub fn clear_tiles() {
    TL_RAW_TILES.with_borrow_mut(|v| v.clear());
    TL_PROCESSED_TILES.with_borrow_mut(|v| v.clear());
}

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

    let mut lns: Vec<String> = t1.iter().map(|ln| ln.trim_end().to_string()).collect();
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
        .iter()
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
    for (i, s) in t2.iter().enumerate() {
        t1[i] = format!("{:<w$}{}", t1[i], s, w = w);
    }
}

fn r_format_using_processed_tiles_data(s: &str) -> Vec<String> {
    let lns: Vec<&str> = s.split('\n').collect();
    let mut res = vec![];
    for ln in lns {
        let mut curr = vec![];
        let mut current_cursor = 0_usize;
        let mut end = 0;
        loop {
            let mut start = ln[current_cursor..].find("@{").unwrap_or(ln.len());
            if current_cursor == ln.len() && start == ln.len() && end == ln.len() && !ln.is_empty()
            {
                break;
            }
            if start < ln.len() {
                start += current_cursor;
            }
            append(&mut curr, vec![ln[end..start].to_string()]);
            if start == ln.len() {
                break;
            }
            end = ln[start..].find('}').map_or(0, |i| i + 1);
            if end == 0 {
                panic!("unifinished @{{}} expression");
            }
            end += start;
            let tile_name = &ln[start + 2..end - 1].to_string();
            current_cursor = end;

            TL_PROCESSED_TILES.with_borrow(|v| {
                if v.contains_key(tile_name) {
                    let tile_value = v.get(tile_name).unwrap().clone();
                    let lns: Vec<&str> = tile_value.split('\n').collect();
                    let lns: Vec<String> = lns.iter().map(|&item| item.to_string()).collect();
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
        loop {
            let mut start = ln[current_cursor..].find("@{").unwrap_or(ln.len());
            if current_cursor == ln.len() && start == ln.len() && end == ln.len() && !ln.is_empty()
            {
                break;
            }
            if start < ln.len() {
                start += current_cursor;
            }
            append(&mut curr, vec![ln[end..start].to_string()]);
            if start == ln.len() {
                break;
            }
            end = ln[start..].find('}').map_or(0, |i| i + 1);
            if end == 0 {
                panic!("unifinished @{{}} expression");
            }
            end += start;
            let tile_name = &ln[start + 2..end - 1].to_string();
            current_cursor = end;

            TL_RAW_TILES.with_borrow(|v_raw| {
                if v_raw.contains_key(tile_name) {
                    let tile_value = v_raw.get(tile_name).unwrap().clone();
                    check_for_recursion_of_tiles(tile_name, &tile_value);
                    process_all_required_tiles_data(tile_name, &tile_value);

                    TL_PROCESSED_TILES.with_borrow(|v| {
                        if v.contains_key(tile_name) {
                            let tile_value = v.get(tile_name).unwrap().clone();
                            let lns: Vec<&str> = tile_value.split('\n').collect();
                            let lns: Vec<String> =
                                lns.iter().map(|&item| item.to_string()).collect();
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

            let inner_tile_value = TL_RAW_TILES.with_borrow(|v| {
                if v.contains_key(inner_tile_name) {
                    v.get(inner_tile_name).unwrap().clone()
                } else {
                    // panic!("{} tile is not found", inner_tile_name);
                    println!("{} tile is not found", inner_tile_name);
                    RTile::new(vec![])
                }
            });
            let result = inner_tile_value.reevaluate();
            TL_PROCESSED_TILES
                .with_borrow_mut(|v| v.insert(inner_tile_name.clone(), result.clone()));
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
        loop {
            let mut start = ln[current_cursor..].find("@{").unwrap_or(ln.len());
            if current_cursor == ln.len() && start == ln.len() && end == ln.len() && !ln.is_empty()
            {
                break;
            }
            if start < ln.len() {
                start += current_cursor;
            }
            append(&mut curr, vec![ln[end..start].to_string()]);
            if start == ln.len() {
                break;
            }
            end = ln[start..].find('}').map_or(0, |i| i + 1);
            if end == 0 {
                panic!("unifinished @{{}} expression");
            }
            end += start;
            current_cursor = end;

            let inner_tile_name = &ln[start + 2..end - 1].to_string();
            if processed_tiles.contains(inner_tile_name) {
                continue;
            } else {
                TL_RAW_TILES.with_borrow(|v| {
                    if v.contains_key(inner_tile_name) {
                        if direct_parents.contains(inner_tile_name) {
                            panic!("detected a recursion");
                        } else {
                            let inner_tile_value = v.get(inner_tile_name).unwrap().clone();
                            inner_tiles.push(inner_tile_name.clone());

                            let mut all_direct_parents = direct_parents.clone();
                            all_direct_parents.insert(inner_tile_name.clone());
                            check_for_recursion_in_inner_tiles(
                                inner_tile_name,
                                &inner_tile_value,
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
        loop {
            let mut start = ln[current_cursor..].find("@{").unwrap_or(ln.len());
            if current_cursor == ln.len() && start == ln.len() && end == ln.len() && !ln.is_empty()
            {
                break;
            }
            if start < ln.len() {
                start += current_cursor;
            }
            append(&mut curr, vec![ln[end..start].to_string()]);
            if start == ln.len() {
                break;
            }
            end = ln[start..].find('}').map_or(0, |i| i + 1);
            if end == 0 {
                panic!("unifinished @{{}} expression");
            }
            end += start;
            current_cursor = end;

            let inner_tile_name = &ln[start + 2..end - 1].to_string();
            if processed_tiles.contains(inner_tile_name) {
                continue;
            } else {
                TL_RAW_TILES.with_borrow(|v| {
                    if v.contains_key(inner_tile_name) {
                        let inner_tile_value = v.get(inner_tile_name).unwrap().clone();
                        inner_tiles.push(inner_tile_name.clone());

                        find_inner_tiles(
                            inner_tile_name,
                            &inner_tile_value,
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
        loop {
            let mut start = ln[current_cursor..].find("@{").unwrap_or(ln.len());
            if current_cursor == ln.len() && start == ln.len() && end == ln.len() && !ln.is_empty()
            {
                break;
            }
            if start < ln.len() {
                start += current_cursor;
            }
            append(&mut curr, vec![ln[end..start].to_string()]);
            if start == ln.len() {
                break;
            }
            end = ln[start..].find('}').map_or(0, |i| i + 1);
            if end == 0 {
                panic!("unifinished @{{}} expression");
            }
            end += start;
            current_cursor = end;

            let inner_tile_name = &ln[start + 2..end - 1].to_string();
            if processed_tiles.contains(inner_tile_name) {
                continue;
            } else {
                TL_RAW_TILES.with_borrow(|v| {
                    if v.contains_key(inner_tile_name) {
                        let inner_tile_value = v.get(inner_tile_name).unwrap().clone();

                        identify_any_missing_inner_tiles(
                            Some(inner_tile_name.clone()),
                            &inner_tile_value.lns,
                            processed_tiles,
                            missing_inner_tiles,
                        );
                    } else if missing_inner_tiles.contains(inner_tile_name) {
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
        loop {
            let mut start = ln[current_cursor..].find("@{").unwrap_or(ln.len());
            if current_cursor == ln.len() && start == ln.len() && end == ln.len() && !ln.is_empty()
            {
                break;
            }
            if start < ln.len() {
                start += current_cursor;
            }
            append(&mut curr, vec![ln[end..start].to_string()]);
            if start == ln.len() {
                break;
            }
            end = ln[start..].find('}').map_or(0, |i| i + 1);
            if end == 0 {
                panic!("unifinished @{{}} expression");
            }
            end += start;
            current_cursor = end;

            let inner_tile_name = &ln[start + 2..end - 1].to_string();
            if processed_tiles.contains(inner_tile_name) {
                continue;
            } else {
                TL_RAW_TILES.with_borrow(|v| {
                    if v.contains_key(inner_tile_name) {
                        let inner_tile_value = v.get(inner_tile_name).unwrap().clone();
                        if inner_tile_value.lns == Vec::<String>::new() {
                            blank_inner_tiles.push(inner_tile_name.clone());
                        }

                        get_blank_inner_tiles_names(
                            Some(inner_tile_name.clone()),
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

    pub fn join<T: Display + RefUnwindSafe + Debug>(
        &self,
        x: &Vec<T>,
        last: Option<RTile>,
    ) -> Self {
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

    pub fn vjoin<T: Display + RefUnwindSafe + Debug>(
        &self,
        x: &Vec<T>,
        inline: bool,
        last: Option<RTile>,
    ) -> Self {
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
        format!("{}", trim(self.lns.clone(), self.do_trimming).join("\n"))
    }

    pub fn has_inner_tiles_in_raw_data(&self) -> bool {
        for ln in &self.lns {
            let start = ln[..].find("@{").unwrap_or(ln.len());

            if start == ln.len() {
                continue;
            }
            let end = ln[start..].find('}').map_or(0, |i| i + 1);
            if end == 0 {
                panic!("unifinished @{{}} expression");
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
            loop {
                let mut start = ln[current_cursor..].find("@{").unwrap_or(ln.len());
                if current_cursor == ln.len()
                    && start == ln.len()
                    && end == ln.len()
                    && !ln.is_empty()
                {
                    break;
                }
                if start < ln.len() {
                    start += current_cursor;
                }
                if start == ln.len() {
                    break;
                }
                end = ln[start..].find('}').map_or(0, |i| i + 1);
                if end == 0 {
                    panic!("unifinished @{{}} expression");
                }
                end += start;
                let tile_name = &ln[start + 2..end - 1].to_string();
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
        //the order of write_guards should be RAW_TILES first and then PROCESSED_TILES,
        //if it is the other way around, then all unit tests cannot be run at a time.
        //The reason for this is because identify_any_missing_inner_tiles or similar functions which are
        //processing/scanning inner tiles uses the read_guard of RAW_TILES.
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
        //Self { lns: [&self.lns[..], &other.lns[..]].concat(), }
        //or
        //Self { lns: [self.lns.borrow(), other.lns.borrow()].concat::<String>(), }
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
        //self.lns = [&self.lns[..], &other.lns[..]].concat();
        //or
        //self.lns = [self.lns.borrow(), other.lns.borrow()].concat::<String>();

        self.lns = [&self.lns[..], &other.lns[..]].concat();
    }
}

impl Display for RTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.do_trimming {
            // trim and format
            //
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
            // just format - using only format causes problem with some unit tests.
            // ex., using just format causes failure in unit test 'test_example_3' as it does not trim the white spaces
            //
            write!(
                f,
                "{}",
                r_format_using_raw_tiles_data(self.lns.join("\n").as_str()).join("\n")
            )
        }

        // actual tile without expansion of inner tiles ( raw() function exists for this case )
        //
        // write!(
        //     f,
        //     "{}",
        //     self.lns.join("\n")
        // )
    }
}
