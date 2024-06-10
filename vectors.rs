fn main() {
    let mut notas: Vec<f32> = Vec::with_capacity(4);
    notas.push(10.0);
    notas.push(8.0);
    notas.push(6.5);
    notas.push(6.8);

    println!("Capacidade = {}", notas.capacity());

    notas.push(7.5);
 
    println!("Capacidade = {}", notas.capacity());
    
    println!("{:?}", notas);

    println!("Nota 1 = {}", notas[0]);

    /*
    while let Some(nota) = notas.pop() {
        println!("Valor removido = {}", nota);
    }
    */

    for nota in &notas {
        println!("Nota = {}", nota);
    }

    //println!("{:?}", notas);
}