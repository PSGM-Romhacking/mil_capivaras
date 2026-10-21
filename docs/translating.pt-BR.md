# Traduzindo o Mil Capivaras

[English](translating.md) · **Português** · [Español](translating.es.md)

O Mil Capivaras vem em inglês, português e espanhol. Qualquer outro idioma pode ser
acrescentado **sem programar e sem compilar nada**: a tradução é um único arquivo JSON que o
emulador lê ao iniciar. Você pode usá-la só para você ou enviá-la para entrar na próxima
versão, para todo mundo.

## Sumário

1. [O que é preciso](#1-o-que-é-preciso)
2. [Passo a passo](#2-passo-a-passo)
3. [O formato do arquivo](#3-o-formato-do-arquivo)
4. [Regras que mantêm a interface funcionando](#4-regras-que-mantêm-a-interface-funcionando)
5. [Testando a tradução](#5-testando-a-tradução)
6. [Corrigindo um idioma que já existe](#6-corrigindo-um-idioma-que-já-existe)
7. [Quando uma versão nova traz textos novos](#7-quando-uma-versão-nova-traz-textos-novos)
8. [Enviando para entrar no emulador](#8-enviando-para-entrar-no-emulador)
9. [Fichas dos jogos (opcional)](#9-fichas-dos-jogos-opcional)
10. [Dúvidas](#10-dúvidas)

## 1. O que é preciso

- O Mil Capivaras **0.9.0 ou mais novo** (qualquer sistema).
- Um editor de texto que salve em **UTF-8** (qualquer um atual: VS Code, Notepad++, Kate,
  gedit, micro…). Evite processadores de texto como o Word ou o LibreOffice Writer.
- Umas duas a quatro horas: são cerca de 330 textos, quase todos curtos (itens de menu,
  botões).

## 2. Passo a passo

1. **Exporte o modelo.** No emulador, abra **Ajuda › Language / Idioma › Exportar modelo
   de tradução…** e salve o arquivo. Ele já é oferecido na pasta de idiomas.
2. **Dê ao arquivo o nome do código do idioma**:
   `fr.json` (francês), `de.json` (alemão), `it.json` (italiano), `ja.json` (japonês),
   `pt-PT.json` (português de Portugal), `zh-CN.json` (chinês simplificado)… É o nome que
   diz ao emulador qual idioma é. Use só letras, números, `-` e `_`, até 16 caracteres.
3. **Traduza.** Abra o arquivo no editor e troque cada valor (o texto da direita) pela
   tradução. Os textos da esquerda ficam como estão. Detalhes nas duas próximas seções.
4. **Escreva o nome do idioma** em `"@name"`, do jeito que quem fala o idioma escreve:
   `"Français"`, `"Deutsch"`, `"日本語"`. É isso que aparece no menu de idiomas.
5. **Ponha o arquivo na pasta de idiomas** e reinicie o emulador. Para achar a pasta:
   **Emulação › Pastas…**, linha **Idiomas**, botão **Abrir**. Por padrão ela é:

   | Sistema | Pasta de idiomas |
   |---|---|
   | Linux, FreeBSD | `~/.config/mil_capivaras/languages/` |
   | Windows | `%APPDATA%\mil_capivaras\languages\` |
   | Modo portátil | `mil_capivaras_data/languages/`, ao lado do programa |

6. **Escolha o idioma** em **Ajuda › Language / Idioma**. O seu aparece depois dos
   embutidos.

Para usar só você, é isso. Para compartilhar, veja a [seção 8](#8-enviando-para-entrar-no-emulador).

## 3. O formato do arquivo

O modelo é uma lista de pares `"texto original": "tradução"`. Recém-exportado, cada
tradução é uma cópia do texto em inglês:

```json
{
  "@name": "Language name",
  "Open game…": "Open game…",
  "Search {n} games…": "Search {n} games…",
  "Screenshot saved: {file}": "Screenshot saved: {file}"
}
```

Traduzido para o francês, fica:

```json
{
  "@name": "Français",
  "Open game…": "Ouvrir un jeu…",
  "Search {n} games…": "Rechercher parmi {n} jeux…",
  "Screenshot saved: {file}": "Capture enregistrée : {file}"
}
```

- **O lado esquerdo é a chave**: o texto em inglês exatamente como está no programa. Nunca
  o altere, senão o emulador não encontra a tradução e mostra o inglês.
- **O lado direito é o que aparece.** Pode ficar como está (igual ao inglês) ou ser
  **esvaziado** (`""`), o que quer dizer "ainda não traduzido": aparece o inglês no lugar.
  Assim um arquivo incompleto já funciona, e dá para traduzir em várias sessões.
- **A ordem das linhas não importa**, e linhas podem ser apagadas (mostram o inglês).

## 4. Regras que mantêm a interface funcionando

1. **Mantenha os campos entre chaves** exatamente como estão: `{n}`, `{name}`, `{file}`,
   `{path}`, `{error}`, `{system}`… O programa os troca por um número, um nome ou um
   caminho. Eles podem mudar de lugar na frase (`"{n} jeux"`), mas não podem ser traduzidos
   nem apagados.
2. **Mantenha as reticências "…"** no fim dos itens de menu que abrem uma janela
   (`"Folders…"` → `"Dossiers…"`). É uma convenção: "…" quer dizer "abre algo em que você
   vai escolher".
3. **Mantenha o "›"** nos caminhos de menu (`Emulation › Controls and shortcuts…`) e
   traduza os nomes dos dois lados do mesmo jeito que traduziu esses menus, para que dê para
   segui-los.
4. **Mantenha o JSON válido:** cada linha termina com vírgula, menos a última; aspas dentro
   de um texto se escrevem `\"`; uma quebra de linha dentro de um texto é `\n`. Se o arquivo
   tiver um erro, o emulador mostra uma mensagem ao iniciar com o nome do arquivo e o motivo,
   e ignora esse arquivo (os outros carregam normalmente).
5. **Textos curtos continuam curtos.** Itens de menu e botões têm pouco espaço. Se a
   tradução ficar bem mais longa que o inglês, procure uma palavra mais curta.
6. **Não traduza** nomes próprios: Mil Capivaras, Casio, PV-1000, RetroArch, libretro,
   RetroAchievements, MiSTer, MAME, NESdev, os formatos de arquivo (`.pv1`, `.cht`, PNG,
   MP4, GIF), os nomes de teclas (`Ctrl`, `Esc`, `F2`) e os nomes de pasta
   `Named_Boxarts`, `Named_Titles` e `Named_Snaps`.
7. **Termos técnicos:** use o que os jogadores do seu idioma realmente falam. "Save state",
   "ROM", "cheat" e "rewind" muitas vezes ficam em inglês; na dúvida, veja como o RetroArch
   foi traduzido para o seu idioma e siga o mesmo.
8. **Os rótulos de teclado e controle** (`KEYBOARD`, `CONTROLLER`, `EMULATOR`…) são
   cabeçalhos de coluna em maiúsculas: mantenha em maiúsculas.

## 5. Testando a tradução

Depois de reiniciar com o seu idioma escolhido, passe por estas telas; juntas, elas mostram
quase todos os textos:

- [ ] todos os menus da barra de menus, inclusive os submenus;
- [ ] a biblioteca (busca, as abas de imagem, clique direito num jogo, a tela de
      boas-vindas com a pasta vazia);
- [ ] um jogo rodando: as mensagens no topo (save state, captura, avanço rápido);
- [ ] **Emulação › Configurar controles e atalhos…** e **Ajuda › Controles**;
- [ ] **Emulação › Pastas…**;
- [ ] **Ferramentas › Depurador…** e **Ferramentas › Trapaças…** (as duas abas);
- [ ] **Ajuda › Ficha do jogo…** e **Ajuda › Sobre o Mil Capivaras**;
- [ ] o menu rápido (botão Home/Guide do controle, ou **Vídeo › Modo sofá** e Esc).

Procure palavras cortadas, textos que passam do tamanho do botão e inglês que ficou para
trás (uma chave alterada sem querer ou um valor ainda vazio).

## 6. Corrigindo um idioma que já existe

Para corrigir alguns textos de um idioma embutido (português ou espanhol), não precisa do
modelo inteiro: crie `pt.json` ou `es.json` na pasta de idiomas **só com as linhas que quer
mudar**. Os seus textos tomam o lugar dos embutidos; o resto continua igual. O `"@name"` é
ignorado nos idiomas embutidos. Se a correção valer para todo mundo, envie também
([seção 8](#8-enviando-para-entrar-no-emulador)).

## 7. Quando uma versão nova traz textos novos

Versões novas acrescentam e mudam textos. Para atualizar uma tradução:

1. Exporte o modelo de novo, pela versão nova.
2. Compare com o seu arquivo. Uma ferramenta de comparação de texto ajuda: a tela
   "comparar arquivos" do VS Code, o Meld, o WinMerge ou o `diff`. As linhas que só existem
   no modelo novo são os textos novos.
3. Copie as linhas novas para o seu arquivo e traduza. As linhas que só existem no seu
   arquivo são textos que o programa não usa mais: não atrapalham, mas podem ser apagadas.

Enquanto isso não é feito, os textos novos simplesmente aparecem em inglês.

## 8. Enviando para entrar no emulador

As traduções enviadas entram na próxima versão, embutidas no programa, com crédito a quem
traduziu no README e nas notas da versão.

**Como enviar**, em ordem de preferência:

1. **Uma issue no GitHub:** abra
   <https://github.com/PSGM-Romhacking/mil_capivaras/issues/new>, com o título
   `Translation: <idioma>` (por exemplo `Translation: Français`), e anexe o arquivo `.json`
   (se o GitHub recusar o anexo, compacte antes em `.zip`). Diga como quer ser creditado:
   nome ou apelido e, se quiser, um link.
2. **Um pull request**, se você usa Git: ponha o arquivo em `assets/i18n/<código>.json` (um
   catálogo por idioma), registre o idioma em `LANGS` e `BUILTIN` em
   `crates/frontend/src/i18n.rs` e abra o pull request.

**Antes de enviar**, confira:

- [ ] o nome do arquivo é o código do idioma (`fr.json`);
- [ ] o `"@name"` está preenchido com o nome do idioma na própria língua;
- [ ] o emulador carrega o arquivo sem mensagem de erro, e as telas da
      [seção 5](#5-testando-a-tradução) estão certas;
- [ ] ele foi exportado da **versão mais recente** (ou atualizado como na
      [seção 7](#7-quando-uma-versão-nova-traz-textos-novos)).

Tradução parcial também é bem-vinda: diga isso na issue, e os textos que faltam aparecem em
inglês até alguém completar.

**Licença:** o Mil Capivaras é GPL-3.0 ou posterior. Ao enviar uma tradução, você concorda que ela seja
distribuída sob a mesma licença, como o resto do programa.

## 9. Fichas dos jogos (opcional)

As descrições de **Ajuda › Ficha do jogo…** (do que o jogo trata, observações) não estão
no modelo: fazem parte do banco de jogos, um arquivo por idioma, no repositório, em
`assets/games/` (`games.en.json`, `games.pt.json`, `games.es.json`). Num idioma sem
elas, o emulador as mostra em inglês, então traduzi-las não é necessário para a interface
ficar completa. (O número de jogadores não é texto: as frases dele, "1 player" e "1 or {n}
players", estão no modelo da interface.)

As fichas são opcionais se você só usa a tradução. Para embutir o idioma no programa, porém,
é preciso um `games.<código>.json` (pode começar como cópia do `games.en.json`), listado em
`TEXTS` em `crates/core/src/games.rs`: os testes exigem um para cada idioma embutido.

Para traduzi-las, copie `games.en.json` para `games.<código>.json`, traduza só os
valores de `"description"` e `"notes"` (mantenha as chaves dos jogos, como `"pooyan"`, e os
nomes dos campos) e envie na mesma issue. Elas vão embutidas no programa, então só
aparecem a partir da versão seguinte.

## 10. Dúvidas

Abra uma issue no GitHub com o título `Translation question: <idioma>`. Dúvidas sobre o
sentido de um texto são bem-vindas: alguns são curtos e dependem da tela em que aparecem.
