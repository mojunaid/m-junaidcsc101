use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();

    let menu = "   menu                           price
                 p= Poundo Yam/Edinkaiko Soup     -N3,200
                 f= Fried Rice and Chicken        -N3,000
                 a= Amala and Ewedu Soup          -N2,500
                 e= Eba and Egusi Soup            -N2,000
                 w= White Rice and Stew           -N2,500";
    
    println!("Enter the price of your order
                   {}",menu);

    println!("Order 1: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let order_1:f32 = input1.trim().parse().expect("Not a valid price");

    println!("Order 2: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let order_2:f32 = input2.trim().parse().expect("Not a valid price");

    println!("Order 3: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let order_3:f32 = input3.trim().parse().expect("Not a valid price");

    println!("Order 4: ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let order_4:f32 = input4.trim().parse().expect("Not a valid price");

    println!("Enter the letter code of your order to confirm (ex. f, e, a): ");
    io::stdin().read_line(&mut input5).expect("Not a valid code");

    let total:f32 = order_1 + order_2 + order_3 + order_4;
    println!("Your total is: {}",total);

    if total > 10000.00 {
        let discount:f32 = total - ((5.00/100.00) * total);
        println!("A 5% discount ha been applied to your order, Your new total is: {}",discount);
    }
}

