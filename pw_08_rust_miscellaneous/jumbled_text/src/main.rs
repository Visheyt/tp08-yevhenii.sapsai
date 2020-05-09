use std::env;
const SEPARATORS: &str = " ,;:!?./%*$=+)@_-('\"&1234567890\r\n";

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 && args.len() < 3 {
        println!("{}", args[1]);
        println!("{}", mix(&args[1]));
    } else {
        println!("This is default text.");
        println!("{}",mix("This is default text."));
    }
}

fn mix(s: &str) -> String {
    let mut a: Vec<char> = s.chars().collect();

    for group in a.split_mut(|num| SEPARATORS.contains(*num)) {
        let len = group.len();

        if len > 3 {
            group[1..len - 1]
                .chunks_exact_mut(2)
                .for_each(|x| x.swap(0, 1));
        }
    }

    a.iter().collect()
}

// You are free to write other functions.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        assert_eq!(mix("According to a researcher at Cambridge University, \
         it doesn't matter in what order the letters in a word are, \
         the only important thing is that the first and last letters be at the right place. \
         The rest can be a total mess and you can still read it without problem. \
         This is because the human mind does not read every letter by itself but the word as a whole."),
         "Accroidng to a rseaecrehr at Cmarbdige Uinevsrtiy, \
         it deosn't mtaetr in waht odrer the lteetrs in a wrod are, \
         the olny ipmroatnt tihng is taht the frist and lsat lteetrs be at the rgiht palce. \
         The rset can be a ttoal mses and you can sitll raed it wtiohut porlbem. \
         Tihs is bceuase the hmuan mnid deos not raed eevry lteetr by istlef but the wrod as a wohle.")
    }
}
