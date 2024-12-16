use std::io;

fn convert_to_int(data_input: &String) -> i32{
    let x: i32 = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    
    let mut entrada: String = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro ao ler entrada");

    let mut fatorial: i32 = 1;
    let entrada_int: i32 = convert_to_int(&entrada);

    for i in 1..entrada_int+1{
        println!("i: {}", i);
        fatorial = fatorial * i;
    }

    println!("fatorial: {}", fatorial)
}
