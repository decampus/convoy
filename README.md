# Convoy

Este projeto é uma aplicação de chat segura e minimalista construída com Rust usando o framework Actix Web. Ele possui autenticação de usuário, um painel de administrador para criação de usuários e criptografia de ponta a ponta para todas as mensagens armazenadas no banco de dados PostgreSQL.

Autor: Moisés William Albuquerque Campos

Matrícula: 2021004847

## Dependências de Criptografia e Hashing

A segurança desta aplicação depende de alguns dependências essenciais para lidar com o hashing de senhas e a criptografia de mensagens.

* **`bcrypt`**: Usado para o hashing de mão única das senhas dos usuários. É uma função de hashing forte e adaptativa, projetada para ser lenta, o que protege contra ataques de força bruta.
* **`aes-gcm`**: Implementa a cifra de criptografia autenticada AES-256-GCM. É usada para criptografar e descriptografar o conteúdo das mensagens, garantindo que as mensagens armazenadas no banco de dados permaneçam confidenciais e à prova de adulteração.
* **`rand`**: Fornece um gerador de números aleatórios criptograficamente seguro, que é essencial para criar um nonce (um número usado apenas uma vez) único para cada mensagem criptografada com AES-GCM.
* **`hex`**: Usado para decodificar a representação hexadecimal da `ENCRYPTION_KEY` do arquivo `.env` para os bytes brutos necessários para a cifra de criptografia.

## Executando o Projeto

Siga estes passos para configurar e executar a aplicação localmente.

### Pré-requisitos

* **Rust e Cargo**: Instale a partir de [rust-lang.org](https://www.rust-lang.org/tools/install).
* **PostgreSQL**: Uma instância do servidor PostgreSQL em execução.
* **`sqlx-cli`**: Instale a ferramenta de linha de comando do SQLx:
    ```bash
    cargo install sqlx-cli
    ```

### Instruções Passo a Passo

1.  **Clone o Repositório:**
    Clone ou baixe os arquivos do projeto para sua máquina local.

2.  **Configure as Variáveis de Ambiente:**
    Crie um arquivo `.env` na raiz do projeto com as seguintes chaves:

    ```
    DATABASE_URL=postgres://USUARIO:SENHA@localhost:5432/NOME_BANCO
    ADMIN_USERNAME=
    ADMIN_PASSWORD=
    ENCRYPTION_KEY=
    ```
    * Atualize a `DATABASE_URL` com suas credenciais do PostgreSQL.
    * **Importante**: Gere uma nova `ENCRYPTION_KEY` usando `openssl rand -hex 32`.

3.  **Configure o Banco de Dados:**
    Execute os seguintes comandos `sqlx` para criar o banco de dados e aplicar todas as migrações de esquema.

    ```bash
    # Cria o banco de dados especificado na DATABASE_URL
    sqlx database create

    # Executa todos os arquivos de migração do diretório /migrations
    sqlx migrate run
    ```

4.  **Compile e Execute a Aplicação:**
    Use o Cargo para compilar e executar o servidor.

    ```bash
    cargo run
    ```
    O servidor será iniciado em `http://127.0.0.1:8080`.

5.  **Acesse a Aplicação:**
    * **Painel do Administrador**: Navegue para `http://127.0.0.1:8080/admin` para fazer login como administrador e criar novos usuários.
    * **Chat**: Navegue para `http://127.0.0.1:8080` para fazer login como um usuário comum e começar a conversar.

## Estrutura do Projeto

O projeto é organizado em vários módulos para separar as responsabilidades, tornando a base de código mais fácil de manter e entender.

| Arquivo / Diretório            | Descrição                                                                                                                                | Relacionamentos                                                                                                                                  |
| ------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `src/main.rs`                  | O ponto de entrada principal. Configura o servidor Actix, o pool de conexões do banco de dados, as rotas e o serviço de arquivos estáticos. | Inicializa e conecta todos os outros módulos. Inicia o servidor e escuta por requisições.                                                        |
| `src/handlers.rs`              | Contém os manipuladores de rota do Actix que processam as requisições HTTP.                                                              | Chamado pelas rotas em `main.rs`. Usa os módulos `auth`, `crypto` e `db` para atender às requisições.                                        |
| `src/db.rs`                    | Gerencia todas as interações com o banco de dados usando `sqlx`. Contém funções para criar/encontrar usuários e criar/obter mensagens.   | Chamado por `handlers.rs` e pela tarefa de inicialização em `main.rs` para interagir com o banco de dados.                                     |
| `src/auth.rs`                  | Lida com toda a lógica de autenticação e autorização de usuários, incluindo verificação de senha e checagens de administrador.            | Usado por `handlers.rs` para proteger rotas. Usa `db.rs` para encontrar usuários e as variáveis do `.env` para as credenciais de administrador. |
| `src/crypto.rs`                | Encapsula toda a lógica de criptografia e descriptografia de mensagens usando AES-GCM.                                                   | Usado por `handlers.rs` para criptografar mensagens antes de serem enviadas para `db.rs` e descriptografá-las após serem buscadas.               |
| `src/models.rs`                | Define as estruturas de dados centrais (`structs`) como `User` e `ChatMessage`, que representam entidades do banco de dados e payloads da API. | Usado por `db.rs` para o mapeamento do banco de dados e por `handlers.rs` para a serialização de requisições/respostas da API.                |
| `src/errors.rs`                | Define o enum customizado `AppError` para um tratamento de erros unificado em toda a aplicação.                                          | Usado em toda a aplicação para retornar respostas de erro HTTP consistentes.                                                                    |
| `static/`                      | Contém todos os arquivos estáticos de frontend, como `index.html` (a UI principal do chat) e `admin.html` (o painel do administrador).      | Servido pelo `actix-files` conforme configurado em `main.rs`. Interage com os endpoints da API definidos em `handlers.rs`.                    |
| `migrations/`                  | Contém os arquivos de migração SQL que definem o esquema do banco de dados. Gerenciado pelo `sqlx-cli`.                                   | O esquema definido aqui deve corresponder às `structs` em `models.rs` e às consultas em `db.rs`.                                                |
| `.env`                         | Armazena segredos de configuração como URLs de banco de dados e chaves de criptografia.                                                  | Lido pela aplicação na inicialização (`main.rs`) para configurar a conexão com o banco de dados, as credenciais de admin e a chave de criptografia. |
| `Cargo.toml`                   | O arquivo do gerenciador de pacotes do Rust. Define os metadados do projeto e todas as dependências.                                     | Define quais crates (como `actix-web`, `sqlx`, `bcrypt`, `aes-gcm`) estão disponíveis para todo o projeto.                                    |


