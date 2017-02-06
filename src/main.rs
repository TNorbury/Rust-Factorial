fn main() {

    //Read the command line arguments the get a number. nth(1) returns an 
    //iterator to the 1st argument (with the 0th argument being the path of the
    // executable). unwrap() then takes the value the iterator is pointing and
    //returns it. parse() then takes this value and converts it to another type
    //which in this case is u64. expect throws an error if the value being 
    //parsed can't be converted to the given type.
    let number: u64 = std::env::args().nth(1).unwrap().parse()
        .expect("Must pass a number");

    //Print the result of number!
    print!("{}", factorial(number));
}


fn factorial(num: u64) -> u64
{

    //This matches the value of num to either 1 or _ (which is equivalent to
    //the default clause in a C switch). The match statement will return either
    //1 or the value of num!
    match num {
        1 => 1,
        _ => num * factorial(num - 1),
    }
}