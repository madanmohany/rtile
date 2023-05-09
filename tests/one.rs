#[cfg(test)]
mod tests {
    use std::any::type_name;
    use std::collections::HashSet;

    // use rtile::gtp;
    // use rtile::gtq;
    // use rtile::k;
    // use rtile::kf;
    // use rtile::kk;
    // use rtile::kkp;
    // use rtile::kkq;
    // use rtile::kp;
    // use rtile::kq;
    // use rtile::ks;
    use rtile::*;
    // use rtile::sr;
    // use rtile::stp;
    // use rtile::stq;
    // use rtile::t;
    // use rtile::tf;
    // use rtile::tp;
    // use rtile::tq;
    // use rtile::ts;
    // use rtile::tt;
    // use rtile::ttp;
    // use rtile::ttq;

    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }
    #[test]
    fn test_type_of() {
        let t = t!("hello");
        assert_eq!(type_of(t), "rtile::RTile");
    }

    macro_rules! using_a {
        ($a:ident, $e:expr) => {{
            let $a = 42;
            $e
        }};
    }

    #[test]
    fn test_macro_example() {
        let four = using_a!(b, b / 10);
        assert_eq!(four, 4);
    }

    #[test]
    fn test_tile_literals() {
        let tile = t!("hello");
        assert_eq!(tile.to_string(), "hello");

        let tile = t!("world");
        assert_eq!(tile.to_string(), "world");

        let tile = t!(r#"
                    Hello,
                      world!
            "#);
        assert_eq!(tile.to_string(), "Hello,\n  world!");
    }

    #[test]
    fn test_tile_interpolation() {
        let h = tp!(ut2_h, "hello");
        assert_eq!(h.to_string(), "hello");
        let w = tp!(ut2_w, "world");
        assert_eq!(w.to_string(), "world");
        let s = r#"
                    @{ut2_h},
                      @{ut2_w}!
            "#;
        let tile = t!(s);
        assert_eq!(tile.to_string(), "hello,\n  world!");

        let w = tp!(
            ut2_w,
            r#"
                                    cruel
                                    world
            "#
        );
        assert_eq!(w.to_string(), "cruel\nworld");
        assert_eq!(tile.to_string(), "hello,\n  cruel!\n  world");
    }

    #[test]
    fn test_multilevel_tiles() {
        tp!(ut3_one, "zero");
        let _two = tp!(ut3_two, "@{ut3_one}");
        let _three = tp!(ut3_three, "@{ut3_two}");
        let _four = tp!(ut3_four, "@{ut3_three}");
        let _five = tp!(ut3_five, "@{ut3_four}");
        let _six = tp!(ut3_six, "@{ut3_five}");
        let _seven = tp!(ut3_seven, "@{ut3_six}");
        let _eight = tp!(ut3_eight, "@{ut3_seven}");
        let _nine = tp!(ut3_nine, "@{ut3_eight}");
        let _ten = tp!(ut3_ten, "@{ut3_nine}");
        let h = tp!(ut3_h, "@{ut3_ten}");
        assert_eq!(h.to_string(), "zero");
        let w = tp!(ut3_w, "world");
        assert_eq!(w.to_string(), "world");
        let s = r#"
                    @{ut3_h},
                      @{ut3_w}!
            "#;
        let tile = t!(s);
        assert_eq!(tile.to_string(), "zero,\n  world!");

        let w = tp!(
            ut3_w,
            r#"
                                    cruel
                                    world
            "#
        );
        assert_eq!(w.to_string(), "cruel\nworld");
        assert_eq!(tile.to_string(), "zero,\n  cruel!\n  world");
    }

    #[test]
    fn test_add_addassign() {
        let tile_one = t!(r#"
                Magenta
                Green
            "#);
        let tile_two = t!(r#"
                Red
                White
                Blue
            "#);
        let result = tile_one + tile_two;
        assert_eq!(result.to_string(), "MagentaRed\nGreen  White\n       Blue");

        let mut tile_one = t!(r#"
                Magenta
                Green
            "#);
        let tile_two = t!(r#"
                Red
                White
                Blue
            "#);
        tile_one += tile_two;
        assert_eq!(
            tile_one.to_string(),
            "MagentaRed\nGreen  White\n       Blue"
        );
    }

    #[test]
    fn test_or_orassign() {
        let tile_one = t!(r#"
                Magenta
                Green
            "#);
        let tile_two = t!(r#"
                Red
                White
                Blue
            "#);
        let result = tile_one | tile_two;
        assert_eq!(result.to_string(), "Magenta\nGreen\nRed\nWhite\nBlue");

        let mut tile_one = t!(r#"
                Magenta
                Green
            "#);
        let tile_two = t!(r#"
                Red
                White
                Blue
            "#);
        tile_one |= tile_two;
        assert_eq!(tile_one.to_string(), "Magenta\nGreen\nRed\nWhite\nBlue");
    }

    #[test]
    fn test_join_with_multilevel_tiles() {
        tp!(ut5_one, ";");
        let _two = tp!(ut5_two, "@{ut5_one}");
        let _three = tp!(ut5_three, "@{ut5_two}");
        let _four = tp!(ut5_four, "@{ut5_three}");
        let _five = tp!(ut5_five, "@{ut5_four}");
        let _six = tp!(ut5_six, "@{ut5_five}");
        let _seven = tp!(ut5_seven, "@{ut5_six}");
        let _eight = tp!(ut5_eight, "@{ut5_seven}");
        let _nine = tp!(ut5_nine, "@{ut5_eight}");
        let _ten = tp!(ut5_ten, "@{ut5_nine}");
        let h = t!("@{ut5_ten}");
        let tile = (t!(",")).join(&[t!("square"), t!("circle"), t!("triangle")].to_vec(), None);
        assert_eq!(tile.to_string(), "square,circle,triangle");

        tp!(ut5_one, "_");

        let tile = (t!(",")).join(
            &[t!("square"), t!("circle"), t!("triangle")].to_vec(),
            Some(h),
        );
        assert_eq!(tile.to_string(), "square,circle,triangle_");
    }

    #[test]
    fn test_join() {
        let tile = (t!(",")).join(&[t!("square"), t!("circle"), t!("triangle")].to_vec(), None);
        assert_eq!(tile.to_string(), "square,circle,triangle");

        let h = t!(";");
        let tile = (t!(",")).join(
            &[t!("square"), t!("circle"), t!("triangle")].to_vec(),
            Some(h),
        );
        assert_eq!(tile.to_string(), "square,circle,triangle;");
    }

    #[test]
    fn test_vjoin() {
        let tile = (t!(",")).vjoin(
            &[t!("square"), t!("circle"), t!("triangle")].to_vec(),
            true,
            None,
        );
        assert_eq!(tile.to_string(), "square,\ncircle,\ntriangle");

        let tile = (t!("----------")).vjoin(
            &[t!("square"), t!("circle"), t!("triangle")].to_vec(),
            false,
            None,
        );
        assert_eq!(
            tile.to_string(),
            "square\n----------\ncircle\n----------\ntriangle"
        );

        let h = t!(";");
        let tile = (t!(",")).vjoin(
            &[t!("square"), t!("circle"), t!("triangle")].to_vec(),
            true,
            Some(h),
        );
        assert_eq!(tile.to_string(), "square,\ncircle,\ntriangle;");

        let h = t!(";");
        let tile = (t!("----------")).vjoin(
            &[t!("square"), t!("circle"), t!("triangle")].to_vec(),
            false,
            Some(h),
        );
        assert_eq!(
            tile.to_string(),
            "square\n----------\ncircle\n----------\ntriangle\n;"
        );
    }

    #[test]
    fn test_macro_t_and_tp() {
        let tile = t!(vec!["   .  hello".to_string(), "  world".to_string()]);
        assert_eq!(tile.to_string(), " .  hello\nworld");
        //println!("{:#?}, \n{}", tile.lns, tile);

        let _fruits = tp!(
            macro_tandtp_one,
            vec!["Apple".to_string(), "Banana".to_string()]
        );
        let _vegetables = tp!(
            macro_tandtp_two,
            vec!["Brinjal".to_string(), "Carrot".to_string()]
        );
        let tile = t!(r#"
            Fruits            
            ------
            @{macro_tandtp_one}  .  
            
            Vegetables
            ----------
            @{macro_tandtp_two}  .
        "#);
        assert_eq!(
            tile.to_string(),
            "Fruits\n------\nApple   .\nBanana\n\nVegetables\n----------\nBrinjal  .\nCarrot"
        );

        // println!("{}", tile);
        // println!("{:#?}", tile.to_string());
    }

    #[test]
    fn test_macro_t_and_tq() {
        let tile = t!(vec!["   .  hello".to_string(), "  world".to_string()]);
        assert_eq!(tile.to_string(), " .  hello\nworld");
        //println!("{:#?}, \n{}", tile.lns, tile);

        let target_tile_name_one = "macro_tandtq_one";
        let target_tile_name_two = "macro_tandtq_two";
        let _fruits = tq!(
            target_tile_name_one,
            vec!["Apple".to_string(), "Banana".to_string()]
        );
        let _vegetables = tq!(
            target_tile_name_two,
            vec!["Brinjal".to_string(), "Carrot".to_string()]
        );
        let tile = t!(r#"
            Fruits            
            ------
            @{macro_tandtq_one}  .  
            
            Vegetables
            ----------
            @{macro_tandtq_two}  .
        "#);
        assert_eq!(
            tile.to_string(),
            "Fruits\n------\nApple   .\nBanana\n\nVegetables\n----------\nBrinjal  .\nCarrot"
        );

        // println!("{}", tile);
        // println!("{:#?}", tile.to_string());
    }

    #[test]
    fn test_multiple_tiles_on_a_line_one() {
        let _fruits = tp!(
            macro_mtpoal_one,
            vec!["Apple".to_string(), "Banana".to_string()]
        );
        let _vegetables = tp!(
            macro_mtpoal_two,
            vec!["Brinjal".to_string(), "Carrot".to_string()]
        );
        let tile = t!(r#"
            Fruits              Vegetables
            -------------------------------
            @{macro_mtpoal_one}    @{macro_mtpoal_two}  
        "#);

        assert_eq!(tile.to_string(), "Fruits              Vegetables\n-------------------------------\nApple     Brinjal\nBanana    Carrot");

        // println!("{:#?}", tile);
        // println!("{:#?}", tile.to_string());
    }

    #[test]
    fn test_multiple_tiles_on_a_line_two() {
        let _fruits = tp!(macro_mtpoal_2_one, "Apple\nBanana");
        let _vegetables = tp!(macro_mtpoal_2_two, "Brinjal\nCarrot");
        let tile = t!(r#"
            Fruits    Vegetables
            -----------------------
            @{macro_mtpoal_2_one}    @{macro_mtpoal_2_two}  <<<
        "#);
        assert_eq!(tile.to_string(), "Fruits    Vegetables\n-----------------------\nApple     Brinjal  <<<\nBanana    Carrot");

        //println!("{:#?}", tile);
        //println!("{}", tile);
        //println!("{:#?}", tile.to_string());
    }

    #[test]
    fn test_multiple_tiles_on_a_line_three() {
        let ln = "@{emp_names} $@{emp_salaries}";
        let expected_non_tile_data_in_the_input_line =
            vec!["".to_string(), " $".to_string(), "".to_string()];
        let expected_tiles_in_the_input_line =
            vec!["emp_names".to_string(), "emp_salaries".to_string()];

        // let ln = "  @{one}After0123456789oneAAAAA@{two}After0123456789twoBBBBB@{three}           .";
        // let expected_non_tile_data_in_the_input_line = vec![
        //     "  ".to_string(),
        //     "After0123456789oneAAAAA".to_string(),
        //     "After0123456789twoBBBBB".to_string(),
        //     "           .".to_string(),
        // ];
        // let expected_tiles_in_the_input_line =
        //     vec!["one".to_string(), "two".to_string(), "three".to_string()];

        //let ln = "  @{one}1@{two}  .";

        let mut non_tile_data_in_the_input_line = vec![];
        let mut tiles_in_the_input_line = vec![];
        let mut current_cursor = 0_usize;
        let mut end = 0;
        loop {
            let mut start = ln[current_cursor..].find("@{").unwrap_or_else(|| ln.len());
            if start < ln.len() {
                start += current_cursor;
            }
            //println!("{}, {}, {}, {}", current_cursor, start, end, ln.len());
            //println!("{:?}", vec![ln[end..start].to_string()]);
            non_tile_data_in_the_input_line.push(ln[end..start].to_string());
            if start == ln.len() {
                break;
            }
            end = ln[start..].find("}").map_or(0, |i| i + 1);
            if end == 0 {
                panic!("unifinished @{{}} expression");
            }
            end += start;
            let tile_name = &ln[start + 2..end - 1].to_string();
            //println!("{}", tile_name);
            tiles_in_the_input_line.push(tile_name.clone());
            current_cursor = end;
            //println!("{}, {}, {}, {}", current_cursor, start, end, ln.len());
        }

        assert_eq!(
            expected_non_tile_data_in_the_input_line,
            non_tile_data_in_the_input_line
        );
        assert_eq!(expected_tiles_in_the_input_line, tiles_in_the_input_line);
    }

    #[test]
    fn test_blank_and_empty_tiles() {
        let blank = t!();
        assert_eq!(blank.to_string(), "");

        let tile = t!("Hello") | blank | t!("world");
        assert_eq!(tile.to_string(), "Hello\nworld");

        let et = t!("");
        assert_eq!(et.to_string(), "");

        let tile = t!("Hello") | et | t!("world");
        assert_eq!(tile.to_string(), "Hello\n\nworld");
    }

    #[test]
    fn test_tiles_with_empty_or_blank_lines_in_between_one() {
        let mut tile = t!(r#"
         Magenta
         Green
         "#);

        let blank = t!();

        tile |= blank
            | t!(r#"
                        Red
                        Blue
                        White
                        "#);

        let expected_result = ts!(r#"
                                            Magenta
                                            Green
                                            Red
                                            Blue
                                            White
                                            "#);
        assert_eq!(tile.to_string(), expected_result);

        let mut tile = t!(r#"
         Magenta
         Green
         "#);

        let et = t!("");

        tile |= et
            | t!(r#"
                        Red
                        Blue
                        White
                        "#);

        let expected_result = ts!(r#"
                                            Magenta
                                            Green

                                            Red
                                            Blue
                                            White
                                            "#);
        assert_eq!(tile.to_string(), expected_result);
    }

    #[test]
    fn test_tiles_with_empty_or_blank_lines_in_between_two() {
        let et = tp!(ut11_et, "");
        assert_eq!(et.to_string(), "");
        let tile = t!(r#"
            hello

            @{ut11_et}

            world
        "#);
        assert_eq!(tile.to_string(), "hello\n\n\n\nworld");

        let blank = tp!(ut11_blank, "");
        assert_eq!(blank.to_string(), "");
        let tile = t!(r#"
            hello

            @{ut11_blank}

            world
        "#);
        assert_eq!(tile.to_string(), "hello\n\n\n\nworld");
    }

    #[test]
    fn test_s_macros() {
        let s = ts!();
        assert_eq!(s, "".to_string());

        let s = ts!("");
        assert_eq!(s, "".to_string());

        let s = ts!(r#"

                        123
                        456
                        
                        "#);
        assert_eq!(s, "123\n456".to_string());
    }

    #[test]
    fn test_example_1() {
        let _colors = tp!(
            tex_1_colors,
            r#"
           White
           Black
           Ultramarine
           Red
           Green
           Blue
           "#
        );

        let _shapes = tp!(
            tex_1_shapes,
            r#"
           Triangle
           Circle
           "#
        );

        let tile = t!(r#"
         Colors: @{tex_1_colors}     Shapes: @{tex_1_shapes}

         That's all, folks!
         "#);

        println!("{}", tile);

        assert_eq!(tile.to_string(), "Colors: White           Shapes: Triangle\n        Black                   Circle\n        Ultramarine\n        Red\n        Green\n        Blue\n\nThat's all, folks!");

        let expected_result_formatted = ts!(r#"
            Colors: White           Shapes: Triangle
                    Black                   Circle
                    Ultramarine
                    Red
                    Green
                    Blue

            That's all, folks!
            "#);
        assert_eq!(tile.to_string(), expected_result_formatted);
        //println!("{}", expected_result_formatted);
    }

    #[test]
    fn test_example_2() {
        tp!(tex2_name);
        let greet = tp!(
            tex2_greet,
            r#"
                print('Hello, @{tex2_name}!')
                print('Welcome!')   
            "#
        );
        tp!(tex2_name, "Alice");
        tp!(tex2_greet_alice, greet.to_string().as_str());
        tp!(tex2_name, "Bob");
        tp!(tex2_greet_bob, greet.to_string().as_str());
        let code = t!(r#"
                                import sys

                                @{tex2_greet_alice}
                                if 'also-greet-bob' in sys.argv:
                                    @{tex2_greet_bob}
        
                            "#);

        let expected_result_formatted = ts!(r#"

                        import sys

                        print('Hello, Alice!')
                        print('Welcome!')
                        if 'also-greet-bob' in sys.argv:
                            print('Hello, Bob!')
                            print('Welcome!')

                "#);

        assert_eq!(code.to_string(), expected_result_formatted);
    }

    #[test]
    fn test_example_3() {
        let arglist = [t!("foo"), t!("bar"), t!("baz")].to_vec();
        let typelist = [t!("int"), t!("char*"), t!("struct quux")].to_vec();

        let argtile = t!("").vjoin(&arglist, true, None);
        stp!(tex3_argtile, argtile);
        //println!("{:#?}", argtile.to_string());
        let typetile = t!(",").vjoin(&typelist, true, Some(t!(");")));
        stp!(tex3_typetile, typetile);
        //println!("{:#?}", typetile.to_string());

        let signature = t!("void frobnicate(@{tex3_argtile} @{tex3_typetile}");
        //println!("{:#?}", signature); //actual tile without expansion of inner tiles
        //println!("{:#?}", signature.to_string());
        //println!("{}", signature);

        let expected_result_formatted = ts!(r#"
            void frobnicate(foo int,
                            bar char*,
                            baz struct quux);
                "#);
        //println!("{}", expected_result_formatted);

        assert_eq!(signature.to_string(), expected_result_formatted);
    }

    #[test]
    fn test_gtp_macro() {
        tp!(gtp_name);
        tp!(
            gtp_numbers,
            r#"
                                one
                                two
                                three
                                four
                                five
                        "#
        );
        tp!(gtp_greet, "hello @{gtp_name} @{gtp_numbers}");

        //get the persisted tile by name
        let greet = gtp!(gtp_greet).unwrap();
        let name = gtp!(gtp_name).unwrap();
        if name.lns.is_empty() {
            tp!(gtp_name, "Alice");
        }

        let expected_result_formatted = ts!(r#"
                                                    hello Alice one
                                                                two
                                                                three
                                                                four
                                                                five
                                                    "#);
        assert_eq!(greet.to_string(), expected_result_formatted);

        //get the persisted tile by name again
        let get_greet_tile_again = gtp!(gtp_greet).unwrap();
        tp!(gtp_name, "Bob");

        let expected_result_formatted = ts!(r#"
                            hello Bob one
                                      two
                                      three
                                      four
                                      five
                "#);

        assert_eq!(get_greet_tile_again.to_string(), expected_result_formatted);
    }

    #[test]
    fn test_gtq_macro() {
        let t_gtq_name = "t_gtq_name_1";
        let t_gtq_numbers = "t_gtq_numbers_1";
        let t_gtq_greet = "t_gtq_greet_1";
        tq!(t_gtq_name);
        tq!(
            t_gtq_numbers,
            r#"
                                one
                                two
                                three
                                four
                                five
                        "#
        );
        tq!(t_gtq_greet, "hello @{t_gtq_name_1} @{t_gtq_numbers_1}");

        //get the persisted tile by name
        let greet = gtq!(t_gtq_greet).unwrap();
        let name = gtq!(t_gtq_name).unwrap();
        if name.lns.is_empty() {
            tq!(t_gtq_name, "Alice");
        }

        let expected_result_formatted = ts!(r#"
                                                    hello Alice one
                                                                two
                                                                three
                                                                four
                                                                five
                                                    "#);
        assert_eq!(greet.to_string(), expected_result_formatted);

        //get the persisted tile by name again
        let get_greet_tile_again = gtq!(t_gtq_greet).unwrap();
        tq!(t_gtq_name, "Bob");

        let expected_result_formatted = ts!(r#"
                            hello Bob one
                                      two
                                      three
                                      four
                                      five
                "#);

        assert_eq!(get_greet_tile_again.to_string(), expected_result_formatted);
    }

    #[test]
    fn test_payroll_example_one() {
        let company_name = "ACME Inc";
        let director_name = "Wile E. Coyote";
        let employees = vec![
            ("Alice", 3000),
            ("Bob", 2500),
            ("Carol", 2800),
            ("Dylan", 2900),
        ];

        tp!(tpe1_company, company_name);
        tp!(tpe1_payroll_header, "@{tpe1_company} - Payroll");

        let mut payroll_data = t!();
        for employee in employees {
            let emp_data = format!("{}   ${}", employee.0, employee.1);
            payroll_data |= t!(emp_data.as_str());
        }
        stp!(tpe1_payroll_data, payroll_data);

        tp!(tpe1_director, director_name);
        tp!(
            tpe1_payroll_footer,
            "Signature: .......... (@{tpe1_director})"
        );

        let payroll = ts!(r#"

                                    @{tpe1_payroll_header}
                                        @{tpe1_payroll_data}
                                    @{tpe1_payroll_footer}
                                    
                                "#);

        let expected_result = ts!(r#"

                        ACME Inc - Payroll
                            Alice   $3000
                            Bob   $2500
                            Carol   $2800
                            Dylan   $2900
                        Signature: .......... (Wile E. Coyote)
        
        "#);
        assert_eq!(payroll.to_string(), expected_result);
        // println!("{}", payroll);
    }

    #[test]
    fn test_payroll_example_two() {
        let company_name = "ACME Inc";
        let director_name = "Wile E. Coyote";
        let employees = vec![
            ("Alice", 3000),
            ("Bob", 2500),
            ("Carol", 2800),
            ("Dylan", 2900),
        ];

        tp!(tpe2_company, company_name);
        tp!(tpe2_payroll_header, "@{tpe2_company} - Payroll");

        let mut names = vec![];
        let mut salaries = vec![];
        for employee in employees {
            names.push(employee.0.clone());
            salaries.push(format!("{}", employee.1));
        }
        tp!(tpe2_emp_names, names.join("\n").as_str());
        tp!(tpe2_emp_salaries, salaries.join("\n").as_str());
        tp!(tpe2_dollar_sign, vec!["$"; names.len()].join("\n").as_str());
        tp!(
            tpe2_payroll_data,
            "@{tpe2_emp_names} @{tpe2_dollar_sign}@{tpe2_emp_salaries}"
        );
        tp!(tpe2_director, director_name);
        tp!(
            tpe2_payroll_footer,
            "Signature: .......... (@{tpe2_director})"
        );

        let payroll = ts!(r#"

                                    @{tpe2_payroll_header}
                                        @{tpe2_payroll_data}
                                    @{tpe2_payroll_footer}
                                    
                                "#);

        let expected_result = ts!(r#"

                        ACME Inc - Payroll
                            Alice $3000
                            Bob   $2500
                            Carol $2800
                            Dylan $2900
                        Signature: .......... (Wile E. Coyote)
        
        "#);
        assert_eq!(payroll.to_string(), expected_result);
        //println!("{}", payroll);
    }

    #[test]
    fn test_macros_tt() {
        let numbers_as_strings = "one, two, three, four, five, six, seven, eight, nine, ten";
        tp!(tmt1_numbers, numbers_as_strings);
        let t1 = tt!("introducing @{tmt1_numbers}");
        assert_eq!(
            t1.to_string(),
            "introducing one, two, three, four, five, six, seven, eight, nine, ten"
        );
        tp!(tmt1_numbers, "1, 2, 3, 4, 5, 6, 7, 8, 9, 10");
        let t2 = tt!("introducing @{tmt1_numbers}");
        assert_eq!(t2.to_string(), "introducing 1, 2, 3, 4, 5, 6, 7, 8, 9, 10");
    }

    #[test]
    fn test_macros_ttp() {
        let numbers_as_strings = "one, two, three, four, five, six, seven, eight, nine, ten";
        tp!(tmt2_numbers, numbers_as_strings);
        ttp!(tmt2_numbers_as_strings, "introducing @{tmt2_numbers}");
        tp!(tmt2_numbers, "1, 2, 3, 4, 5, 6, 7, 8, 9, 10");
        ttp!(tmt2_numbers_as_digits, "introducing @{tmt2_numbers}");
        let result = t!(r#"
            numbers as strings : @{tmt2_numbers_as_strings}
            numbers as digits : @{tmt2_numbers_as_digits}
            "#);
        assert_eq!(
            result.to_string(),
            ts!(r#"
                    numbers as strings : introducing one, two, three, four, five, six, seven, eight, nine, ten
                    numbers as digits : introducing 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
            "#)
        );
    }

    #[test]
    fn test_macros_ttq() {
        let numbers_as_strings = "one, two, three, four, five, six, seven, eight, nine, ten";
        let target_tq_name = "tmtq2_numbers";
        let target_ttq_name_one = "tmttq2_numbers_as_strings";
        let target_ttq_name_two = "tmttq2_numbers_as_digits";
        tq!(target_tq_name, numbers_as_strings);
        ttq!(target_ttq_name_one, "introducing @{tmtq2_numbers}");
        tq!(target_tq_name, "1, 2, 3, 4, 5, 6, 7, 8, 9, 10");
        ttq!(target_ttq_name_two, "introducing @{tmtq2_numbers}");
        let result = t!(r#"
            numbers as strings : @{tmttq2_numbers_as_strings}
            numbers as digits : @{tmttq2_numbers_as_digits}
            "#);
        assert_eq!(
            result.to_string(),
            ts!(r#"
                    numbers as strings : introducing one, two, three, four, five, six, seven, eight, nine, ten
                    numbers as digits : introducing 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
            "#)
        );
    }

    #[test]
    fn test_macros_kkp() {
        let numbers_as_strings = "one, two, three, four, five, six, seven, eight, nine, ten";
        tp!(tmt2_numbers, numbers_as_strings);
        kkp!(tmt2_numbers_as_strings, "introducing @{tmt2_numbers}\n");
        tp!(tmt2_numbers, "1, 2, 3, 4, 5, 6, 7, 8, 9, 10");
        kkp!(tmt2_numbers_as_digits, "introducing @{tmt2_numbers}   ");
        let result = t!(r#"
            numbers as strings : @{tmt2_numbers_as_strings}
            numbers as digits : @{tmt2_numbers_as_digits}
            "#);
        assert_eq!(
            result.to_string(),
            ts!(r#"
                    numbers as strings : introducing one, two, three, four, five, six, seven, eight, nine, ten

                    numbers as digits : introducing 1, 2, 3, 4, 5, 6, 7, 8, 9, 10   
            "#)
        );
    }

    #[test]
    fn test_macros_kkq() {
        let numbers_as_strings = "one, two, three, four, five, six, seven, eight, nine, ten";
        let target_tq_name = "tmtq2_numbers";
        let target_name_one = "tm2_numbers_as_strings";
        let target_name_two = "tm2_numbers_as_digits";
        tq!(target_tq_name, numbers_as_strings);
        kkq!(target_name_one, "introducing @{tmtq2_numbers}\n");
        tq!(target_tq_name, "1, 2, 3, 4, 5, 6, 7, 8, 9, 10");
        kkq!(target_name_two, "introducing @{tmtq2_numbers}   ");
        let result = t!(r#"
            numbers as strings : @{tm2_numbers_as_strings}
            numbers as digits : @{tm2_numbers_as_digits}
            "#);
        assert_eq!(
            result.to_string(),
            ts!(r#"
                    numbers as strings : introducing one, two, three, four, five, six, seven, eight, nine, ten

                    numbers as digits : introducing 1, 2, 3, 4, 5, 6, 7, 8, 9, 10   
            "#)
        );
    }

    #[test]
    fn test_raw_tile() {
        let tile = t!("   @{another_tile_one}             @{another_tile_two}       ");
        assert_eq!(
            tile.raw(),
            "@{another_tile_one}             @{another_tile_two}".to_string()
        );
        let tile = t!(r#"   
                         @{another_tile_one}             @{another_tile_two}     
                                 @{another_tile_three}- @{other_tiles}
                             0   1   2   3   4   5   6   7   8   9
                             ----------------------------------------
                     "#);

        let s_raw = sr!(tile);
        let result = "@{another_tile_one}             @{another_tile_two}\n        @{another_tile_three}- @{other_tiles}\n    0   1   2   3   4   5   6   7   8   9\n    ----------------------------------------".to_string();
        assert_eq!(tile.raw(), result);
        assert_eq!(s_raw, result);
    }

    #[test]
    fn test_inner_tiles_none() {
        let tile = t!("some tile without any inner tiles");
        assert_eq!(tile.inner_tiles(), HashSet::new());
    }

    #[test]
    fn test_inner_tiles_one() {
        tp!(it1_another_tile_one);
        tp!(it1_another_tile_two);
        tp!(it1_another_tile_three);
        tp!(it1_other_tiles);
        let tile = t!(r#"   
                         @{it1_another_tile_one}             @{it1_another_tile_two}     
                                 @{it1_another_tile_three}- @{it1_other_tiles}
                             0   1   2   3   4   5   6   7   8   9
                             ----------------------------------------
                     "#);

        let mut expected_result = HashSet::new();

        expected_result.insert("it1_another_tile_one".to_string());
        expected_result.insert("it1_another_tile_two".to_string());
        expected_result.insert("it1_another_tile_three".to_string());
        expected_result.insert("it1_other_tiles".to_string());

        assert_eq!(tile.inner_tiles(), expected_result);
    }

    #[test]
    fn test_inner_tiles_two() {
        tp!(it2_11);
        tp!(it2_12);
        tp!(it2_21);
        tp!(it2_22);
        tp!(it2_23);
        tp!(it2_31);
        tp!(it2_32);
        tp!(it2_33);
        tp!(it2_34);
        tp!(it2_other_1);
        tp!(it2_other_2);
        tp!(it2_other_3);
        tp!(it2_another_tile_one, "@{it2_11}@{it2_12}");
        tp!(it2_another_tile_two, "@{it2_21}@{it2_22}@{it2_23}");
        tp!(
            it2_another_tile_three,
            "@{it2_31}@{it2_32}@{it2_33}@{it2_34}"
        );
        tp!(
            it2_other_tiles,
            "@{it2_other_1}@{it2_other_1}@{it2_other_2}@{it2_other_2}@{it2_other_3}@{it2_other_3}"
        );
        let tile = t!(r#"   
                         @{it2_another_tile_one}             @{it2_another_tile_two}     
                                 @{it2_another_tile_three}- @{it2_other_tiles}
                             0   1   2   3   4   5   6   7   8   9
                             ----------------------------------------
                     "#);

        let mut expected_result = HashSet::new();
        expected_result.insert("it2_another_tile_one".to_string());
        expected_result.insert("it2_another_tile_two".to_string());
        expected_result.insert("it2_another_tile_three".to_string());
        expected_result.insert("it2_other_tiles".to_string());
        expected_result.insert("it2_11".to_string());
        expected_result.insert("it2_12".to_string());
        expected_result.insert("it2_21".to_string());
        expected_result.insert("it2_22".to_string());
        expected_result.insert("it2_23".to_string());
        expected_result.insert("it2_31".to_string());
        expected_result.insert("it2_32".to_string());
        expected_result.insert("it2_33".to_string());
        expected_result.insert("it2_34".to_string());
        expected_result.insert("it2_other_1".to_string());
        expected_result.insert("it2_other_2".to_string());
        expected_result.insert("it2_other_3".to_string());

        assert_eq!(tile.inner_tiles(), expected_result);
    }

    #[test]
    fn test_inner_tiles_three() {
        tp!(it3_11);
        tp!(it3_12);
        tp!(it3_another_tile_one, "@{it3_11}@{it3_12}");
        tp!(it3_another_tile_two, "no inner tiles");
        tp!(it3_another_tile_three, "no inner tiles");
        tp!(it3_other_tiles, "no inner tiles");
        tp!(it3_numbers, "0   1   2   3   4   5   6   7   8   9");
        tp!(it3_line, "----------------------------------------");
        let tile = t!(r#"   
                         @{it3_another_tile_one}             @{it3_another_tile_two}     
                                 @{it3_another_tile_three}- @{it3_other_tiles}
                             @{it3_numbers}
                             @{it3_line}
                     "#);

        let mut expected_result = HashSet::new();
        expected_result.insert("it3_another_tile_one".to_string());
        expected_result.insert("it3_another_tile_two".to_string());
        expected_result.insert("it3_another_tile_three".to_string());
        expected_result.insert("it3_other_tiles".to_string());
        expected_result.insert("it3_11".to_string());
        expected_result.insert("it3_12".to_string());
        expected_result.insert("it3_numbers".to_string());
        expected_result.insert("it3_line".to_string());

        assert_eq!(tile.inner_tiles(), expected_result);
    }

    #[test]
    fn test_create_blank_tiles_of_any_missing_inner_tiles_one() {
        //using s!
        let s1 = "@{bt1_tile_1}-@{bt1_tile_2}";
        let result = ts!(s1);
        assert_eq!(result, "-");

        let blank_tiles = get_blank_tiles();
        assert_eq!(blank_tiles.contains(&"bt1_tile_1".to_string()), true);
        assert_eq!(blank_tiles.contains(&"bt1_tile_2".to_string()), true);

        tp!(bt1_tile_1, "tile1");
        tp!(bt1_tile_2, "tile2");
        let result = ts!(s1);
        assert_eq!(result, "tile1-tile2");

        let blank_tiles = get_blank_tiles();
        assert_eq!(blank_tiles.contains(&"bt1_tile_1".to_string()), false);
        assert_eq!(blank_tiles.contains(&"bt1_tile_2".to_string()), false);

        remove_tile("bt1_tile_1");
        remove_tile("bt1_tile_2");

        //using t!
        let result = t!("@{bt1_tile_1}-@{bt1_tile_2}");
        assert_eq!(result.to_string(), "-");

        let blank_tiles = get_blank_tiles();
        assert_eq!(blank_tiles.contains(&"bt1_tile_1".to_string()), true);
        assert_eq!(blank_tiles.contains(&"bt1_tile_2".to_string()), true);

        tp!(bt1_tile_1, "tile1");
        tp!(bt1_tile_2, "tile2");
        assert_eq!(result.to_string(), "tile1-tile2");

        let blank_tiles = get_blank_tiles();
        assert_eq!(blank_tiles.contains(&"bt1_tile_1".to_string()), false);
        assert_eq!(blank_tiles.contains(&"bt1_tile_2".to_string()), false);

        remove_tile("bt1_tile_1");
        remove_tile("bt1_tile_2");

        //using t!
        let v1 = vec!["@{bt1_tile_1}-@{bt1_tile_2}".to_string()];
        let result = t!(v1);
        assert_eq!(result.to_string(), "-");

        let blank_tiles = get_blank_tiles();
        assert_eq!(blank_tiles.contains(&"bt1_tile_1".to_string()), true);
        assert_eq!(blank_tiles.contains(&"bt1_tile_2".to_string()), true);

        tp!(bt1_tile_1, "tile1");
        tp!(bt1_tile_2, "tile2");
        assert_eq!(result.to_string(), "tile1-tile2");

        let blank_tiles = get_blank_tiles();
        assert_eq!(blank_tiles.contains(&"bt1_tile_1".to_string()), false);
        assert_eq!(blank_tiles.contains(&"bt1_tile_2".to_string()), false);
    }

    #[test]
    fn test_create_blank_tiles_of_any_missing_inner_tiles_two() {
        //using s!
        let result = t!("@{bt2_12} @{bt2_34}");
        tp!(bt2_12, "@{bt2_tile_1}-@{bt2_tile_2}");
        tp!(bt2_34, "@{bt2_tile_3}-@{bt2_tile_4}");
        assert_eq!(result.to_string(), "- -");

        let blank_tiles = get_blank_tiles();
        assert_eq!(blank_tiles.contains(&"bt2_tile_1".to_string()), true);
        assert_eq!(blank_tiles.contains(&"bt2_tile_2".to_string()), true);
        assert_eq!(blank_tiles.contains(&"bt2_tile_3".to_string()), true);
        assert_eq!(blank_tiles.contains(&"bt2_tile_4".to_string()), true);

        tp!(bt2_tile_1, "tile1");
        tp!(bt2_tile_2, "tile2");
        tp!(bt2_tile_3, "tile3");
        tp!(bt2_tile_4, "tile4");
        assert_eq!(result.to_string(), "tile1-tile2 tile3-tile4");

        let blank_tiles = get_blank_tiles();
        assert_eq!(blank_tiles.contains(&"bt2_tile_1".to_string()), false);
        assert_eq!(blank_tiles.contains(&"bt2_tile_2".to_string()), false);
        assert_eq!(blank_tiles.contains(&"bt2_tile_3".to_string()), false);
        assert_eq!(blank_tiles.contains(&"bt2_tile_4".to_string()), false);

        remove_tile("bt2_tile_1");
        remove_tile("bt2_tile_2");
        remove_tile("bt2_tile_3");
        remove_tile("bt2_tile_4");
        remove_tile("bt2_12");
        remove_tile("bt2_34");

        //using t!
        let v1 = vec!["@{bt2_12} @{bt2_34}".to_string()];
        let v2 = vec!["@{bt2_tile_1}-@{bt2_tile_2}".to_string()];
        let v3 = vec!["@{bt2_tile_3}-@{bt2_tile_4}".to_string()];
        tp!(bt2_12, v2);
        tp!(bt2_34, v3);
        let result = t!(v1);
        assert_eq!(result.to_string(), "- -");

        let blank_tiles = get_blank_tiles();
        assert_eq!(blank_tiles.contains(&"bt2_tile_1".to_string()), true);
        assert_eq!(blank_tiles.contains(&"bt2_tile_2".to_string()), true);
        assert_eq!(blank_tiles.contains(&"bt2_tile_3".to_string()), true);
        assert_eq!(blank_tiles.contains(&"bt2_tile_4".to_string()), true);

        tp!(bt2_tile_1, "tile1");
        tp!(bt2_tile_2, "tile2");
        tp!(bt2_tile_3, "tile3");
        tp!(bt2_tile_4, "tile4");
        assert_eq!(result.to_string(), "tile1-tile2 tile3-tile4");

        let blank_tiles = get_blank_tiles();
        assert_eq!(blank_tiles.contains(&"bt2_tile_1".to_string()), false);
        assert_eq!(blank_tiles.contains(&"bt2_tile_2".to_string()), false);
        assert_eq!(blank_tiles.contains(&"bt2_tile_3".to_string()), false);
        assert_eq!(blank_tiles.contains(&"bt2_tile_4".to_string()), false);
    }

    #[test]
    fn test_clear_tiles() {
        let t1 = tp!(tct_1, "one");
        let t2 = tp!(tct_2, "two");
        assert_eq!(get_raw_tile("tct_1").unwrap(), t1);
        assert_eq!(get_raw_tile("tct_2").unwrap(), t2);
        clear_tiles();
        assert_eq!(get_raw_tile("tct_1"), None);
        assert_eq!(get_raw_tile("tct_2"), None);
    }

    #[test]
    #[should_panic(expected = "detected a recursion")]
    fn test_for_recursion_one() {
        let tile = tp!(tfr1, "-@{tfr1}-");
        println!("{}", tile);
        //tile.to_string();
    }

    #[test]
    #[should_panic(expected = "detected a recursion")]
    fn test_for_recursion_two() {
        let tile = tp!(tfr2_1, "@{tfr2_2}");
        tp!(tfr2_2, "@{tfr2_1}");
        println!("{}", tile);
    }

    #[test]
    #[should_panic(expected = "detected a recursion")]
    fn test_for_recursion_three() {
        tp!(tfr3_1, "@{tfr3_1_1}@{tfr3_1_2}@{tfr3_1_3}");
        tp!(
            tfr3_2,
            r#"
                    @{tfr3_2_1}
                    @{tfr3_2_2}
                    @{tfr3_2_3}
            "#
        );

        let tile = tp!(tfr3, ">@{tfr3_1} - @{tfr3_2}<");

        assert_eq!(tile.to_string(), "> - <");

        tp!(tfr3_1_1, "@{tfr3_2_1}");
        assert_eq!(tile.to_string(), "> - <");

        tp!(tfr3_2_1, "@{tfr3_1_1}");
        println!("{}", tile);
    }

    #[test]
    #[should_panic(expected = "detected a recursion")]
    fn test_for_recursion_four() {
        tp!(tfr4_1, "@{tfr4_1_1}@{tfr4_1_2}@{tfr4_1_3}");
        tp!(
            tfr4_2,
            r#"
                    @{tfr4_2_1}
                    @{tfr4_2_2}
                    @{tfr4_2_3}
            "#
        );

        let tile = tp!(tfr4, ">@{tfr4_1} - @{tfr4_2}<");

        assert_eq!(tile.to_string(), "> - <");

        tp!(tfr4_2_3, "@{tfr4_1}");
        assert_eq!(tile.to_string(), "> - <");

        tp!(tfr4_1_3, "@{tfr4_2_3}");

        //panics as well
        //tp!(tfr4_1_3, "@{tfr4_2}");

        println!("{}", tile);
    }

    #[test]
    fn test_for_blank_inner_tiles_one() {
        let tile = t!("@{tbit1_1}-@{tbit1_2}");
        assert_eq!(
            tile.get_names_of_blank_inner_tiles(),
            vec!["tbit1_1".to_string(), "tbit1_2".to_string()]
        );

        let tile = t!("@{tbit1}");
        assert_eq!(
            tile.get_names_of_blank_inner_tiles(),
            vec!["tbit1".to_string()]
        );

        let tile = tp!(tbit1, "not empty");
        assert_eq!(tile.get_names_of_blank_inner_tiles(), Vec::<String>::new());

        let tile = tp!(tbit1, "");
        assert_eq!(tile.get_names_of_blank_inner_tiles(), Vec::<String>::new());
    }

    #[test]
    fn test_for_blank_inner_tiles_two() {
        tp!(tbit2_1, "@{tbit2_1_1}@{tbit2_1_2}@{tbit2_1_3}");
        tp!(
            tbit2_2,
            r#"
                    @{tbit2_2_1}
                    @{tbit2_2_2}
                    @{tbit2_2_3}
            "#
        );

        let tile = tp!(tbit2, ">@{tbit2_1} - @{tbit2_2}<");
        //println!("{:#?}", tile.get_blank_tiles());
        assert_eq!(
            tile.get_names_of_blank_inner_tiles(),
            vec![
                "tbit2_1_1".to_string(),
                "tbit2_1_2".to_string(),
                "tbit2_1_3".to_string(),
                "tbit2_2_1".to_string(),
                "tbit2_2_2".to_string(),
                "tbit2_2_3".to_string()
            ]
        );
    }

    #[test]
    fn test_string_from_s_macro() {
        tp!(
            tft_inner_tile_s2,
            r#"
                    seven,
            "#
        );
        tp!(
            tft_inner_tile_s1,
            r#"
                    six,
                    @{tft_inner_tile_s2}
                    eight,
            "#
        );
        let input_tile = t!(r#"
                    one,
                    two,
                    three,
                    four,
                    five,
                    @{tft_inner_tile_s1}
                    nine,
                    ten
        "#);
        let output = ts!(input_tile);
        let expected_output =
            "one,\ntwo,\nthree,\nfour,\nfive,\nsix,\nseven,\neight,\nnine,\nten".to_string();
        assert_eq!(output, expected_output);
        assert_eq!(
            input_tile.to_string(),
            "one,\ntwo,\nthree,\nfour,\nfive,\nsix,\nseven,\neight,\nnine,\nten".to_string()
        );
        assert_eq!(
            input_tile.raw(),
            "one,\ntwo,\nthree,\nfour,\nfive,\n@{tft_inner_tile_s1}\nnine,\nten".to_string()
        );
    }

    #[test]
    fn test_process_tile_using_t_macro() {
        tp!(
            tft_inner_tile_2,
            r#"
                    seven,
            "#
        );
        tp!(
            tft_inner_tile_1,
            r#"
                    six,
                    @{tft_inner_tile_2}
                    eight,
            "#
        );
        let input_tile = t!(r#"
                    one,
                    two,
                    three,
                    four,
                    five,
                    @{tft_inner_tile_1}
                    nine,
                    ten
        "#);
        let output = t!(input_tile);
        let expected_output =
            t!("one,\ntwo,\nthree,\nfour,\nfive,\nsix,\nseven,\neight,\nnine,\nten");
        assert_eq!(output, expected_output);
        assert_eq!(
            input_tile.to_string(),
            "one,\ntwo,\nthree,\nfour,\nfive,\nsix,\nseven,\neight,\nnine,\nten".to_string()
        );
        assert_eq!(
            input_tile.raw(),
            "one,\ntwo,\nthree,\nfour,\nfive,\n@{tft_inner_tile_1}\nnine,\nten".to_string()
        );
    }

    #[test]
    fn test_flatten_tile_using_macro_one() {
        tp!(
            tft_inner_tile_m2,
            r#"
                    seven,
            "#
        );
        tp!(
            tft_inner_tile_m1,
            r#"
                    six,
                    @{tft_inner_tile_m2}
                    eight,
            "#
        );
        let input_tile = t!(r#"
                    one,
                    two,
                    three,
                    four,
                    five,
                    @{tft_inner_tile_m1}
                    nine,
                    ten
        "#);
        let output = tf!(input_tile);
        let expected_output = "one,two,three,four,five,six,seven,eight,nine,ten".to_string();
        assert_eq!(output, expected_output);
        assert_eq!(
            input_tile.to_string(),
            "one,\ntwo,\nthree,\nfour,\nfive,\nsix,\nseven,\neight,\nnine,\nten".to_string()
        );
        assert_eq!(
            input_tile.raw(),
            "one,\ntwo,\nthree,\nfour,\nfive,\n@{tft_inner_tile_m1}\nnine,\nten".to_string()
        );
    }

    #[test]
    fn test_flatten_tile_using_macro_two() {
        let input_tile = t!(r#"
                    1234567890
                        @
                     numbers
                        .
                       com
        "#);
        let output = tf!(input_tile);
        let expected_output = "1234567890@numbers.com".to_string();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_flatten_tile_raw_data_using_function() {
        tp!(
            tft_inner_tile_f4,
            r#"
                    seven,
            "#
        );
        tp!(
            tft_inner_tile_f3,
            r#"
                    six,
                    @{tft_inner_tile_f4}
                    eight,
            "#
        );
        let input_tile = t!(r#"
                    one,
                    two,
                    three,
                    four,
                    five,
                    @{tft_inner_tile_f3}
                    nine,
                    ten
        "#);
        let output = input_tile.flatten();
        let expected_output = "one,two,three,four,five,@{tft_inner_tile_f3}nine,ten".to_string();
        assert_eq!(output, expected_output);
        assert_eq!(
            input_tile.to_string(),
            "one,\ntwo,\nthree,\nfour,\nfive,\nsix,\nseven,\neight,\nnine,\nten".to_string()
        );
        assert_eq!(
            input_tile.raw(),
            "one,\ntwo,\nthree,\nfour,\nfive,\n@{tft_inner_tile_f3}\nnine,\nten".to_string()
        );
    }

    #[test]
    fn test_flatten_tile_data_using_function_one() {
        tp!(
            tft_inner_tile_f2,
            r#"
                    seven,
            "#
        );
        tp!(
            tft_inner_tile_f1,
            r#"
                    six,
                    @{tft_inner_tile_f2}
                    eight,
            "#
        );
        let input_tile = t!(r#"
                    one,
                    two,
                    three,
                    four,
                    five,
                    @{tft_inner_tile_f1}
                    nine,
                    ten
        "#);
        let output = t!(input_tile).flatten();
        let expected_output = "one,two,three,four,five,six,seven,eight,nine,ten".to_string();
        assert_eq!(output, expected_output);
        assert_eq!(
            input_tile.to_string(),
            "one,\ntwo,\nthree,\nfour,\nfive,\nsix,\nseven,\neight,\nnine,\nten".to_string()
        );
        assert_eq!(
            input_tile.raw(),
            "one,\ntwo,\nthree,\nfour,\nfive,\n@{tft_inner_tile_f1}\nnine,\nten".to_string()
        );
    }

    #[test]
    fn test_flatten_tile_data_using_function_two() {
        let input_tile = t!(r#"
                    1234567890
                        @
                     numbers
                        .
                       com
        "#);
        let output = input_tile.flatten();
        let expected_output = "1234567890@numbers.com".to_string();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_t_macro_with_rtile() {
        let input = t!("hello");
        let output = t!(input);
        assert_eq!(input, output);
        assert_eq!("hello", input.to_string().as_str());
        assert_eq!("hello", output.to_string().as_str());
    }

    #[test]
    fn test_t_macro_with_str() {
        let input = t!("hello");
        assert_eq!("hello", input.to_string().as_str());
    }

    #[test]
    fn test_t_macro_with_string() {
        let input = t!("hello".to_string());
        assert_eq!("hello", input.to_string().as_str());

        let input_string = "hello".to_string();
        let input = t!(input_string);
        assert_eq!("hello", input.to_string().as_str());
    }

    #[test]
    fn test_stq_macro_one() {
        let mut input_t = t!();
        for i in 1..=10 {
            let target_tile_name = format!("t_stq_m_1_{}", i);

            //1_one
            //2_two
            stq!(
                target_tile_name,
                t!(format!("{}{}", i.to_string(), number_to_words(i)))
            );
            tp!(t_stq_input_tile, target_tile_name);

            //@@{}{some_value} => @{some_value}
            let mi = t!("@@{}{@{t_stq_input_tile}}");
            input_t |= t!(mi);
        }

        assert_eq!(
            input_t.raw(),
            t!(r#"
                @{t_stq_m_1_1}
                @{t_stq_m_1_2}
                @{t_stq_m_1_3}
                @{t_stq_m_1_4}
                @{t_stq_m_1_5}
                @{t_stq_m_1_6}
                @{t_stq_m_1_7}
                @{t_stq_m_1_8}
                @{t_stq_m_1_9}
                @{t_stq_m_1_10}
        "#)
            .raw()
        );

        assert_eq!(
            input_t.to_string(),
            t!(r#"
                1_one
                2_two
                3_three
                4_four
                5_five
                6_six
                7_seven
                8_eight
                9_nine
                10_ten
        "#)
            .to_string()
        );
        // println!("{}", input_t.raw());
        // println!("{}", input_t);
    }

    #[test]
    fn test_tq_macro_one() {
        let target_tile_name = "t_tq_m_1";
        tq!(target_tile_name, "target_tile");
        let input_t = t!("@{t_tq_m_1}");
        assert_eq!("target_tile".to_string(), input_t.to_string());
        assert_eq!("@{t_tq_m_1}".to_string(), input_t.raw());
        //println!("{}", input_t);
    }

    #[test]
    fn test_t_with_format_arguments_one() {
        let input_tile_one = t!("test 1");
        let input_tile_two = t!("test 2");
        tp!(
            t_tfmt_m_1,
            t!(
                r#"
                            input1 : {}

                            input2 : {}
                "#,
                input_tile_one,
                input_tile_two
            )
        );
        let result = tp!(t_tfmt_m_2, r#"input values = @{t_tfmt_m_1}"#);
        let expected_result = ts!(r#"
                    input values = input1 : test 1
                    
                                   input2 : test 2
        "#);

        assert_eq!(result.to_string(), expected_result);

        //println!("{}", result);
    }

    #[test]
    fn test_macros_with_format_arguments_one() {
        let val = 5;

        let t1 = t!("{}", val);
        let t2 = tp!(tmwfa_one_1, "{}", val);
        let tile_name = "tmwfa_one_2";
        let t3 = tq!(tile_name, "{}", val);

        let s1 = ts!("{}", val);
        let ts2 = tp!(tmwfa_one_3, "{}", val);
        let s2 = ts!(ts2);
        let tile_name = "tmwfa_one_4";
        let ts3 = tq!(tile_name, "{}", val);
        let s3 = ts!(ts3);
        let s4 = sr!("{}", val);

        let tt1 = tt!("{}", val);
        let tt2 = ttp!(tmwfa_one_5, "{}", val);
        let tile_name = "tmwfa_one_6";
        let tt3 = ttq!(tile_name, "{}", val);

        let expected_result = ts!("5");
        assert_eq!(t1.to_string(), expected_result);
        assert_eq!(t2.to_string(), expected_result);
        assert_eq!(t3.to_string(), expected_result);

        assert_eq!(s1, expected_result);
        assert_eq!(s2, expected_result);
        assert_eq!(s3, expected_result);
        assert_eq!(s4, expected_result);

        assert_eq!(tt1.to_string(), expected_result);
        assert_eq!(tt2.to_string(), expected_result);
        assert_eq!(tt3.to_string(), expected_result);
    }

    #[test]
    fn test_with_unicode_character() {
        let price_one = "€ 111,- ";
        let price_two = "€ 222,- ";

        tp!(tmt_1, "{},", price_one.to_string());
        tp!(tmt_2, "{},", price_two.to_string());

        let pt = t!("({}, {})", price_one.to_string(), price_two.to_string());
        assert_eq!(pt.to_string(), "(€ 111,- , € 222,- )".to_string());
        assert_eq!(
            t!("@{tmt_1}@{tmt_1}@{tmt_1}@{tmt_1}@{tmt_1}").to_string(),
            "€ 111,- ,€ 111,- ,€ 111,- ,€ 111,- ,€ 111,- ,".to_string()
        );
        assert_eq!(
            t!("@{tmt_2}@{tmt_2}@{tmt_2}@{tmt_2}@{tmt_2}").to_string(),
            "€ 222,- ,€ 222,- ,€ 222,- ,€ 222,- ,€ 222,- ,".to_string()
        );

        let s1 = "abc";
        let s2 = "€€€";
        assert_eq!(s1.len(), 3);
        assert_eq!(s2.len(), 9);
        assert_eq!(s1.chars().count(), 3);
        assert_eq!(s2.chars().count(), 3);
    }

    #[test]
    fn test_ts_macro() {
        let val = t!("one");
        assert_eq!(ts!(val), "one");
    }

    #[test]
    fn test_ks_macro_one() {
        let val = k!("   one   ");
        assert_eq!(ks!(val), "   one   ");
    }
    
    #[test]
    fn test_ks_macro_two(){
        kp!(numbers, "   1, 2, 3   ");
        kp!(alphabets, "   a, b, c, d   ");
        let result = ks!("Numbers: [@{numbers}]
                          Alphabets: [@{alphabets}]");
        assert_eq!(result, "Numbers: [   1, 2, 3   ]\n                          Alphabets: [   a, b, c, d   ]");
    }

    #[test]
    fn test_tile_without_trim() {
        let k = k!("   one");
        assert_eq!(ks!(k), "   one");
        //println!("{}", k);
    }

    #[test]
    fn test_kp_macro() {
        let k1 = kp!(t_kp_one, " one     ");
        assert_eq!(ks!(k1), " one     ");
    }

    #[test]
    fn test_kq_macro() {
        let tile_variable_name = "t_kq_one";
        let k1 = kq!(tile_variable_name, " one     ");
        assert_eq!(ks!(k1), " one     ");
        let result = k!("@{t_kq_one}");
        assert_eq!(ks!(result), " one     ");
    }

    #[test]
    fn test_kk_macro() {
        let k1 = kp!(t_kk_one, " one     ");
        let k2 = kp!(t_kk_two, "     two ");
        let result = kk!("@{t_kk_one}\n@{t_kk_two}");
        assert_eq!(ks!(k1), " one     ");
        assert_eq!(ks!(k2), "     two ");
        assert_eq!(ks!(result), " one     \n     two ");
        //println!("{}", result);
    }

    #[test]
    fn test_t_macro() {
        let v1 = vec!["  one  ", "  two  ", "  three  "];
        let val = t!(v1);
        assert_eq!(ts!(val), "one\ntwo\nthree");
    }

    #[test]
    fn test_k_macro_one() {
        let v1 = vec!["  one  ", "  two  ", "  three  "];
        let val = k!(v1);
        assert_eq!(ks!(val), "  one  \n  two  \n  three  ");
    }

    #[test]
    fn test_k_macro_two() {
        let v1 = vec!["one ", " two", "three"];
        let val = k!(v1);
        assert_eq!(ks!(val), "one \n two\nthree");

        //println!("{}", val);
    }

    #[test]
    fn test_k_and_kp_macro_one() {
        kp!(t_mkkp_1_1, " abc ");
        kp!(t_mkkp_1_2, "@{t_mkkp_1_1}");

        let result_1 = k!("@{t_mkkp_1_2}");
        assert_eq!(ks!(result_1), " abc ");

        let result_1 = k!("@{t_mkkp_1_2}");
        kp!(t_mkkp_1_1, "  def  ");
        let result_2 = k!("@{t_mkkp_1_2}");

        assert_eq!(ks!(result_1), "  def  ");
        assert_eq!(ks!(result_2), "  def  ");
        //println!("{}", result_1);
        //println!("{}", result_2);
    }

    #[test]
    fn test_k_and_kq_macro_one() {
        let tile_name_one = "t_mkkq_1_1";
        let tile_name_two = "t_mkkq_1_2";
        kq!(tile_name_one, " abc ");
        kq!(tile_name_two, "@{t_mkkq_1_1}");
        let result_1 = k!("@{t_mkkq_1_2}");
        assert_eq!(ks!(result_1), " abc ");

        let result_1 = k!("@{t_mkkq_1_2}");
        kp!(t_mkkq_1_1, "  def  ");
        let result_2 = k!("@{t_mkkq_1_2}");

        assert_eq!(ks!(result_1), "  def  ");
        assert_eq!(ks!(result_2), "  def  ");
        //println!("{}", result_1);
        //println!("{}", result_2);
    }

    #[test]
    fn test_kk_macro_one() {
        kp!(t_mkk_1_1, " abc ");
        kp!(t_mkk_1_2, "@{t_mkk_1_1}");
        let result_tile = k!("@{t_mkk_1_2}");
        let result_1 = kk!(result_tile);
        kp!(t_mkk_1_1, "  def  ");
        let result_2 = k!(result_tile);

        assert_eq!(ks!(result_1), " abc ");
        assert_eq!(ks!(result_2), "  def  ");
    }

    #[test]
    fn test_kk_macro_two() {
        let tile_name_one = "t_mkk_2_1";
        let tile_name_two = "t_mkk_2_2";
        kq!(tile_name_one, " abc ");
        kq!(tile_name_two, format!("@{{{}}}", tile_name_one));
        //println!("{:#?}", tile_name_two);
        let result_tile = k!(format!("@{{{}}}", tile_name_two));
        //println!("{:#?}", result_tile);
        let result_1 = kk!(result_tile);
        kq!(tile_name_one, "  def  ");
        let result_2 = k!(result_tile);

        assert_eq!(ks!(result_1), " abc ");
        assert_eq!(ks!(result_2), "  def  ");
    }

    #[test]
    fn test_kf_macro_one() {
        let v1 = vec!["  one  ", "  two  ", "  three  "];
        let val = k!(v1);
        assert_eq!(kf!(val), "  one    two    three  ");
    }

    #[test]
    fn test_kf_macro_two() {
        let v1 = vec!["  one  ", "  two  ", "  three  "];
        let v2 = vec!["  1  ", "  2  ", "  3  "];
        let k1 = k!(v1);
        let k2 = k!(v2);
        let val = k1 + k2;

        assert_eq!(kf!(val), "  one      1    two      2    three    3  ");
    }

    #[test]
    fn test_tf_macro_two() {
        let v1 = vec!["  one  ", "  two  ", "  three  "];
        let v2 = vec!["  1  ", "  2  ", "  3  "];
        let k1 = k!(v1);
        let k2 = k!(v2);
        let val = k1 + k2;

        assert_eq!(tf!(val), "one      1two      2three    3");
    }

    #[test]
    fn test_combine_two_k_and_treat_the_result_as_t() {
        let v1 = vec!["  one  ", "  two  ", "  three  "];
        let v2 = vec!["  1  ", "  2  ", "  3  "];
        let k1 = k!(v1);
        let k2 = k!(v2);
        let val = k1 + k2;

        assert_eq!(ts!(val), "one      1\ntwo      2\nthree    3");
    }

    #[test]
    fn test_tile_with_spaces_one() {
        let k1 = k!(vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"
        ]);
        let k2 = k!(vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]);
        let spaces = vec![" "; 10];

        let result = k!(k1.clone() + k!(vec![spaces.join(""); 10]) + k2.clone());

        let expected_result = ts!(t!(r#"
        one            1
        two            2
        three          3
        four           4
        five           5
        six            6
        seven          7
        eight          8
        nine           9
        ten            10
        "#));

        assert_eq!(ts!(result), expected_result);
        assert_eq!(ks!(result), expected_result);

        println!("{}", ks!(k1 + k!(vec![spaces.join(""); 10]) + k2));
    }

    #[test]
    fn test_tile_with_spaces_two() {
        let _k1 = kp!(
            t_tws_2_1,
            vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"]
        );

        let _k2 = kp!(
            t_tws_2_2,
            vec!["1 ", "2", "3", "4", "5", "6", "7", "8", "9", "10"]
        );
        let spaces = vec![" "; 10];
        let _spaces_tile = kp!(t_tws_2_spaces, vec![spaces.join(""); 10]);

        let result = k!("@{t_tws_2_1}@{t_tws_2_spaces}@{t_tws_2_2}");
        let expected_result = ts!(t!(r#"
        one            1
        two            2
        three          3
        four           4
        five           5
        six            6
        seven          7
        eight          8
        nine           9
        ten            10
        "#));

        assert_eq!(ts!(result), expected_result);

        assert_eq!(ks!(result), "one            1 \ntwo            2\nthree          3\nfour           4\nfive           5\nsix            6\nseven          7\neight          8\nnine           9\nten            10".to_string());
    }

    #[test]
    fn test_inner_tiles_with_k_ts_and_ks_macros_one() {
        let _k1 = kp!(t_itwktsks_1_1, vec!["1 ", "2", "3"]);

        let result = k!("@{t_itwktsks_1_1}");

        assert_eq!(ts!(result), "1\n2\n3".to_string());
        assert_eq!(ks!(result), "1 \n2\n3".to_string());
    }

    #[test]
    fn test_t_macro_with_format_argument_one() {
        let result = t!("{:?}", vec![1, 2, 3, 4, 5]);
        assert_eq!(ts!(result), "[1, 2, 3, 4, 5]");
    }

    #[test]
    fn test_k_macro_with_format_argument_one() {
        let result = k!("{:?}", vec![1, 2, 3, 4, 5]);
        assert_eq!(ks!(result), "[1, 2, 3, 4, 5]");
    }

    #[test]
    fn test_dimensions() {
        let data = vec![
            "",
            "",
            "",
            "",
            "",
            "one            ",
            "two",
            "three",
            "four",
            "five",
            "six",
            "seven",
            "eight",
            "nine",
            "ten",
            "",
            "",
            "",
            "",
            "",
        ];
        let t = t!(data);
        let k = k!(data);

        let (t_width, t_height) = t.dimensions();
        assert_eq!(t_width, 5);
        assert_eq!(t_height, 10);
        let (k_width, k_height) = k.dimensions();
        assert_eq!(k_width, 15);
        assert_eq!(k_height, 20);
    }

    #[test]
    fn same_tile_one() {
        tp!(same_tile_name, "one");
        assert_eq!(
            "one".to_string(),
            gtq!("same_tile_name").unwrap().to_string()
        );
        assert_eq!("one".to_string(), gtp!(same_tile_name).unwrap().to_string());
    }

    #[test]
    fn same_tile_two() {
        tp!(same_tile_name, "two");
        assert_eq!("two".to_string(), gtp!(same_tile_name).unwrap().to_string());
    }

    #[test]
    fn same_tile_three() {
        tp!(same_tile_name, "three");
        assert_eq!(
            "three".to_string(),
            gtp!(same_tile_name).unwrap().to_string()
        );
    }

    #[test]
    fn same_tile_four() {
        tp!(same_tile_name, "four");
        assert_eq!(
            "four".to_string(),
            gtp!(same_tile_name).unwrap().to_string()
        );
    }

    #[test]
    fn same_tile_five() {
        tp!(same_tile_name, "five");
        assert_eq!(
            "five".to_string(),
            gtp!(same_tile_name).unwrap().to_string()
        );
    }

    #[test]
    fn same_tile_six() {
        tp!(same_tile_name, "six");
        assert_eq!("six".to_string(), gtp!(same_tile_name).unwrap().to_string());
    }

    #[test]
    fn same_tile_seven() {
        tp!(same_tile_name, "seven");
        assert_eq!(
            "seven".to_string(),
            gtp!(same_tile_name).unwrap().to_string()
        );
    }

    #[test]
    fn same_tile_eight() {
        tp!(same_tile_name, "eight");
        assert_eq!(
            "eight".to_string(),
            gtp!(same_tile_name).unwrap().to_string()
        );
    }

    #[test]
    fn same_tile_nine() {
        tp!(same_tile_name, "nine");
        assert_eq!(
            "nine".to_string(),
            gtp!(same_tile_name).unwrap().to_string()
        );
    }

    #[test]
    fn same_tile_ten() {
        tp!(same_tile_name, "ten");
        assert_eq!("ten".to_string(), gtp!(same_tile_name).unwrap().to_string());
    }

    fn inner_fn_one() {
        tp!(inner_tile_one, "one");
        inner_fn_two();
    }

    fn inner_fn_two() {
        tp!(inner_tile_two, "two");
        inner_fn_three();
    }

    fn inner_fn_three() {
        tp!(inner_tile_three, "three");
    }

    #[test]
    fn test_tiles_from_inner_fns() {
        inner_fn_one();
        let result = t!("@{inner_tile_one}, @{inner_tile_two}, @{inner_tile_three}");
        assert_eq!("one, two, three".to_string(), result.to_string());
    }

    #[test]
    fn test_ref_tile(){
        let tile = t!("some value");
        let result = ts!(&tile);
        assert_eq!(&result, "some value");
    }

    fn number_to_words(num: usize) -> String {
        if num == 0 {
            return "zero".to_string();
        }

        let mut words = String::new();

        if num >= 1000 {
            let thousands = num / 1000;
            words += &format!("{}_thousand", number_to_words(thousands));
        }

        let hundreds = (num % 1000) / 100;
        if hundreds > 0 {
            words += &format!("{}_hundred", number_to_words(hundreds));
        }

        let tens = (num % 100) / 10;
        let ones = num % 10;

        match tens {
            0 => (),
            1 => match ones {
                0 => words += "_ten",
                1 => words += "_eleven",
                2 => words += "_twelve",
                3 => words += "_thirteen",
                4 => words += "_fourteen",
                5 => words += "_fifteen",
                6 => words += "_sixteen",
                7 => words += "_seventeen",
                8 => words += "_eighteen",
                9 => words += "_nineteen",
                _ => (),
            },
            2 => words += "_twenty",
            3 => words += "_thirty",
            4 => words += "_forty",
            5 => words += "_fifty",
            6 => words += "_sixty",
            7 => words += "_seventy",
            8 => words += "_eighty",
            9 => words += "_ninety",
            _ => (),
        }

        if tens != 1 && ones > 0 {
            match ones {
                1 => words += "_one",
                2 => words += "_two",
                3 => words += "_three",
                4 => words += "_four",
                5 => words += "_five",
                6 => words += "_six",
                7 => words += "_seven",
                8 => words += "_eight",
                9 => words += "_nine",
                _ => (),
            }
        }

        words.trim_end().to_string()
    }
}
