pub fn es_primo(number: u128)-> bool{
    let mut es_primo: bool = true;
    let num: f64 = number as f64;
    if number > 1 {
        for i in 2..((num.sqrt() as i128) + 1){
            if (number as i128) % i == 0{
                es_primo = false;
                break;
            }
        }
    }
    es_primo
}