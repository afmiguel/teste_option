fn testa_divisor(numero: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        -1  // Retornamos -1 se o divisor for zero, pois a divisão por zero não é permitida
    } else if numero % divisor == 0 {
        divisor  // Retornamos o divisor se o divisor for um divisor do número
    } else {
        -1  // Retornamos -1 se o divisor não for um divisor do número
    }
}

fn main() {
    let numero = 20;
    let divisor = 4;

    let resultado = testa_divisor(numero, divisor);
    if resultado == -1 {
        println!("Nenhum divisor encontrado ou divisor é zero");
    } else {
        println!("{} é um divisor de {}", resultado, numero);
    }

    let divisor_zero = 0;
    let resultado_zero = testa_divisor(numero, divisor_zero);
    if resultado_zero == -1 {
        println!("Nenhum divisor encontrado ou divisor é zero");
    } else {
        println!("{} é um divisor de {}", resultado_zero, numero);
    }
}
