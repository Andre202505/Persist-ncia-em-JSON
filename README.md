# 🦀 Cadastro de Clientes em Rust

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Serde](https://img.shields.io/badge/Serde-JSON-orange?style=for-the-badge)
![Cargo](https://img.shields.io/badge/Cargo-Rust-black?style=for-the-badge&logo=rust&logoColor=white)

> Aplicação de linha de comando desenvolvida em **Rust** para gerenciamento de clientes e persistência de dados utilizando **JSON**.

---

## 📌 Sobre o projeto

Este projeto foi desenvolvido como parte dos meus estudos em **Rust**, com o objetivo de praticar conceitos fundamentais de desenvolvimento Back-end e persistência de dados.

A aplicação permite realizar operações de **CRUD (Create, Read, Update e Delete)** sobre clientes, armazenando as informações em um arquivo JSON.

O projeto também foi organizado em módulos, buscando aplicar uma estrutura de código mais limpa e facilitar a manutenção e evolução da aplicação.

---

## ⚙️ Funcionalidades

- [x] ➕ Cadastro de clientes
- [x] 📋 Listagem de clientes
- [x] ✏️ Atualização de clientes
- [x] 🗑️ Exclusão de clientes
- [x] 💾 Persistência dos dados em JSON
- [x] 🆔 Identificação dos clientes através de UUID
- [x] 🖥️ Interface através do terminal
- [x] 📦 Organização do código em módulos

---

## 🛠️ Tecnologias utilizadas

| Tecnologia | Utilização |
|---|---|
| 🦀 **Rust** | Linguagem principal |
| 📦 **Cargo** | Gerenciamento do projeto e dependências |
| 🔄 **Serde** | Serialização e desserialização |
| 📄 **Serde JSON** | Manipulação dos dados em JSON |
| 🆔 **UUID** | Identificação única dos clientes |
| 💻 **CLI** | Interface através do terminal |

---

## 🏗️ Estrutura do projeto

```text
Persist-ncia-em-JSON/
│
├── 📁 db/
│   └── clientes.json
│
├── 📁 src/
│   ├── 📁 models/
│   │   ├── cliente.rs
│   │   └── mod.rs
│   │
│   ├── 📁 servicos/
│   │   ├── cliente_servico.rs
│   │   └── mod.rs
│   │
│   ├── 📁 tela/
│   │   ├── display.rs
│   │   └── mod.rs
│   │
│   └── main.rs
│
├── 📄 Cargo.toml
├── 📄 Cargo.lock
└── 📄 .gitignore
```

### 📂 `models`

Contém as estruturas responsáveis por representar os dados utilizados pela aplicação.

### 📂 `servicos`

Concentra a lógica relacionada às operações realizadas sobre os clientes e à persistência dos dados.

### 📂 `tela`

Responsável pela interação com o usuário através do terminal.

### 📂 `db`

Local onde os dados persistidos da aplicação são armazenados em formato JSON.

---

## 🔄 Como funciona

Ao iniciar a aplicação, o usuário pode escolher uma das operações disponíveis no terminal.

```text
CRUD Cliente

1. Criar Cliente
2. Listar Clientes
3. Atualizar Cliente
4. Excluir Cliente
5. Sair
```

As informações cadastradas são armazenadas em um arquivo JSON, permitindo que os dados permaneçam disponíveis mesmo após o encerramento da aplicação.

---

## 🚀 Como executar

### Pré-requisitos

Para executar o projeto, é necessário ter o **Rust** e o **Cargo** instalados.

### 1️⃣ Clone o repositório

```bash
git clone https://github.com/Andre202505/Persist-ncia-em-JSON.git
```

### 2️⃣ Acesse a pasta

```bash
cd Persist-ncia-em-JSON
```

### 3️⃣ Execute o projeto

```bash
cargo run
```

### 4️⃣ Para compilar

```bash
cargo build
```

---

## 🧠 Conceitos praticados

Durante o desenvolvimento deste projeto, foram praticados conceitos importantes da linguagem Rust:

- `struct`
- `enum`
- `mod`
- `match`
- `Result`
- tratamento de erros
- ownership e borrowing
- leitura e escrita de arquivos
- serialização e desserialização
- manipulação de JSON
- UUID
- organização modular
- operações CRUD
- entrada e saída de dados pelo terminal

---

## 📚 O que aprendi

Este projeto foi importante para consolidar conhecimentos sobre como uma aplicação Rust pode trabalhar com **dados persistentes**, indo além de simplesmente manter informações em memória.

Também pratiquei a separação de responsabilidades através de módulos, trabalhando conceitos que podem ser utilizados posteriormente em aplicações maiores.

Um dos principais aprendizados foi entender o fluxo:

```text
Usuário
   ↓
Interface CLI
   ↓
Serviço
   ↓
Modelo
   ↓
Arquivo JSON
```

---

## 🔮 Próximos passos

Pretendo continuar evoluindo o projeto adicionando novas funcionalidades e melhorias:

- [ ] 🔎 Busca de clientes
- [ ] ✅ Validação dos dados
- [ ] ⚠️ Melhor tratamento de erros
- [ ] 🧪 Testes automatizados
- [ ] 🖥️ Melhorar a interface do terminal
- [ ] 📊 Melhorar a organização dos dados
- [ ] 🗄️ Utilizar banco de dados
- [ ] 🌐 Criar uma API REST
- [ ] 🔐 Implementar autenticação
- [ ] 📖 Melhorar a documentação

---

## 🎯 Objetivo

O objetivo deste projeto é aplicar na prática os conhecimentos adquiridos durante meus estudos de **Rust**, desenvolvendo uma aplicação simples, mas estruturada, para gerenciamento e persistência de dados.

Este projeto representa parte da minha evolução como **Desenvolvedor Full Stack em formação**, com foco no fortalecimento dos conhecimentos em Back-end e desenvolvimento de software.

---

## 👨‍💻 Autor

### André Aragão

**Desenvolvedor Full Stack em formação**

Estudando e desenvolvendo projetos práticos com:

- JavaScript
- React
- Node.js
- Rust
- APIs REST
- PostgreSQL
- MongoDB
- Git & GitHub

### 🌐 GitHub

👉 [github.com/Andre202505](https://github.com/Andre202505)

---

## ⭐ Contribuição

Este é um projeto de estudos e evolução profissional, mas sugestões e contribuições são sempre bem-vindas.

Se você gostou do projeto, considere deixar uma ⭐ no repositório!

---

<p align="center">
  Desenvolvido com 🦀 Rust por <strong>André Aragão</strong>
</p>
