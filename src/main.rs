use polars::prelude::*;

// teste

fn main() {
    // Rust in Visual Studio Code : [https://code.visualstudio.com/docs/languages/rust]
    // Rust Basics                : [https://medium.com/learning-rust/rust-basics-e73304ab35c7]

    let word = "Teste"; // let > Variable
    let num = 1;
    const B : bool = true; // const > Constant
    let (x, y) = (1, 2); // dupla atribuição de valores


    // Aqui, um comentário no RUST

    println!("\nCaso 1 : {} {}", word, num);
    println!("Caso 2 : {palavra} {numero}", palavra = word, numero = num);

    if B == true {
        println!("Caso 3: {}\n", x+y);
    } else {
        println!("Caso 4: {}\n", x-y);
    }

    //-------------------------------------------------------------------

    for a in 0..10 { // (a = 0; a <10; a++) // 0 to 10 (exclusive)
        println!("Current value : {}", a);
    }

    println!("-------------------------");

    let lista = ["um" , "dois" , "três"];

    for x in lista {
        println!("{x}");
    }

    //-------------------------------------------------------------------

    println!("\n");
    
    let df = df_maker("/home/ego/Downloads/Codes and Data/Datasets/COUGHVID/metadata_compiled_plusExt.csv");

    println!("{:?}", df);

}

//-------------------------------------------------------------------

fn df_maker(path: &str) -> Result<DataFrame, PolarsError> {
    CsvReader::from_path(path)?
            .has_header(true)
            .finish()
}