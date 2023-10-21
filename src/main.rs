use std::io;

fn convert_to_int(data_input: &String) -> i32{
    let x: i32 = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut numero_medias: String = String::new();
    io::stdin().read_line(&mut numero_medias).expect("Erro ao ler entrada_fatorial");

    let mut numero_medias_int: i32 = convert_to_int(&numero_medias);


    let mut qtd_alunos_reprovados: i32 = 0;

    while numero_medias_int > 0{
        let mut entrada_nota: String = String::new();
        io::stdin().read_line(&mut entrada_nota).expect("Erro ao ler entrada_fatorial");

        if convert_to_int(&entrada_nota) < 6 {qtd_alunos_reprovados = qtd_alunos_reprovados +1}


        numero_medias_int = numero_medias_int -1;
    }

    println!("Reprovados: {}", qtd_alunos_reprovados)    
}
