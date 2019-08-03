use std::io;
use std::io::Read;

//https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&page=show_problem&problem=2354

fn main() -> io::Result<()> {
    let mut stringbuffer = String::new();

    io::stdin().read_to_string(&mut stringbuffer)?;
    let lines: Vec<&str> = stringbuffer.split('\n').collect();

    let mut it = lines.into_iter();
    let number_of_tests = it.next().unwrap().parse::<i32>().unwrap();
    for _i in 0..number_of_tests {
        //skip reading the number of prices
        it.next();
        handle_case(it.next().unwrap());
    }
    Ok(())
}

fn handle_case(line: &str) -> () {
    let mut prices: Vec<i32> = line.split_whitespace().map(|string| string.parse::<i32>().unwrap()).collect();
    prices.sort_unstable();
    prices.reverse();
    let total_discount :i32 =
        prices.into_iter().
        enumerate().
        filter(|(index, _price)| index % 3 == 2).
        map(|(_index, price)| price).
        sum();
    println!("{}",total_discount);
}
