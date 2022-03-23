fn main() {
    let mut number = 3;
    let number2 = 7;

    let mut aux = 1;
    while aux < 3{
        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }
        number = number2;
        aux = aux + 1; 
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

}
