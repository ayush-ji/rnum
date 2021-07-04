#[allow(unused_imports)]
use super::*;

#[test]
#[should_panic]
fn check_logic_function() {
    let test_vec: Vec<String> = vec![String::from("hello")];

    assert_eq!(test_vec, logic::parse(vec![String::from("hello")]))
}

#[test]
fn check_first_item_to_be_i_or_f() {
    let test_vect: Vec<String> = vec![String::from("i"), String::from("f")];

    assert_eq!(test_vect.clone(), logic::parse(test_vect));
}

#[test]
fn check_for_failing_if_i_and_f_are_both_present() {
    let test_vect: Vec<String> = vec![
        String::from("rettreg"),
        String::from("f"),
        String::from("-l"),
        String::from("3"),
        String::from("-h"),
        String::from("4sds"),
    ];

    assert_eq!(test_vect.clone(), logic::parse(test_vect));
}
