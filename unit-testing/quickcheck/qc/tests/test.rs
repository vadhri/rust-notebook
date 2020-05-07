#[cfg(test)]
#[macro_use]
extern crate quickcheck;
use qc::string_reverse;

quickcheck! {
      fn sr_test_qc(xs: String) -> bool {
          xs == string_reverse(string_reverse(xs.clone()))
      }
}

#[test]
pub fn sr_test() {
    let input = "1234";
    assert_eq!(string_reverse(input.to_string()), "4321");
}
