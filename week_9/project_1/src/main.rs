use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("Drink categories.txt").expect("create failed");
    file.write_all("The categories of drinks under Nigerian Breweries Plc are as follows:\n".as_bytes()).expect("Write failed");
    file.write_all("\nLager:- \n33 Export \nDesperados \nGoldberg \nGulder \nHeineken \nStar\n".as_bytes()).expect("Write failed");
    file.write_all("\nStout:- \nLegend \nTurbo King \nWilliams\n".as_bytes()).expect("Write failed");
    file.write_all("\nNon-alcoholic:- \nMaltina \nAmstel Malta \nMalta Gold \nFayrouz".as_bytes()).expect("write failed");

    println!("Data has been written to file.");
}
