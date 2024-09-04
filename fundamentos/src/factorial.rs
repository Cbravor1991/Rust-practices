
pub fn calcular_factorial(number: u128) -> u128{
    if number == 0 || number == 1{
        1
    }else {
        let mut result: u128 = number;
        for i in (1..number).rev(){
            result = result * i;
        }
        result
    }
}