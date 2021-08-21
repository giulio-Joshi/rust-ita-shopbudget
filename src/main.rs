use budgetteer::budget_optimize;


pub mod budgetteer;

fn main() {

    // TODO stdin input

    let keybs = vec![30, 60,22, 55 ];
    let drivers : Vec<u64> = vec![12,33,65];

    if let Ok(budget) = budget_optimize(72, keybs, drivers){
        println!("{}", budget );
    } else  {
        println!("-1");
    }
}
