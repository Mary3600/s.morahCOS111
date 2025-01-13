struct Laptops{
    cost:u32,
}
//formula to calculate cost of 3
impl Laptops{
    fn cost_of_3(&self,)->u32{
        self.cost * 3
    }
}
fn main() {
    let hp = Laptops{
        cost:650_000
    };
    let ibm = Laptops{
        cost:755_000
    };
    let toshiba = Laptops{
        cost:550_000
    };
    let dell = Laptops{
        cost:850_000
    };

    //total cost of buying 3 of each
    let total_cost = hp.cost_of_3() + ibm.cost_of_3() + toshiba.cost_of_3() + dell.cost_of_3();

    println!("Thecost of buying 3 of each laptop is ${}",total_cost);
}