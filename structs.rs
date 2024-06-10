//#[derive(Debug)]
struct Conta {
    titular: Titular,
    saldo: f64
}

impl Conta {
    fn sacar(&mut self, valor: f64) {
        self.saldo -= valor;
    }
}

//#[derive(Debug)]
struct Titular {
    nome: String,
    sobrenome: String
}

fn main() {
    let titular = Titular{nome: String::from("Pedro"), sobrenome: String::from("Augusto")};
    let mut conta: Conta = Conta{
        titular,
        saldo: 100.0
    };

    conta.sacar(50.0);

    println!(
        "Dados da conta: Titular = {} {}, Saldo = {}",
        conta.titular.nome,
        conta.titular.sobrenome,
        conta.saldo
    );

    //println!("Conta: {:?}", conta);     
}