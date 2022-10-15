use crate::parser::{ErrorReason, NumberError, Parser};
use crate::Version;

#[test]
fn two_component() {
    let p = Parser::from_slice("123.456".as_bytes());
    let version = p.parse().unwrap();

    assert_eq!(version, Version::new_core_version(123, 456))
}

#[test]
fn three_component() {
    let p = Parser::from_slice("123.456.789".as_bytes());
    let version = p.parse().unwrap();

    assert_eq!(version, Version::new_full_version(123, 456, 789))
}

#[yare::parameterized(
    zeros = {"10.101.100", (10, 101, 100) },
    simple = {"1.2.3", (1, 2, 3) },
    v1_point_oh = {"1.0.0", (1, 0, 0) }, 
    v2_point_oh = {"2.0.0", (2, 0, 0) },
)]
fn three_component_parse_ok_variations(input: &str, expected: (u64, u64, u64)) {
    let p = Parser::from_slice(input.as_bytes());
    let outcome = p.parse().unwrap();

    assert_eq!(outcome, Version::from(expected));
}

// Leading zero's are disallowed, unless the complete value consist singularly of the digit '0'
#[yare::parameterized(
    first = {"0123.456.789"},
    second = {"123.0456.789"},
    third = {"123.456.0789"}
)]
fn starts_with_zero(input: &str) {
    let p = Parser::from_slice(input.as_bytes());
    let result = p.parse();

    assert_eq!(
        result.unwrap_err().reason(),
        &ErrorReason::NumberError(NumberError::LeadingZero)
    );
}

#[yare::parameterized(
    t0 = {"0.456.789"},
    t2 = {"123.0.789"},
    t3 = {"123.456.0"},
    t01 = {"0.0.123"},
    t02 = {"0.123.0"},
    t12 = {"123.0.0"},
    t012 = {"0.0.0"},
    d0 = {"0.123"},
    d1 = {"123.0"},
    d01 = {"0.0"},
)]
fn has_zero_component(input: &str) {
    let p = Parser::from_slice(input.as_bytes());
    let result = p.parse();
    let value = result.unwrap();

    assert!(
        value.major() == 0
            || value.minor() == 0
            || (value.patch().is_some() && value.patch().unwrap() == 0)
    );
}

#[yare::parameterized(
    empty = { "" },
    one = { "1" },
    one_dot = { "1." },
    dot_v1 = { ".1.0.0" },
    v1_dot = { "1.0.0." },
    v1_commas = { "1,0,0" },
    v1_dot_with_spaces = { "1. 0. 0" },
    v1_dot_untrimemd_lhs = { " 1.0.0" },
    v1_dot_untrimemd_rhs = { "1.0.0 " },
    v1_double_dots = { "1..0.0 " },
    v1_dot_zero_dot_eoi = { "1.0. " },
    v1_zwsp = { "0.\u{200B}1.0" },
    range0 = { "^0.1.0" },
    range1 = { "~0.1.0" },
    range2 = { "=0.1.0" },
    overflow0 = { &format!("{}.1.2", u128::from(u64::MAX) + 1) },
    overflow1 = { &format!("1.{}.2", u128::from(u64::MAX) + 1) },
    overflow2 = { &format!("1.2.{}", u128::from(u64::MAX) + 1) },
    unexpected_input0 = { "1e.0.0" },
    unexpected_input1 = { "1.0p.0" },
    unexpected_input2 = { "1.0.j" },
)]
fn failure_variations(input: &str) {
    let p = Parser::from_slice(input.as_bytes());
    let result = p.parse();

    assert!(result.is_err());
}
