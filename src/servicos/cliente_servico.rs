use crate::models::cliente::Cliente;
use std::io::{Seek, SeekFrom, Write};
use std::fs::{File, OpenOptions};
use std::io::{Read};
use uuid::Uuid;

const CLIENTES_JSON: &str = "db/clientes.json";

pub fn criar(nome: &str, telefone: &str) -> std::io::Result<()> {
    let mut arquivo_leitura = File::open(CLIENTES_JSON).unwrap_or_else(|_| File::create(CLIENTES_JSON).unwrap());
    let mut dados = String::new();
    arquivo_leitura.read_to_string(&mut dados)?;
    drop(arquivo_leitura); //Encerra explicintamente o uso do arquivo de leitura

    let mut clientes: Vec<Cliente> = serde_json::from_str(&dados).unwrap_or_else(|_| Vec::new());

    // Adiconar o novo cliente.

    let novo_cliente = Cliente {
        id: Uuid::new_v4(),
        nome: nome.to_string(),
        telefone: telefone.to_string(),
    };
    clientes.push(novo_cliente);

    // Serializa a lista de clientes atualizada para JSON.
    let dados_json = serde_json::to_string(&clientes)?;

    // Reabre o arquivo para escrita, truncando-o automaticamente.
    let mut arquivo_escrita = OpenOptions::new().write(true).truncate(true).truncate(true).open(CLIENTES_JSON)?;
    arquivo_escrita.write_all(dados_json.as_bytes())?;

    Ok(())
}
pub fn listar() -> std::io::Result<Vec<Cliente>> {
    let mut arquivo = File::open(CLIENTES_JSON)?;
    
    let mut dados = String::new();
    arquivo.read_to_string(&mut dados)?;

    let clientes:Vec<Cliente> = serde_json::from_str(&dados)?;
    Ok(clientes)
}

pub fn atualizar(id: Uuid, novo_nome: &str, novo_telefoe: &str) -> std::io::Result<()> {
    
    let mut clientes = listar()?;

    if let Some(clientes) = clientes.iter_mut().find(|c| c.id == id) {
       clientes.nome = novo_nome.to_string();
       clientes.telefone = novo_telefoe.to_string();
    } else {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Cliente não encontrado"));
    }

    let dados_json = serde_json::to_string(&clientes)?;
    let mut arquivo = OpenOptions::new().read(true).write(true).open(CLIENTES_JSON)?;
    arquivo.set_len(0)?;
    arquivo.seek(SeekFrom::Start(0))?; // Reposicionado o cursor para iricio do arquivo antes da posição de escreve o arquivo
    arquivo.write_all(dados_json.as_bytes())?;

    Ok(())
    
}

pub fn excluir(id: Uuid) -> std::io::Result<()> {
    let mut clientes = listar()?;

    clientes.retain(|cliente| cliente.id != id); // Remove o cliente com o ID especificado

    let dados_json = serde_json::to_string(&clientes)?;
    let mut arquivo = OpenOptions::new().read(true).write(true).open(CLIENTES_JSON)?;
    arquivo.set_len(0)?;
    arquivo.seek(std::io::SeekFrom::Start(0))?; //Reposiciona o cursor para o inicio do arquivo
    arquivo.write_all(dados_json.as_bytes())?;

    Ok(())

}