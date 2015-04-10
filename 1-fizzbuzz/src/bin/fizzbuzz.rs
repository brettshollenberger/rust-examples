fn div_by_three(n: i64) -> bool {
    n % 3 == 0
}

fn div_by_five(n: i64) -> bool {
    n % 5 == 0
}

fn div_by_fifteen(n: i64) -> bool {
    n % 15 == 0
}

#[test]
fn test_one_not_div_by_three() {
    assert_eq!(div_by_three(1), false)
}

#[test]
fn test_six_div_by_three() {
    assert_eq!(div_by_three(6), true)
}

#[test]
fn test_three_not_div_by_five() {
    assert_eq!(div_by_five(3), false)
}

#[test]
fn test_ten_div_by_five() {
    assert_eq!(div_by_five(5), true)
}

#[test]
fn test_three_not_div_by_fifteen() {
    assert_eq!(div_by_fifteen(3), false)
}

#[test]
fn test_thirty_div_by_fifteen() {
    assert_eq!(div_by_fifteen(30), true)
}

fn main() {
    for num in (1i64..101) {
        if div_by_fifteen(num) {
            println!("fizzbuzz");
        } else if div_by_five(num) {
            println!("buzz");
        } else if div_by_three(num) {
            println!("fizz");
        } else {
            println!("{}", num);
        }
    }
}
