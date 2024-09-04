pub fn pruebaInmutalidad(){
    let mut x: i32 = 1;
    println!("El valor de x es {}", x);
    x = 2;
    print!("Esta linea se ejecutara si esta el mut indicando la mutabilidad") 
}

const ESTOESUNACONSTANTE :i8 = 1;


//El shadowing crea una nueva varible y oculta el valor de la anterior
pub fn shadowing(){
    let x: i32 = 1;
    let x: i32 = x + 23;
    let x: i32 = x-23/23;
    println!("El resultado es x = {}", x)
}

// Tipos de Datos en Rust
pub fn scalars() {
    // Enteros
    let ocho_bits :i8 = 1;
    let diezseis_bits :i16 = 1;
    let tresdos_bits :u32 = 1;
    let seiscuatro_bits :i64 = 1;
    let unodosocho_bits :i128 = 1; 

    //Floats y operaciones simples
    let float_tresdos :f32 = 2.65;
    let float_dato: f32 = float_tresdos + 42.5;

    //Bool
    let variable : bool = true;

    //Character
    let my_char:char = 'a';
}

//para indicar que esto es una funcion debemos poner el simbolo ->i32
pub fn compound() ->i32 {
    //Tipos compuestos
    //Tuplas
    let tup: (i32, f64, char) = (1,2.345, 'b');

    //Arrays
    let arr: [i32; 4] = [1,2,3,4];

    //obtengo los valores del array
    let val: i32 = arr[0];
    let val2: i32 = arr[3];

    //La estructura del for
    for i in arr.iter(){
        print!("{}", i);
    }

    // La estructura del while
    let mut ent: i32 = 0;
    while ent < 10 {
        ent = ent +1;
    }

    //Esto seria una especia del while do
    loop {
        ent = ent - 10;
        println!("Valor de ent: {}", ent);

        // Agregar una condición de salida para evitar un bucle infinito
        if ent <= 0 {
            break;
        }
    }

    //supongo que esta es una funcio y devuelve el ent
    return ent;
    //no hace falta poner el return puedo poner solo el ent
    //ent;

}


