mod lib;
use lib::TBMSort;

fn main() {
    let mut v: Vec<usize> =
        match std::env::args().skip(1).map(|a| a.parse()).collect() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("usage: tbmsort NATURALS...");
                eprintln!("abort for argument parse failure.");
                return;
            }
        };
    if v.len() == 0 {
        print_help();
        return;
    }
    v.tbmsort();
    println!(
        "{}",
        v.into_iter()
            .map(|x| format!("{x}"))
            .collect::<Vec<String>>()
            .join(" "),
    );
}

fn print_help() {
    eprintln!("usage:");
    eprintln!("    tbmsort NATURALS...");
    eprintln!();
    eprintln!("Takes a list of natural numbers and sorts them using ThreadedBogoMergeSort.");
    eprintln!("Whew, what a mouthful.");
    eprintln!();
    eprintln!("Fun fact: a sorting algorithm called BogoSort shuffles a list randomly until it");
    eprintln!("becomes sorted; it is O(n*n!) where n is the size of the sequence, and '!' is");
    eprintln!("the postfix unary operator for factorial. BogoSort is often cited as a slowest");
    eprintln!("sorting algorithm.");
    eprintln!();
    eprintln!("Fun fact 2: TBMSort is multithreaded AND *slower* than BogoSort.");
}
