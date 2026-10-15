use crate::servicos::cliente_servico;
use std::thread;
use std::time::Duration;
use std::io;
use std::process::Command;
use uuid::Uuid;

pub fn criar_clientes() {
    limpar_tela();
    let mut nome = String::new();
    let mut telefone = String::new();

    println!("Nome do Cliente: ");
    io::stdin().read_line( &mut nome).expect("Falha ao ler o nome");

    println!("telefone do cliente: ");
    io::stdin().read_line( &mut telefone).expect("Falha ao ler o telefone");

    cliente_servico::criar(&nome.trim(), &telefone.trim()).expect("Falha ao criar");

    println!("Cadastro realizado com sucesso");
    pausar_por_segundos(2);
    limpar_tela();

}

pub fn mostrar_clientes() -> Result<(), std::io::Error> {
    limpar_tela();
    let clientes = cliente_servico::listar()?; // Se retorna erro do servço, dispara o erro
    for cliente in &clientes {
        println!("--------------------------------"); //Risco na tela
        println!("ID: {}", cliente.id);
        println!("Nome: {}", cliente.nome);
        println!("Telefone: {}", cliente.telefone);
    }

    println!("----------------------------------"); // Risco apos o ultimo cliente

    pausar_ate_enter();
    limpar_tela();

    Ok(())
}

fn pausar_ate_enter() {
    println!("Pressione Enter para continuar...");
    let mut _descarta = String::new(); // Variavel temporaria, prefixada
    io::stdin().read_line(&mut _descarta).expect("Falha ao ler a entrada");
    limpar_tela();
}

fn limpar_tela() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "cls"])
                .status()
                .unwrap();
    } else {
        Command::new("clear")
                .status()
                .unwrap();
    }
}

fn pausar_por_segundos(segundos: u64 ) {
    thread::sleep(Duration::from_secs(segundos));
}

pub fn atualizar_cliente() -> Result<(), std::io::Error> {
    limpar_tela();
    let mut id  = String::new();
    let mut nome  = String::new();
    let mut telefone = String::new();

    println!("ID do Cliente a ser atualizado: ");
    io::stdin().read_line(&mut id).expect("Falha ao ler o ID");
    let  id = id.trim().parse::<Uuid>().expect("ID invalida");
    

    println!("Novo nome do cliente: ");
    io::stdin().read_line(&mut nome).expect("Falha ao ler o nome");

    println!("Novo telefone do Cliente: ");
    io::stdin().read_line(&mut telefone).expect("Falha ao ler o telefone");

    cliente_servico::atualizar(id, &nome.trim(), &telefone.trim()).expect("Falha");

    println!("Cliente atualizado com sucesso");
    pausar_por_segundos(2);
    limpar_tela();

    Ok(())

}

pub fn excluir_cliente() -> Result<(), std::io::Error> {
    limpar_tela();
    let mut id = String::new();

    println!("ID do Cliente a ser excluido: ");
    io::stdin().read_line(&mut id).expect("Falha ao ler o ID");
    let id = id.trim().parse::<Uuid>().expect("ID invalido");

    cliente_servico::excluir(id).expect("Falha ao excluir cliente");

    println!("Cliente excluido com sucesso.");
    pausar_por_segundos(2);
    limpar_tela();

    Ok(())
}