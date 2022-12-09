pub fn is_armstrong_number(num: u32) -> bool {
    let mut number_of_digits: u32 = 0;
    let mut number = num;
    let mut stack: Vec<u32> = vec![];
    while number / 10 > 0 || number % 10 > 0 {
        number_of_digits += 1;
        stack.append(&mut vec![number % 10]);
        number = number / 10;
    }

    let mut number: u32 = 0;
    for item in stack {
        number += u32::pow(item, number_of_digits)
    }
    number == num
}
