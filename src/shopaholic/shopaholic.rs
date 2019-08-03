use std::io;
use std::io::Read;

//https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&page=show_problem&problem=2354

fn main() -> io::Result<()> {
    let mut stringbuffer = String::new();

    io::stdin().read_to_string(&mut stringbuffer)?;
    handle_string(&stringbuffer);
    Ok(())
}

fn handle_string(string: &str) {
    let lines: Vec<&str> = string.split('\n').collect();
    let mut it = lines.into_iter();
    let number_of_tests = it.next().unwrap().parse::<i32>().unwrap();
    for _i in 0..number_of_tests {
        //skip reading the number of prices
        it.next();
        let total_discount = handle_case(it.next().unwrap());
        println!("{}", total_discount);
    }
}

fn handle_case(line: &str) -> i32 {
    let mut prices: Vec<i32> = line.split_whitespace().map(|string| string.parse::<i32>().unwrap()).collect();
    prices.sort_unstable();
    prices.reverse();
    prices.into_iter().
        enumerate().
        filter(|(index, _price)| index % 3 == 2).
        map(|(_index, price)| price).
        sum::<i32>()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case_550() {
        let test_string = "400 100 200 350 300 250 500";
        assert_eq!(handle_case(test_string), 550);
    }
}