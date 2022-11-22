pub mod sms;
pub fn is_valid_iranian_national_code(n: &String) -> bool {
    let numbers = n.chars().filter_map(|i| i.to_digit(10)).collect::<Vec<u32>>();
    if n.len() != 10 || numbers.len() != 10 {
        return false;
    }
    let check = numbers[9];
    let sum = (0..9).map(|x| numbers[x] * (10 - x) as u32).sum::<u32>();
    let sum = sum % 11;
    if sum < 2 {
        check == sum
    } else {
        check + sum == 11
    }
}