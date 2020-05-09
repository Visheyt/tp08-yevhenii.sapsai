use structopt::StructOpt;

// You are free to define some constants.

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Numbers to Words Converter",
    about = "Convert an integer into letters."
)]
struct Arg {
    ///Number to convert to text u64(only positive)
    number: u64,
}

fn main() {
    let arg = Arg::from_args();
    println!("{}", to_letters(arg.number));
}

const ONES: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const TENS: [&str; 10] = [
    "zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];
const ORDERS: [&str; 7] = [
    "zero",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

pub fn to_letters(n: u64) -> String {
    match n {
        0..=19 => ONES[n as usize].to_string(),
        20..=99 => {
            let upper = (n / 10) as usize;
            match n % 10 {
                0 => TENS[upper].to_string(),
                lower => format!("{}-{}", TENS[upper], to_letters(lower)),
            }
        }

        100 | 200 | 300 | 400 | 500 | 600 | 700 | 800 | 900 => format_num(n, 100, "hundred"),
        101..=199
        | 201..=299
        | 301..=399
        | 401..=499
        | 501..=599
        | 601..=699
        | 701..=799
        | 801..=899
        | 901..=999 => format_num(n, 100, "hundred and"),
        _ => {
            let (div, order) = std::iter::successors(Some(1u64), |v| v.checked_mul(1000))
                .zip(ORDERS.iter())
                .find(|&(e, _)| e > n / 1000)
                .unwrap();

            format_num(n, div, order)
        }
    }
}
// You are free to define other functions.
fn format_num(num: u64, div: u64, order: &str) -> String {
    match (num / div, num % div) {
        (upper, 0) => format!("{} {}", to_letters(upper), order),
        (upper, lower) => format!("{} {} {}", to_letters(upper), order, to_letters(lower)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quintillions() {
        assert_eq!(to_letters(0), "zero");
        assert_eq!(to_letters(1), "one");
        assert_eq!(to_letters(2), "two");
        assert_eq!(to_letters(3), "three");
        assert_eq!(to_letters(4), "four");
        assert_eq!(to_letters(5), "five");
        assert_eq!(to_letters(6), "six");
        assert_eq!(to_letters(7), "seven");
        assert_eq!(to_letters(8), "eight");
        assert_eq!(to_letters(9), "nine");

        assert_eq!(to_letters(10), "ten");
        assert_eq!(to_letters(11), "eleven");
        assert_eq!(to_letters(12), "twelve");
        assert_eq!(to_letters(13), "thirteen");
        assert_eq!(to_letters(14), "fourteen");
        assert_eq!(to_letters(15), "fifteen");
        assert_eq!(to_letters(16), "sixteen");
        assert_eq!(to_letters(17), "seventeen");
        assert_eq!(to_letters(18), "eighteen");
        assert_eq!(to_letters(19), "nineteen");
        assert_eq!(to_letters(20), "twenty");
        assert_eq!(to_letters(21), "twenty-one");
        assert_eq!(to_letters(30), "thirty");
        assert_eq!(to_letters(32), "thirty-two");
        assert_eq!(to_letters(40), "forty");
        assert_eq!(to_letters(43), "forty-three");
        assert_eq!(to_letters(50), "fifty");
        assert_eq!(to_letters(54), "fifty-four");
        assert_eq!(to_letters(60), "sixty");
        assert_eq!(to_letters(65), "sixty-five");
        assert_eq!(to_letters(70), "seventy");
        assert_eq!(to_letters(76), "seventy-six");
        assert_eq!(to_letters(80), "eighty");
        assert_eq!(to_letters(87), "eighty-seven");
        assert_eq!(to_letters(90), "ninety");
        assert_eq!(to_letters(98), "ninety-eight");

        assert_eq!(to_letters(100), "one hundred");
        assert_eq!(to_letters(101), "one hundred and one");
        assert_eq!(to_letters(115), "one hundred and fifteen");
        assert_eq!(to_letters(165), "one hundred and sixty-five");
        assert_eq!(to_letters(200), "two hundred");
        assert_eq!(to_letters(277), "two hundred and seventy-seven");
        assert_eq!(to_letters(580), "five hundred and eighty");
        assert_eq!(to_letters(999), "nine hundred and ninety-nine");

        assert_eq!(to_letters(1_000), "one thousand");
        assert_eq!(
            to_letters(5_454),
            "five thousand four hundred and fifty-four"
        );
        assert_eq!(
            to_letters(9_999),
            "nine thousand nine hundred and ninety-nine"
        );

        assert_eq!(to_letters(100_002), "one hundred thousand two");
        assert_eq!(
            to_letters(200_100_003),
            "two hundred million one hundred thousand three"
        );

        assert_eq!(
            to_letters(18_446_744_073_709_551_615),
            "eighteen quintillion four hundred and forty-six \
             quadrillion seven hundred and forty-four trillion \
             seventy-three billion \
             seven hundred and nine million \
             five hundred and fifty-one thousand \
             six hundred and fifteen"
        );
    }
}
