 //solving for the roots of a quadratic equation using the determinant

 use std::io;

 fn main() {

    let input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Input value for a: ");
    io::stdin().read_line(&mut input2).expect("Not a valid value");
    let coefficienta:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Input value for b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid value");
    let coefficientb:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Input value for c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid value");
    let constant:f32 = input3.trim().parse().expect("Not a valid value");

    let evaluation:f32 = (coefficientb * coefficientb) - (4.00 * coefficienta * constant);
    let determinant:f32 = evaluation.sqrt();

    println!("The value of the determinant = {}",determinant);

    if determinant > 0.00 {
        println!("There are two distinct roots",);
    }
    else if determinant == 0.00 {
        println!("There is one real root",);
    }
    else if determinant < 0.00 {
        println!("There are no real roots");
    }
}

    

