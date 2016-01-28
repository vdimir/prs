
extern crate prs;
use prs::*;

#[test]
fn or_test() {
    #[derive(Debug, PartialEq)]
    enum NumOrString<'a> {
        Num(i32),
        Str(&'a str),
        Space,
    }

    let num_parser = pred(|c: &char| c.is_numeric())
                         .greedy()
                         .map(|s: &str| NumOrString::Num(s.parse::<i32>().unwrap()));

    let uppercase_parser = pred(|c: &char| c.is_uppercase()).greedy().map(|s| NumOrString::Str(s));

    let space_parser = pred(|c: &char| c == &' ').greedy().map(|_| NumOrString::Space);

    let num_or_uppercase = num_parser.or(space_parser)
                                     .or(uppercase_parser);


    let test_list = &[("633XA", Some(NumOrString::Num(633)), "XA"),
                      ("XA5", Some(NumOrString::Str("XA")), "5"),
                      ("633", Some(NumOrString::Num(633)), ""),
                      (" 633", Some(NumOrString::Space), "633"),
                      ("   x ", Some(NumOrString::Space), "x "),
                      ("d5A", None, "d5A"),
                      ("6A33xa", Some(NumOrString::Num(6)), "A33xa"),
                      ("FOO", Some(NumOrString::Str("FOO")), "")];


    for t in test_list {
        let parsed = num_or_uppercase.parse(t.0);
        assert_eq!(parsed.res.ok(), t.1);
        assert_eq!(parsed.other, t.2);
    }
}

#[test]
fn and_test() {
    let num_parser = pred(|c: &char| c.is_numeric())
                         .greedy()
                         .map(|s: &str| (s.parse::<i32>().unwrap()));

    let uppercase_parser = pred(|c: &char| c.is_uppercase()).greedy();

    let num_or_uppercase = num_parser.and(uppercase_parser);

    let test_list = &[("633XA", Some((633, "XA")), ""),
                      ("5", None, "5"),
                      ("633X", Some((633, "X")), ""),
                      ("XA", None, "XA"),
                      ("500FFbar", Some((500, "FF")), "bar"),
                      ("d5A", None, "d5A")];


    for t in test_list {
        let parsed = num_or_uppercase.parse(t.0);
        assert_eq!(parsed.res.ok(), t.1);
        assert_eq!(parsed.other, t.2);
    }

}

#[test]
fn skip_test() {
    let num_parser = pred(|c: &char| c.is_numeric())
                         .greedy()
                         .map(|s: &str| s.parse::<i32>().unwrap());

    let uppercase_parser = pred(|c: &char| c.is_uppercase()).greedy();

    let space_parser = pred(|c: &char| c == &' ').greedy();

    let num_space_uppercase = num_parser.skip(space_parser)
                                        .and(uppercase_parser);

    let test_list = &[("633 XA", Some((633, "XA")), ""),
                      ("5", None, "5"),
                      ("633X", None, "633X"),
                      ("XA", None, "XA"),
                      ("500 FFbar", Some((500, "FF")), "bar")];


    for t in test_list {
        let parsed = num_space_uppercase.parse(t.0);
        assert_eq!(parsed.res.ok(), t.1);
        assert_eq!(parsed.other, t.2);
    }
}


#[test]
fn mabye_test() {
    let num_parser = pred(|c: &char| c.is_numeric())
                         .greedy()
                         .map(|s: &str| s.parse::<i32>().unwrap());

    let mabye_num = maybe(num_parser);

    let test_list = &[("633x", Some(Some(633)), "x"), ("X5", Some(None), "X5")];

    for t in test_list {
        let parsed = mabye_num.parse(t.0);
        assert_eq!(parsed.res.ok(), t.1);
        assert_eq!(parsed.other, t.2);
    }
}

#[test]
fn rep_test() {
    let num_parser = pred(|c: &char| c.is_numeric())
                         .greedy()
                         .map(|s: &str| s.parse::<i32>().unwrap());

    let space_parser = pred(|c: &char| c == &' ').greedy().map(|_| ());

    let list_of_nums_sum = rep(num_parser.skip(maybe(space_parser)))
                               .map(|x| x.iter().fold(0, |acc, &x| acc + x));

    let test_list = &[("5", Some(5), ""),
                      ("1 2 3 4 50  12 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1",
                       Some(100),
                       ""),
                      ("633X", Some(633), "X"),
                      (" 5XA", None, " 5XA"),
                      ("500 20  bar", Some(520), "bar")];

    for t in test_list {
        let parsed = list_of_nums_sum.parse(t.0);
        assert_eq!(parsed.res.ok(), t.1);
        assert_eq!(parsed.other, t.2);
    }
}

#[test]
fn num_parser_test() -> () {
    let num_parser = pred(|c: &char| c.is_numeric()).greedy();

    let num_parser_num = num_parser.map(|x: &str| x.parse::<i32>().unwrap());

    let test_list = &[("633xa", Some(633), "xa"),
                      ("-1", None, "-1"),
                      ("633", Some(633), ""),
                      ("a633xa", None, "a633xa"),
                      ("6_33xa", Some(6), "_33xa"),
                      ("s633a", None, "s633a")];

    for t in test_list {
        let parsed = num_parser_num.parse(t.0);
        assert_eq!(parsed.res.ok(), t.1);
        assert_eq!(parsed.other, t.2);
    }
}




// #[test]
// fn expr_test() {
//     enum Node {
//         Num(i32),
//         Add(Box<(Node, Node)>),
//         Sub(Box<(Node, Node)>),
//         Mul(Box<(Node, Node)>),
//         Div(Box<(Node, Node)>),
//     }

// let num = pred(|c: &char| c.is_numeric()).map(|s: &str| s.parse::<i32>().unwrap());

//     let add_symb = token('+');
//     let sub_symb = token('-');
//     let add_sub_symb = add_symb.or(sub_symb);
//     assert_eq!(add_sub_symb.parse("+-"), (Ok('+'), "-"));
// }

#[test]
fn simple_char_test() {
    let x_char = token('x');
    assert_eq!(x_char.parse("xxy").res, Ok('x'));

    // let x_char = token('x');
    let y_char = token('y');

    let xy = x_char
                .or(y_char.clone())
                .and(y_char);
    assert_eq!(xy.parse("yyx").res, Ok(('y','y')));

    let x_char = token('x').greedy();
    assert_eq!(x_char.parse("xxy").res, Ok("xx"));


}
