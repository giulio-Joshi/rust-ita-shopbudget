
///
/// Optimize selection of items from two lists of prices, based on initial budget balue.
///
///
pub fn budget_optimize(budget: u64, mut keyboards: Vec<u64>, mut memory: Vec<u64>) -> Result<u64, &'static str > {

    if budget < 1 {
        return Err( "Budget too low to purchase anything");
    }

    keyboards.sort_unstable();
    memory.sort_unstable();

    println!("Sorted keyboards: {:?}", keyboards);
    println!("Sorted memory: {:?}" , memory);

    let mut remaining = budget;
    let mut selected_keyboards : Vec<u64> = Vec::new();
    let mut selected_memory : Vec<u64> = Vec::new();

    while !keyboards.is_empty() && !memory.is_empty() {

        // Remove the minimum price from other list to allow at least one element in
        let mut affordable = remaining;
        if selected_memory.is_empty() {
            if let Some(minimum) = memory.iter().min() {
                affordable -= minimum;
            }
        }

        keyboards = filter_out_of_budget( affordable, keyboards);
        println!("After removing too pricy keyb: {:?}", keyboards);
        
        if let Some(k) = keyboards.pop(){
            remaining-= k;
            selected_keyboards.push(k);
        }

        memory = filter_out_of_budget(remaining, memory);
        println!("After removing too pricy drivers: {:?}", memory);

        if let Some(d) = memory.pop(){
            remaining-= d;
            selected_memory.push(d);
        }
    }

    println!("Selected keyboards: {:?}", selected_keyboards);
    println!("Selected memory: {:?}" , selected_memory);

    if !selected_keyboards.is_empty() && !selected_memory.is_empty() {
        Ok( budget-remaining)
    } else {
        Err("Failed, did could not select one item each")
    }
}

fn filter_out_of_budget(budget: u64, list: Vec<u64> ) -> Vec<u64> {

    list.into_iter()
        .filter(|x| *x <= budget && *x > 0 )
        .collect()

}

#[cfg(test)]
mod tests {

    use std::ops::Add;
    use super::budget_optimize;


    #[test]
    fn exact_budget() {

        let keybs = vec![10,30,20];
        let memory = vec![5,10,15];

        let mut expected = keybs.iter().fold(0,|acc,elem|{acc.add(elem)});
        expected+=  memory.iter().fold(0, |acc,elem|{acc.add(elem)});
        let result = budget_optimize( expected, keybs, memory).unwrap();
        assert_eq!(expected, result);
    }

    // Provided HackerRank test
    #[test]
    fn test_valid() {

        let keybs = vec![3, 1];
        let memory = vec![5, 2, 8];

        assert_eq!(9, budget_optimize( 10, keybs, memory).unwrap());
    }

    // Provided HackerRank test
    #[test]
    fn impossible_choice() -> Result< (), String> {
       if budget_optimize(5,  vec![5],  vec![4]).is_err() {
            Ok(())
       } else {
           Err( "Options out of budget should fail".into())
       }
    }


    // Pesky case, solution should be 50+10 (60)
    #[test]
    fn uneven_solution() {
        assert_eq!(60, budget_optimize( 60, vec![50, 60], vec![5, 10]).unwrap());
    }


    #[test]
    fn wrong_budget() -> Result< (), String> {
       if budget_optimize(0,  vec![12, 30,55],  vec![12, 30,55]).is_err() {
            Ok(())
       } else {
           Err( "0 budget should fail".into())
       }
    }

    #[test]
    fn wrong_list() -> Result<(), String> {
        if budget_optimize(10, Vec::<u64>::new() ,  vec![123,123]).is_err() && 
            budget_optimize(10,   vec![123,123],Vec::<u64>::new() ).is_err() 
        {
            Ok(())
        } else {
            Err("Empty starting lists should fail".into())
        }
    }

}