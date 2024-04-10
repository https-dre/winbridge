# windows-filesystem-for-wsl

Criar um App de Linha de Comando Open-Source para interação de arquivos do Windows dentro do próprio WSL. Por enquanto as primeiras funcionalidades a serem adicionadas são:

* Copiar arquivos sem ter que digitar um path muito grande, por exemplo:

        cp /mnt/c/Users/GitHubUser/Downloads/app/bin/dist/linux ./
    
* Navegar rapidamente entre os dois sistemas operacionais

* Copiar Pastas

* Outras

## É Open-Source

Fique a vontade para dar sugestões de novas funcionalidades, estou aberto a novas ideias. Basta a abrir a sua Issue e efetuar seu Pull Request.

**Ainda está completamente em fase de desenvolvimento.** Não há nenhuma dessas funcionalidades que citei acima, também é um projeto que criei com o objetivo de aprender Rust. Por isso, provavelmente o código pode conter erros e muitas coisas a melhorar. Por isso, não exite em abrir sua Issue para efetuar correções.

## Para testes

Lembre-se de criar um arquivo .fs-config na raiz do projeto, esse arquivo vai conter variáveis de configuração do app. Por enquanto só existe uma configuração:

```
windows_user_path="/Users/Seu usuário"
```