
mod primo;
mod fundamentos;
mod factorial;

fn main() {
    //fundamentos::pruebaInmutalidad();

    //Vamos a calcular el factorial
    println!("Calculo el factorial de 25");
    let resultado_factorial = factorial::calcular_factorial(25);
    println!("El resultado del factorial es: {}", resultado_factorial );

    //vamos a calcular si un nro es primo o no
    println!("Calculo si un nro es primo, el 25 en este caso");
    let resultado_primo: bool = primo::es_primo(25);
    println!("Es primo: {}", resultado_primo);


}

