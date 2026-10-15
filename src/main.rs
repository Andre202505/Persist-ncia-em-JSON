mod models;
mod servicos;
mod tela;

use std::io;
use tela::display;


fn main() {
    loop {
        println!("CRUD Cliente");
        println!("1. Criar Cliente");
        println!("2. listar Clientes");
        println!("3. Atualizar Cliente");
        println!("4. Excluir Clientes");
        println!("5. Sair");

        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).expect("Falha ao ler a entrada");

        match escolha.trim() {
            "1" => display::criar_clientes(),
            "2" => {
                if let Err(e) = display::mostrar_clientes() {
                    println!("Erro ao listar Clientes: {}", e);
                }
            },
            "3" => {
                if let Err(e) = display::atualizar_cliente() {
                    println!("Erro ao atualizar cliente: {}", e);
                    
                }
            },
            "4" => {
                if let Err(_e) = display::excluir_cliente() {
                    println!("Erro ao excluir cliente");
                    
                }
            },
            "5" => {
                println!("Saindo...");
                break;
            }
            _ => println!("Opção invalida, tente novamente."),
        }
    }
}
