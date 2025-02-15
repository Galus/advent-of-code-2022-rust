mod readfile;
use readfile::readfile;

fn main() {
    let input = readfile();
    let len = input.len();
    println!("input: {:?}", &input[len - 20..]);
    let mut a: Vec<u32> = input
        .split("\n\n")
        .map(|x| {
            //println!("x: {:?}", x);
            x.split("\n")
                .map(|y| {
                    //println!("y: {:?}", y);
                    if !y.is_empty() {
                        y.parse::<u32>().unwrap()
                    } else {
                        return 0;
                    }
                })
                .sum::<u32>()
        })
        .collect();
    println!("a: {:?}", a);
    a.sort();
    a.reverse();
    a.truncate(3);
    println!("top 3: {:?}", a);
    let b = a.iter().sum::<u32>();
    println!("top 3 sum: {:?}", b);

    // let elves: Vec<&str> = input.split("\n\n").collect();
    // let elves_vecs: Vec<Vec<&str>> = elves.iter().map(|x| x.split("\n").collect()).collect();
    // println!("elves_vecs: {:?}", elves_vecs);
    // let elves_nums: Vec<Vec<u32>> = elves_vecs
    //     .iter()
    //     .map(|vec| vec.iter().map(|x| x.parse::<u32>().unwrap()).collect())
    //     .collect();

    // println!("elves_nums: {:?}", elves_nums);
    // let elftotals: Vec<u32> = elves
    //     .iter()
    //     .map(|x| {
    //         let listofstringnums: Vec<&str> = x.split("\n").collect();
    //         let nums: Vec<u32> = listofstringnums
    //             .iter()
    //             .map(|y| {
    //                 y.parse::<u32>().unwrap().collect();
    //             })
    //             .collect();
    //         x.iter().sum();
    //     })
    //     .collect();
    // println!("elftotals: {:?}", elftotals);
}
