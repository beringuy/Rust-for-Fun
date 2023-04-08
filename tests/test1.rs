#[test]
fn main () {
    let cont = 1;

    let x = match cont == 1{
        true => 11,
        false => 12,
    };

    let y = if cont == 2 {11} else {12};

    {

    }

    println!("Resultado 1:");
    println!("{}", x);

    println!("Resultado 2:");
    println!("{}", y);

}