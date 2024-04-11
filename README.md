# windows-bridge

Um App de Linha de Comando Open-Source para interação de arquivos do Windows dentro do próprio WSL. Por enquanto as primeiras funcionalidades a serem adicionadas são:

* Copiar arquivos e Pastas

* Criar Arquivos e Pastas

* Remover arquivos e pastas

* Navegar rapidamente entre os dois sistemas operacionais

O objetivo é facilitar o uso do terminal, para que o usuário de WSL não tenha que digitar coisas como o exemplo abaixo, caso queira copiar algo que está no Windows.

        cp /mnt/c/Users/GitHubUser/Downloads/app/bin/dist/linux ./

## É Open-Source

Fique a vontade para dar sugestões de novas funcionalidades, estou aberto a novas ideias. Basta a abrir a sua Issue e efetuar seu Pull Request.

**Ainda está completamente em fase de desenvolvimento.** Não há nenhuma dessas funcionalidades que citei acima, também é um projeto que criei com o objetivo de aprender Rust. Por isso, provavelmente o código pode conter erros e muitas coisas a melhorar. Por isso, não exite em abrir sua Issue para efetuar correções.

## Para testes

Lembre-se de criar um arquivo .fs-config na raiz do projeto, esse arquivo vai conter variáveis de configuração do app. Por enquanto só existe uma configuração:

```
windows_user_path="/Users/Seu usuário"
```