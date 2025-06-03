# 🖥️ AtmaDesk

> Controle remoto P2P rápido, leve e seguro — construído sobre o poder do [RustDesk](https://github.com/rustdesk/rustdesk), com uma interface personalizada e melhorias de usabilidade.

---

## 📘 Sobre o Projeto

O **AtmaDesk** nasceu a partir da base sólida do projeto open-source **RustDesk**, com o objetivo de oferecer uma solução de controle remoto moderna, customizável e com uma cara nova — pensada especialmente para empresas e usuários que desejam independência e performance.

Este projeto mantém a arquitetura **P2P (peer-to-peer)**, evitando servidores intermediários sempre que possível, garantindo **latência mínima**, **alta segurança** e **maior privacidade**.

---

## ⚙️ Tecnologias Utilizadas

- 🦀 **Rust** — o núcleo de performance e segurança do AtmaDesk.
- 🪟 **Sciter** — framework leve para interfaces gráficas em HTML/CSS/JS.
- 🔧 **vcpkg** — gerenciador de pacotes para dependências C++.
- 📡 **P2P e NAT traversal** — conexão direta entre cliente e host, com fallback para relay caso necessário.
- 🔐 **Criptografia ponto a ponto (E2EE)** — seus dados sempre seguros.

---

## 📁 Pré-requisitos

Antes de compilar o AtmaDesk, certifique-se de ter:

- [Rust (cargo)](https://www.rust-lang.org/tools/install)
- [vcpkg](https://vcpkg.io/en/index.html) instalado e configurado no seu PATH.
- **Visual Studio Build Tools** (com suporte a C++).
- A biblioteca `sciter.dll` — necessária para execução da GUI.

> 🔸 **Importante:** o arquivo `sciter.dll` **não é incluído** no repositório por questões de licenciamento. Você deve baixá-lo manualmente.

---

### 🔽 Baixando a `sciter.dll`

1. Acesse: [https://sciter.com/download/](https://sciter.com/download/)
2. Baixe a versão **Windows 64-bit** (`bin.zip`)
3. Extraia e copie o arquivo `sciter.dll` para a mesma pasta do executável compilado.

---

## 🛠️ Como Rodar Localmente

```bash
# Clone o repositório
git clone https://github.com/DanielVitorio/AtmaDesk.git
cd AtmaDesk

# Configure as dependências via vcpkg (ex: openssl, libsodium, etc.)
vcpkg install openssl:x64-windows libsodium:x64-windows

# Compile o projeto com cargo
cargo build --release --features inline # Para compilar com inline assembly
```

> ❗ Certifique-se de que o comando `vcpkg integrate install` foi executado e que as variáveis de ambiente do `vcpkg` estão ativas.

---

### 📂 Pós-compilação

Após compilar, copie a `sciter.dll` para o diretório do executável

Agora, execute o AtmaDesk normalmente:

```bash
cd target\release
atmadesk.exe
```

---

## 🌐 Conexão P2P e Segurança

O AtmaDesk utiliza uma rede peer-to-peer que permite conexões diretas entre dispositivos, sempre que possível. Isso significa:

- 🚀 **Velocidade máxima**, com o menor ping possível.
- 🔐 **Criptografia ponta a ponta** (E2EE).
- 🌍 **Sem depender de servidores centrais** — ideal para uso privado ou em redes internas.

> Quando necessário, o AtmaDesk faz uso de um servidor relay (opcional e configurável) como fallback para NATs mais restritivas.

---

## 🧩 Personalização da Interface

A interface do AtmaDesk foi totalmente reimaginada usando **Sciter**, permitindo que você:

- Crie temas personalizados com HTML e CSS.
- Altere botões, janelas e comportamento com facilidade.
- Use JavaScript no frontend para ampliar funcionalidades.

Ideal para empresas que desejam **interface com branding próprio**.

---

## 🤝 Baseado em RustDesk

Este projeto se inspira diretamente no excelente [RustDesk](https://github.com/rustdesk/rustdesk), uma alternativa open-source ao TeamViewer e AnyDesk.

Nosso diferencial:

- Interface com design próprio;
- Facilidade de compilação;
- Integração amigável com redes locais e corporativas;
- Foco em leveza e portabilidade.

---

## 📜 Licença

Este projeto segue os termos da licença **AGPL-3.0**, exceto onde explicitamente indicado.

> ⚠️ O uso da biblioteca `sciter.dll` está sujeito à [licença própria da Sciter](https://sciter.com/licensing/).

---

## 🚀 Autor

**Daniel Vitório Augusto**  
Desenvolvedor e idealizador do AtmaDesk  
📧 [danielvitorio.augusto@gmail.com]

---

## 💬 Agradecimentos

- Ao projeto **RustDesk**, por tornar o controle remoto moderno e open-source possível.
- À comunidade open-source de Rust e Sciter pelo ecossistema incrível.
