fn main() {
    let mut uma_string = String::from("Teste");
    rouba(uma_string);

    println!("{}", uma_string);
}

fn rouba(string: String) {
    println!("{}", string);
}