#[cfg(test)]
mod tests {
    use rtile::prelude::*;

    fn c_1(tile: &RTile) -> Vec<&str> {
        match tile.lns.len() {
            0 => vec![""],
            1 => vec!["└──", ""],
            n => {
                let mut v = vec!["├──"; n - 1];
                v.push("└──");
                v.push("");
                v
            }
        }
    }

    fn ct1(tile: &RTile) -> RTile {
        let tile_pre = k!(c_1(&tile));
        k!(tile_pre) + k!("    ") + tile.clone()
    }

    fn ct2(tile: &RTile) -> RTile {
        if tile.has_inner_tiles_in_raw_data() {
            let inner_tiles_in_raw_data = tile.inner_tiles_in_raw_data();
            let mut pre_vec = vec![];
            if inner_tiles_in_raw_data.len() == 1 {
                pre_vec = vec!["└──", ""];
            } else {
                for (index, inner_tiles) in inner_tiles_in_raw_data.iter().enumerate() {
                    if index < inner_tiles_in_raw_data.len() - 1 {
                        pre_vec.push("├──");
                        let mut max_len = 1;
                        if let Some(result) = inner_tiles.iter().max_by(|x, y| {
                            let mut x_len = 0;
                            let mut y_len = 0;
                            if let Some(inner_tile_val) = gtq!(x) {
                                x_len = t!(inner_tile_val).lns.len()
                            }
                            if let Some(inner_tile_val) = gtq!(y) {
                                y_len = t!(inner_tile_val).lns.len()
                            }
                            x_len.cmp(&y_len)
                        }) {
                            if let Some(inner_tile_val) = gtq!(result) {
                                max_len = t!(inner_tile_val).lns.len();
                                if max_len == 0 {
                                    max_len = 1;
                                }
                            }
                        }
                        for _ in 0..max_len - 1 {
                            pre_vec.push("│");
                        }
                    } else {
                        pre_vec.push("└──");
                    }
                }
            }
            let tile_pre = k!(pre_vec);
            k!(tile_pre) + k!("    ") + t!(tile.clone())
        } else {
            ct1(tile)
        }
    }

