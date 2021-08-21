use budgetteer::budget_optimize;



pub mod budgetteer;

fn main() {

    let mut keybs : Vec<u64> = Vec::new();
    let mut drivers : Vec<u64> = Vec::new();

    keybs.push(30);
    keybs.push(60);
    keybs.push(22);
    keybs.push(55);


    drivers.push(12);
    drivers.push(33);
    drivers.push(65);


    if let Ok(budget) = budget_optimize(72, keybs, drivers){
        println!("{}", budget );
    } else  {
        println!("-1");
    }
}
