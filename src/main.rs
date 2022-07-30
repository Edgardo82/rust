use regex::Regex; //clase 11

//clase 15 definir funcion, se le puede definir retorno
fn sumar_uno(numero_a_sumar:i32)->i32{
    let numero_final=numero_a_sumar +1;
    println!("{}",numero_final);
    return numero_final
}

fn main() {
/*    
// clase 6: creacion de variables

    let nombre:&str = "Edgardo";
    let edad:u8 = 40;       
    println!("Hola soy {} y tengo {} años",nombre, edad);
    
//clase 7: ingresar datos de usuario

    println!("por favor introduce tu nombre");
    let mut nombre:String = String ::new();
    std ::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();

    //obtener edad
    println!("por favor introduce tu edad");
    let mut edad:String = String ::new();
    std ::io::stdin().read_line(&mut edad).unwrap();

    // convertir edad a int
    let edad_int:u8=edad.trim().parse().unwrap();
    println!("hola, bienvenido {} con {} años", nombre, edad_int);

//clase 8: condicionales
    println!("por favor introduce tu edad");
    let mut edad:String = String ::new();
    std ::io::stdin().read_line(&mut edad).unwrap();
    // convertir edad a int
    let edad_int:u8=edad.trim().parse().unwrap();
    if edad_int >= 18 {
        println!("puede entrar al boliche");
    } else{
        println!("no puedes entrar al boliche")
    }
    println!("tu edad es de {} años", edad_int);
//clase 9: loop
    let numero_1 = 123;
    let numero_2 = 321;
    let suma = numero_1 + numero_2;
    loop {
        println!("Por favor imprimir la suma de {} y {}:", numero_1,numero_2);

        let mut suma_usuario = String :: new();
        std::io::stdin().read_line(&mut suma_usuario).unwrap();

        let suma_usuario_int : i16 = suma_usuario.trim().parse().unwrap();

        if suma_usuario_int == suma {
            println!("los has hecho muy bien, el resultado {} es correcto",suma);
            break;
        }else{
            println!("El resultado {} no es correcto por favor intentalo nuevamente",suma_usuario_int)
        }

    }
 */
//clase 13
/*
    //regex
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_subs=Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let re_mult= Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_div=Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap();
    //traer datos de usuarios
    println!("por favor introduce tu expresion : ");
    let mut expression = String :: new();
    std::io::stdin().read_line(&mut expression).unwrap();
    //división
    loop{
        //aplicar operaciones
        let caps=re_div.captures(expression.as_str());
        if caps.is_none(){
            break;
        }
        let caps= caps.unwrap();

        let cap_expression =caps.get(0).unwrap().as_str();
        let left_value:i32 =caps.get(1).unwrap().as_str().parse().unwrap();
        let rigth_value:i32 =caps.get(2).unwrap().as_str().parse().unwrap();        
        println!("{:?} izq:{},der:{},cap_expression:{}",caps,left_value,rigth_value,cap_expression);//imprimir detalle de la variable {:?}
        let mult = left_value / rigth_value;
        expression = expression.replace(cap_expression,&mult.to_string());
        //expresion espera str, la variable es i32, se castea a string con to_string() y con el & se castea a str

    }
    //multiplicacion
    loop{
        //aplicar operaciones
        let caps=re_mult.captures(expression.as_str());
        if caps.is_none(){
            break;
        }
        let caps= caps.unwrap();

        let cap_expression =caps.get(0).unwrap().as_str();
        let left_value:i32 =caps.get(1).unwrap().as_str().parse().unwrap();
        let rigth_value:i32 =caps.get(2).unwrap().as_str().parse().unwrap();        
        println!("{:?} izq:{},der:{},cap_expression:{}",caps,left_value,rigth_value,cap_expression);//imprimir detalle de la variable {:?}
        let mult = left_value * rigth_value;
        expression = expression.replace(cap_expression,&mult.to_string());
        //expresion espera str, la variable es i32, se castea a string con to_string() y con el & se castea a str

    }
    //suma
    loop{
        //aplicar operaciones
        let caps=re_add.captures(expression.as_str());
        if caps.is_none(){
            break;
        }
        let caps= caps.unwrap();

        let cap_expression =caps.get(0).unwrap().as_str();
        let left_value:i32 =caps.get(1).unwrap().as_str().parse().unwrap();
        let rigth_value:i32 =caps.get(2).unwrap().as_str().parse().unwrap();        
        println!("{:?} izq:{},der:{},cap_expression:{}",caps,left_value,rigth_value,cap_expression);//imprimir detalle de la variable {:?}
        let addition = left_value +rigth_value;
        expression = expression.replace(cap_expression,&addition.to_string());
        //expresion espera str, la variable es i32, se castea a string con to_string() y con el & se castea a str

    }
    loop{
        //aplicar operaciones
        let caps=re_subs.captures(expression.as_str());
        if caps.is_none(){
            break;
        }
        let caps= caps.unwrap();

        let cap_expression =caps.get(0).unwrap().as_str();
        let left_value:i32 =caps.get(1).unwrap().as_str().parse().unwrap();
        let rigth_value:i32 =caps.get(2).unwrap().as_str().parse().unwrap();        
        println!("{:?} izq:{},der:{},cap_expression:{}",caps,left_value,rigth_value,cap_expression);//imprimir detalle de la variable {:?}
        let mult = left_value - rigth_value;
        expression = expression.replace(cap_expression,&mult.to_string());
        //expresion espera str, la variable es i32, se castea a string con to_string() y con el & se castea a str

    }   

    
    //println!("{:?} izq:{},der:{}",caps,left_value,rigth_value);//imprimir detalle de la variable {:?}
    //mostrar resulatado   
    println!("Resultado: {}",expression);
*/
// clase 14
/*
    //vectores
    let mut nombres:Vec<String>=Vec::new();
    for i in  0..5   {
    
    println!("Por favor introduce tu nombre: ");
    let mut nombre = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();

    nombres.push(nombre);
    
    println!("{}",nombres[i]);

    }
    println!("{:?}",nombres);
    for nombre in nombres{
        println!("el nombre es {}",nombre)
    }
    let hola = ["HOla", "Chau", "Bye"];//lista inmutable 
    
    println!("{:?}",hola);
*/
//clase 15
    
    let mut suma = sumar_uno(10);
    suma = sumar_uno(suma);
    println!("{}",suma);
    
    for i in 10..11{
        sumar_uno(i);
    }

}
