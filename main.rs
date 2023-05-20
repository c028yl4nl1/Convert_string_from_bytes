// vou usar um conversor de caracter para bytes
use std::io;

fn main() {
    println!("Sejam Muito bem vindo ao conversor de caracter para bytes");

    println!("Digite o Caracterer Abaixo:");

    let mut obter_caracter: String = String::new();

    io::stdin()
        .read_line(&mut obter_caracter)
        .expect("Error ao ler o caracter");

    println!(
        "O Total de Carater é de {}",
        obter_caracter.trim().chars().count()
    );

    println!(
        "Esses {} caracter São {} Bytes ",
        obter_caracter.trim().chars().count(),
        obter_caracter.trim().len()
    );
}
