# img-tool

`img-tool` é uma ferramenta de linha de comando escrita em **Rust** para processamento em lote de imagens.

Ela permite:

- Converter o formato das imagens (ex.: **PNG ↔ JPG**, WebP → JPG/PNG)
- Redimensionar imagens (`--resize LARGURAxALTURA`, ex.: `800x600`)
- Converter para **tons de cinza** (`--grayscale`)
- Gerar um **relatório em JSON** com informações das imagens processadas
  - caminho de entrada e saída
  - formato original e novo
  - tamanho do arquivo antes e depois

A entrada pode ser **um arquivo único** ou **um diretório** com várias imagens.  
A saída são as imagens processadas em um diretório de saída (por padrão, `output/`) e, opcionalmente, um arquivo JSON com o resumo.

---

## Tecnologias utilizadas

- Linguagem: **Rust**
- Gerenciador de dependências / build: **Cargo**

Principais crates (bibliotecas de terceiros) usadas:

- [`clap`](https://crates.io/crates/clap)  
  Parser de argumentos de linha de comando. Lê opções como `--to-format`, `--resize`, `--grayscale`, `--report` e gera automaticamente `--help` e `--version`.

- [`image`](https://crates.io/crates/image)  
  Leitura, manipulação e escrita de imagens em vários formatos (PNG, JPG/JPEG, WebP, etc.). No projeto, é responsável por:
  - abrir os arquivos de imagem
  - redimensionar
  - converter para tons de cinza
  - salvar no novo formato

- [`walkdir`](https://crates.io/crates/walkdir)  
  Percorre diretórios recursivamente. Permite que o usuário passe uma pasta como entrada e o programa descubra todos os arquivos dentro dela.

- [`serde`](https://crates.io/crates/serde) e [`serde_json`](https://crates.io/crates/serde_json)  
  Serializam os dados de relatório (struct em Rust) para JSON, gerando um arquivo de resumo legível por pessoas e por outras aplicações.

- [`anyhow`](https://crates.io/crates/anyhow)  
  Facilita o tratamento de erros, permitindo usar `Result<T, anyhow::Error>` e o operador `?` de forma mais simples.

Todo o gerenciamento dessas dependências é feito automaticamente pelo **Cargo**.

---

## Como executar localmente

### Pré-requisitos

- **Rust** e **Cargo** instalados  
  (ver: <https://www.rust-lang.org/tools/install>)

### Clonar o repositório

```bash
git clone https://github.com/<seu-usuario>/versionamento-imgtool.git
cd versionamento-imgtool


## Executar com cargo RUN
```bash
    cargo run -- <entrada> [opções]


###Exemplos:

# Converter um único arquivo PNG para JPG
cargo run -- imagem.png --to-format jpg

# Converter um único arquivo WebP para PNG
cargo run -- imagem.webp --to-format png

# Processar todas as imagens de uma pasta, convertendo para JPG
cargo run -- ./imagens --to-format jpg

# Converter, redimensionar e gerar relatório JSON
cargo run -- ./imagens \
  --to-format jpg \
  --resize 800x600 \
  --report relatorio.json

# Converter para tons de cinza e salvar no diretório de saída customizado
cargo run -- ./imagens \
  --to-format png \
  --grayscale \
  --output saida