    #[test]
    fn test_if_tile_has_inner_tiles() {
        let t1 = t!(vec!["one", "two", "three"]);
        assert_eq!(t1.has_inner_tiles_in_raw_data(), false);

        let g1 = t!(r#"
        group 1
        @{t1}
        @{t2}
        @{t3}
        @{t4}
        "#);
        assert_eq!(g1.has_inner_tiles_in_raw_data(), true);
    }

    #[test]
    fn test_inner_tiles_in_raw_data_to_create_tree_view_of_data() {
        tp!(t1, ct2(&t!(vec!["one", "two", "three"])));

        tp!(t2, ct2(&t!(vec!["1", "2", "3", "4", "5"])));

        tp!(t3, ct2(&t!(vec!["a", "b", "c", "d", "e", "f", "g"])));

        tp!(
            t4,
            ct2(&t!(vec![
                "name 1",
                "name 2",
                "name 3",
                "name 4",
                " name 5",
                "  name 6",
                "     name 7"
            ]))
        );

        tp!(t5, ct2(&t!(vec!["folder1", "folder2", "folder4"])));

        tp!(
            t6,
            ct2(&t!(vec![
                "file 1",
                "file 2",
                "file  3",
                "  file 4",
                "   file  5"
            ]))
        );

        tp!(
            t7,
            ct2(&t!(vec![
                "txt file a",
                " txt file b",
                "txt file c",
                "txt file d",
                "     txt file e",
                "  txt file f",
                "txt file g"
            ]))
        );

        tp!(
            t8,
            ct2(&t!(vec![
                "directory 1",
                "directory 2",
                "   directory 3",
                "   directory 4",
            ]))
        );

        tp!(
            g1,
            ct2(&t!(r#"
            group 1
            @{t1}
            @{t2}
            @{t3}
            @{t4}
        "#))
        );

        tp!(
            g2,
            ct2(&t!(r#"
            group 2
            @{t5}
            @{t6}
            @{t7}
            @{t8}
        "#))
        );

        let all_groups = tp!(
            g_all,
            r#"
            1 and 2
            @{g1}
            hello
            world
            @{g2}
                                                @{t6}
                                                @{t7}
                                                @{t8}
        "#
        );

        let expected_result_one = ts!(r#"
                1 and 2
                ├──    group 1
                ├──    ├──    one
                │      ├──    two
                │      └──    three
                ├──    ├──    1
                │      ├──    2
                │      ├──    3
                │      ├──    4
                │      └──    5
                ├──    ├──    a
                │      ├──    b
                │      ├──    c
                │      ├──    d
                │      ├──    e
                │      ├──    f
                │      └──    g
                └──    ├──    name 1
                       ├──    name 2
                       ├──    name 3
                       ├──    name 4
                       ├──     name 5
                       ├──      name 6
                       └──         name 7
                hello
                world
                ├──    group 2
                ├──    ├──    folder1
                │      ├──    folder2
                │      └──    folder4
                ├──    ├──    file 1
                │      ├──    file 2
                │      ├──    file  3
                │      ├──      file 4
                │      └──       file  5
                ├──    ├──    txt file a
                │      ├──     txt file b
                │      ├──    txt file c
                │      ├──    txt file d
                │      ├──         txt file e
                │      ├──      txt file f
                │      └──    txt file g
                └──    ├──    directory 1
                       ├──    directory 2
                       ├──       directory 3
                       └──       directory 4
                                                    ├──    file 1
                                                    ├──    file 2
                                                    ├──    file  3
                                                    ├──      file 4
                                                    └──       file  5
                                                    ├──    txt file a
                                                    ├──     txt file b
                                                    ├──    txt file c
                                                    ├──    txt file d
                                                    ├──         txt file e
                                                    ├──      txt file f
                                                    └──    txt file g
                                                    ├──    directory 1
                                                    ├──    directory 2
                                                    ├──       directory 3
                                                    └──       directory 4
        "#);

        let expected_result_two = ts!(r#"
                ├──    1 and 2
                ├──    ├──    group 1
                │      ├──    ├──    one
                │      │      ├──    two
                │      │      └──    three
                │      ├──    ├──    1
                │      │      ├──    2
                │      │      ├──    3
                │      │      ├──    4
                │      │      └──    5
                │      ├──    ├──    a
                │      │      ├──    b
                │      │      ├──    c
                │      │      ├──    d
                │      │      ├──    e
                │      │      ├──    f
                │      │      └──    g
                │      └──    ├──    name 1
                │             ├──    name 2
                │             ├──    name 3
                │             ├──    name 4
                │             ├──     name 5
                │             ├──      name 6
                │             └──         name 7
                ├──    hello
                ├──    world
                ├──    ├──    group 2
                │      ├──    ├──    folder1
                │      │      ├──    folder2
                │      │      └──    folder4
                │      ├──    ├──    file 1
                │      │      ├──    file 2
                │      │      ├──    file  3
                │      │      ├──      file 4
                │      │      └──       file  5
                │      ├──    ├──    txt file a
                │      │      ├──     txt file b
                │      │      ├──    txt file c
                │      │      ├──    txt file d
                │      │      ├──         txt file e
                │      │      ├──      txt file f
                │      │      └──    txt file g
                │      └──    ├──    directory 1
                │             ├──    directory 2
                │             ├──       directory 3
                │             └──       directory 4
                ├──                                        ├──    file 1
                │                                          ├──    file 2
                │                                          ├──    file  3
                │                                          ├──      file 4
                │                                          └──       file  5
                ├──                                        ├──    txt file a
                │                                          ├──     txt file b
                │                                          ├──    txt file c
                │                                          ├──    txt file d
                │                                          ├──         txt file e
                │                                          ├──      txt file f
                │                                          └──    txt file g
                └──                                        ├──    directory 1
                                                           ├──    directory 2
                                                           ├──       directory 3
                                                           └──       directory 4
        "#);

        //println!("{}", all_groups);
        //println!("-----------------");
        //println!("{}", expected_result_one);
        assert_eq!(all_groups.to_string(), expected_result_one);
        assert_eq!(ts!(ct2(&all_groups)), expected_result_two);
    }
}
