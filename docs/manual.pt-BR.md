# Manual

[English](manual.md) · **Português** · [Español](manual.es.md)

Tudo o que o emulador faz e como usar. Download, compilação e controles padrão estão no [README](../README.pt-BR.md).

As referências entre colchetes ([N2], [M1]…) estão em [Referências](references.pt-BR.md).

## Recursos do emulador

![Biblioteca: tela de jogo](../screenshots/library-screens.png)
*Biblioteca: tela de jogo*

![Ficha do jogo](../screenshots/game-info.png)
*Ficha do jogo*

![Moldura: console e controles](../screenshots/game-console.png)
*Moldura: console e controles*

![Moldura: TV](../screenshots/game-tv.png)
*Moldura: TV*

![Depurador: memória](../screenshots/debugger-memory.png)
*Depurador: memória*

![Depurador: CPU](../screenshots/debugger-cpu.png)
*Depurador: CPU*

![Depurador: tiles](../screenshots/debugger-tiles.png)
*Depurador: tiles*

![Trapaças](../screenshots/cheats.png)
*Trapaças*

### Biblioteca

É o que a janela mostra quando não há jogo aberto (como no DuckStation e no PCSX2):
- grade com a **capa** (retrato, 7 × 2) ou com a **tela de título / tela de jogo**
  (paisagem 4:3, 6 × 3);
- busca, troca de pasta, e clique para abrir;
- **F5** (ou **Arquivo › Atualizar lista de jogos**) relê a pasta de ROMs, para jogos
  acrescentados, apagados ou renomeados com o emulador aberto;
- **primeira execução:** com a pasta sem jogos, a biblioteca mostra uma tela de
  boas-vindas, com o botão **Escolher a pasta de ROMs…** (ou o A do controle) e links para
  **jogos gratuitos e legais** (os homebrews do Inufuto, o PVong e a
  ROM de teste dos controles);
- clique direito: **Abrir** e **Ficha do jogo…**;
- os jogos são reconhecidos **pelo conteúdo** (CRC32), então o nome do arquivo não
  importa (`digdug.bin`, `jogo.7z`…);
- as capas vêm do **libretro-thumbnails**, baixadas em segundo plano e guardadas em
  cache. Os homebrews não estão lá: as deles vêm do
  [emulation-thumbnails](https://github.com/PSGM-Romhacking/emulation-thumbnails) do
  projeto, no mesmo formato. As duas são buscadas pelo nome No-Intro da ROM, como no RetroArch.

Formatos aceitos: `.pv1`, `.bin`, `.rom`, e esses mesmos dentro de `.zip` ou `.7z`. O `.pv1`
é uma extensão própria do PV-1000, para não confundir com ROMs de outros sistemas; o MAME
e os outros emuladores só aceitam `.bin`, então é melhor distribuir ROMs com `.bin`.

### Ficha do jogo

Título original em japonês, ano, publisher, desenvolvedora, código do cartucho, nome e
caminho completo do arquivo, e descrição, em inglês, português e espanhol. Todos os jogos
do banco têm ficha; as descrições dos homebrews do Inufuto vêm das páginas do Inufuto
(Aerial e Guntus não têm texto lá), as dos outros homebrews, dos próprios autores e
projetos, e Dirty Chameleon ainda não tem descrição. Vem do banco de jogos do projeto
(`assets/games/`), que começou a partir da lista de software do MAME (CC0). Formato e regras em
[`docs/sheets.pt-BR.md`](sheets.pt-BR.md). Os títulos japoneses usam uma fonte embutida (46 KB,
recorte da Noto Sans CJK), então aparecem em qualquer distro.

### Jogar

| Recurso | Detalhe |
|---|---|
| **Save states** | 9 slots por jogo, com miniatura do momento salvo e data/hora; o arquivo guarda a impressão digital da ROM e recusa estado de outro jogo |
| **Retroceder** | um estado a cada 2 quadros, até ~30 s (~30 MB); segure Backspace ou L2 |
| **Avanço rápido** | segurar (Espaço / R2) ou ligar e desligar; 2x, 3x, 4x ou 8x |
| **Pausar** | P por padrão (o Guide do controle abre o menu rápido, que também pausa); pausa sozinho ao perder o foco (opcional) e com menu ou diálogo aberto |
| **2 jogadores** | o 1º controle é o J1, o 2º é o J2; o teclado tem os dois (J2 em WASD) |
| **Controles remapeáveis** | cada ação tem uma tecla e uma entrada por controle, salva pelo nome do driver; qualquer controle que o sistema reconheça (`/dev/input/jsX` no Linux, evdev no FreeBSD, XInput via `gilrs` no Windows) |

### Vídeo

| Recurso | Detalhe |
|---|---|
| Janela 1x–6x | N pixels de tela por pixel do jogo, exatos inclusive no Wayland |
| Escala inteira | só múltiplos inteiros (pixels uniformes) ou ocupando a janela |
| Filtros | Nenhum, Scanlines, CRT |
| **Molduras** | preenchem as laterais em tela cheia: **TV** (o jogo ocupa a altura toda, a TV aparece dos lados), **Console** (o PV-1000 e os controles desenhados) e **Capa do jogo** (desfocada e escurecida) |
| Tela cheia | F11; a barra de menus some e volta quando o mouse encosta no topo |
| FPS e velocidade | indicador opcional |

### Som

Começa **mudo** (a onda quadrada é alta) e com volume em 30%. O **som suave** (passa-baixa
de ~4 kHz e bloqueio de DC) vem ligado e tira a aspereza da onda quadrada.

### Gravação

| Recurso | Detalhe |
|---|---|
| Captura de tela | F12: PNG 4x em `~/Imagens/Mil Capivaras/` |
| GIF | F9: paleta exata das 8 cores, 2x, ~30 fps |
| MP4 com som | F10: H.264 + AAC via `ffmpeg`, 4x, na taxa exata de 59,538 fps (59,299 no modelo de hardware) |

As gravações vão para `~/Vídeos/Mil Capivaras/` (são as pastas XDG de Imagens e Vídeos; o nome
segue o idioma do sistema, por exemplo `~/Pictures/` e `~/Videos/` num sistema em inglês). A
codificação roda numa thread separada, e fechar o programa no meio de uma gravação termina o arquivo antes de sair.

### Ferramentas

- **Depurador** (janela separada):
  - RAM ao vivo, no formato de editor hexadecimal: cabeçalho de colunas, bytes em dois
    grupos de 8, coluna ASCII, os bytes que mudaram no último quadro em destaque e um
    inspetor do byte escolhido (hex, decimal, com sinal, binário) com edição;
  - os 256 tiles como o chip os vê, com grade e o número do tile sob o mouse;
  - cartões com os registradores e flags do Z80, as interrupções, o estado do vídeo (cor
    da borda, banco de padrões, endereços do mapa e do PCG), os 3 canais de som com a
    frequência em Hz e as portas F8–FF;
  - estado (rodando / pausado), pausar, avançar 1 quadro e ligar ou desligar cada canal.
- **Trapaças**, em duas abas:
  - **Códigos:** os códigos do jogo (valor editável, valor atual ao vivo, **Gravar
    agora**, **Remover** e **Fixar**, que prende o valor a cada quadro) e, embaixo, o
    formulário de **novo código** (endereço, valor, nome; "Gravar agora" muda uma vez e o
    jogo continua dali, "Salvar como código" guarda na lista);
  - **Buscar na RAM:** 1. comece a busca; 2. jogue até o valor mudar e diga o que
    aconteceu (Aumentou, Diminuiu, Mudou, Não mudou, É igual a), repetindo até sobrarem
    poucos endereços; 3. **Usar** leva o endereço escolhido para a aba Códigos.
  - Valores em decimal (0 a 255) ou hexadecimal com `0x`; mostrados nos dois formatos.
  - **Pontuações costumam estar em BCD** (dois dígitos decimais por byte), então se busca
    com Aumentou / Não mudou, não com o número da tela. E o placar na tela só muda quando
    o jogo o redesenha. Exemplo testado, **Pooyan:** pontuação em 0xBB22 (baixo) e 0xBB23
    (alto), em dezenas de pontos; gravar 0x50 e 0x12 dá 12.500 pontos.
  - Salvas por jogo no formato `.cht` do RetroArch, que funciona também no RetroArch com o
    core libretro.

### Controle e modo sofá

Para jogar do sofá, na TV ou num portátil, sem teclado nem mouse:

- **Menu rápido, o "Esc" do controle:** abre e fecha em qualquer lugar (biblioteca ou
  jogo, com ou sem modo sofá) com:
  - o botão **Home/Guide** (o botão central que muitos controles têm), por padrão. É a ação
    **Menu rápido** em **Emulação › Configurar controles e atalhos…**, e pode ir para
    qualquer outro botão ou tecla;
  - **Select + Start segurados juntos por meio segundo**, que servem para controles sem
    Home. Enquanto você segura, aparece "Continue segurando Select + Start…". Dá para
    desligar em **Emulação › Select + Start (segurar) abre o menu**;
  - o **Esc**, no modo sofá.
  O menu pausa o jogo e oferece: Continuar, Salvar estado, Carregar estado, Reiniciar,
  Voltar à biblioteca, **Modo sofá** (é por aqui que se liga pelo controle) e Sair. Na
  biblioteca as entradas são Continuar, Imagem da biblioteca, Modo sofá e Sair; no modo
  sofá, Modo sofá vira **Sair do modo sofá**. O direcional escolhe, o botão de baixo (A no
  Xbox) confirma, o da direita (B) volta.
- **Biblioteca pelo controle:** o direcional (ou o analógico esquerdo, ou as setas) move
  uma seleção entre as capas, com borda de foco; segurar repete. O botão de baixo abre o
  jogo; **Select tocado sozinho** troca a imagem (capa, título, jogo). O Start não é usado
  para navegar: ele fica livre para a combinação e para o jogo.
- **Modo sofá** (**Vídeo › Modo sofá**, ou `--couch` na linha de comando): tela cheia, sem
  a barra de menus, capas maiores e uma linha de ajuda com os botões (durante o jogo, só
  por alguns segundos, para não cobrir a imagem). Fica salvo nas preferências até ser
  desligado no menu rápido.
- **O botão não "vaza" para o jogo:** enquanto Select + Start estão segurados juntos, o
  jogo não os recebe; e depois de abrir um jogo ou fechar o menu pelo controle, a entrada
  do jogo fica ignorada até todos os botões serem soltos.

![Biblioteca no modo sofá](../screenshots/couch-library.png)
*Biblioteca no modo sofá*

![Menu rápido](../screenshots/couch-menu.png)
*Menu rápido sobre o jogo*

### Interface

- Barra de menus no padrão de desktop (Arquivo, Emulação, Áudio, Vídeo, Gravação,
  Ferramentas, Ajuda), cores cinza-escuro do Blender, sem cantos arredondados.
- **Idiomas:** inglês (padrão), português e espanhol, trocados ao vivo.
- Preferências, pasta de ROMs, jogos recentes, teclas e controles ficam salvos.
- Ícone próprio, atalho no menu de aplicativos e lançador na pasta (`make install`).

## Instalação e uso

### FreeBSD

No FreeBSD o programa usa as bibliotecas do sistema, instaladas uma vez com:

```sh
pkg install alsa-lib alsa-plugins libxkbcommon gtk3 zenity
```

O `zenity` (ou o `xdg-desktop-portal-gtk`) mostra os seletores de pasta e de arquivo; sem
ele, "Escolher a pasta de ROMs…" não abre nada. Depois é só descompactar o `.tar.gz` e rodar
`./mil-capivaras` dentro da pasta `mil-capivaras-0.9.0-freebsd-x86_64/` que ele cria. Ele é compilado numa VM FreeBSD 14.3 pelo mesmo workflow de release e foi
testado no FreeBSD 15.1 (XFCE): imagem, biblioteca, janelas, seletores de pasta e um
controle (Flydigi Vader 3). O som no FreeBSD ainda não foi testado.

**Controles no FreeBSD** são lidos de `/dev/input/event*` (evdev), a interface dos drivers
`hgame` (gamepads HID genéricos) e `xb360gp` (tipo Xbox 360). Por padrão só o root lê esses
dispositivos, então dê acesso ao seu usuário uma vez: ponha-o no grupo `operator`
(`pw groupmod operator -m $USER`) e, como root, acrescente esta regra:

```sh
printf "[localrules=10]\nadd path 'input/event*' mode 0660 group operator\n" >> /etc/devfs.rules
sysrc devfs_system_ruleset=localrules
service devfs restart
```

Sem ela, a tela de controles avisa "sem permissão para ler /dev/input". Os controles saem
com a mesma numeração de botões e o mesmo mapeamento padrão do Linux, então os
mapeamentos valem nos dois.

**Entre os BSDs, só o FreeBSD é suportado.** OpenBSD, NetBSD e DragonFly BSD são sistemas
diferentes: o programa do FreeBSD não roda neles, e não há compilação para eles (o OpenBSD
ainda precisaria de outra saída de som, porque não tem ALSA). Os desktops baseados no
FreeBSD, como o GhostBSD e o NomadBSD, devem funcionar, mas não foram testados. O FreeBSD
é de longe o BSD mais usado: cerca de 95% dos sites que rodam algum BSD
([W3Techs](https://w3techs.com/technologies/details/os-bsd/all/all), 2026).

### O que é preciso para rodar

Só o arquivo **`mil-capivaras-0.9.0-linux-x86_64`** (nome, versão, sistema e arquitetura). Tudo vai embutido nele: ícone, traduções, banco de jogos, fonte japonesa, tema e molduras. Ao rodar, ele cria:

| Pasta | Conteúdo |
|---|---|
| `~/.config/mil_capivaras/config.json` | preferências, pasta de ROMs, recentes, teclas e controles |
| `~/.config/mil_capivaras/states/` | save states e miniaturas |
| `~/.config/mil_capivaras/cheats/` | trapaças (`.cht`) |
| `~/.config/mil_capivaras/covers/` | suas próprias capas (opcional, veja abaixo) |
| `~/.config/mil_capivaras/languages/` | suas próprias traduções (opcional, veja abaixo) |
| `~/.config/mil_capivaras/cache/covers/` | capas baixadas (pode apagar à vontade) |
| `~/Imagens/Mil Capivaras/`, `~/Vídeos/Mil Capivaras/` | capturas e gravações |

Tudo, menos capturas e gravações, fica em **uma pasta só**: para backup ou sincronização
(Syncthing, Nextcloud…) basta `~/.config/mil_capivaras/`. No Windows ela é
`%APPDATA%\mil_capivaras\`, com a mesma estrutura. As versões de desenvolvimento anteriores à 0.9 separavam
os dados no padrão XDG (`~/.local/share/mil_capivaras/` e `~/.cache/mil_capivaras/`); na
primeira execução, o emulador os move sozinho para a pasta nova, sem sobrescrever nada.

### Modo portátil

Crie um arquivo vazio chamado **`portable.txt`** ao lado do AppImage ou do `.exe`, e tudo
(preferências, saves, trapaças, capas, idiomas, capturas e gravações) passa a ficar numa
pasta **`mil_capivaras_data/`** ao lado dele, e não na pasta do usuário:

```
Pendrive/MilCapivaras/
├── mil-capivaras-0.9.0-linux-x86_64.AppImage
├── portable.txt
├── roms/
└── mil_capivaras_data/
```

Os caminhos dentro da pasta do programa (a pasta de ROMs, os jogos recentes, as pastas
alteradas) são salvos relativos a ela, então tudo continua funcionando quando o pendrive é
montado em outro lugar ou em outro PC. Serve para pendrive, pasta sincronizada ou o
computador de outra pessoa (nada é gravado no perfil do usuário). Se a pasta não aceitar
gravação (um CD, um compartilhamento somente leitura), o emulador avisa ao iniciar e usa a
pasta normal. Uma cópia portátil começa do zero: não traz os dados da instalação normal.

### Pastas, capas próprias e idiomas novos

**Emulação › Pastas…** lista todas as pastas (ROMs, save states, trapaças, capas próprias,
idiomas, capturas, gravações), cada uma com **Abrir** (no gerenciador de arquivos),
**Alterar…** e **Padrão**. Com "Ao mudar uma pasta, levar os arquivos junto" ligado
(o padrão), mudar uma pasta leva os arquivos junto; um arquivo com o mesmo nome que já
esteja na pasta nova nunca é sobrescrito. Uma pasta alterada que não exista ao iniciar (um
disco desconectado) gera um aviso, e a padrão é usada naquela sessão, sem esquecer a sua
escolha.

**Capas próprias.** Ponha PNGs na pasta de capas com a estrutura do
[libretro-thumbnails](https://github.com/libretro-thumbnails/libretro-thumbnails):
`Named_Boxarts/` (capa), `Named_Titles/` (tela de título) e `Named_Snaps/` (jogo). O nome
do arquivo é o nome No-Intro da ROM (`Pooyan (Japan).png`) ou o nome que aparece embaixo
do cartão na biblioteca (`Cacorm.png`). As suas imagens têm prioridade sobre as baixadas,
então também servem para trocar uma capa de que você não gosta. Um clone de um
repositório no formato do libretro-thumbnails funciona direto.

**Idiomas novos.** **Ajuda › Language / Idioma › Exportar modelo de tradução…** salva um JSON com
todos os textos da interface, cada um apontando para ele mesmo (o texto em inglês), mais
`"@name"`. Traduza os valores, escreva o nome do idioma na própria língua em `"@name"`
("Français"), dê ao arquivo o nome do código do idioma (`fr.json`, `pt-PT.json`), ponha-o
na pasta de idiomas e reinicie: o idioma aparece em **Ajuda › Language / Idioma**. Um valor vazio quer
dizer "ainda não traduzido" (mostra o texto em inglês). Um arquivo com o nome de um idioma
embutido (`pt.json`, `es.json`) corrige só os textos que contém.

O guia completo, com as regras do formato, uma lista de telas para testar e **como enviar
uma tradução para entrar na próxima versão**, está em
[`docs/translating.pt-BR.md`](translating.pt-BR.md).

### Linha de comando

```bash
./mil-capivaras-0.9.0-linux-x86_64                              # abre a biblioteca
./mil-capivaras-0.9.0-linux-x86_64 "Dig Dug (Japan).7z"         # abre um jogo (.pv1, .bin, .rom, .zip, .7z)
./mil-capivaras-0.9.0-linux-x86_64 "/caminho/Casio - PV-1000/"  # abre a biblioteca nessa pasta
./mil-capivaras-0.9.0-linux-x86_64 jogo.bin --scale 4           # janela em 4x
./mil-capivaras-0.9.0-linux-x86_64 --couch                      # modo sofá (fica salvo)
./mil-capivaras-0.9.0-linux-x86_64 --identify PASTA             # CRC32 e jogo do MAME de cada ROM
./mil-capivaras-0.9.0-linux-x86_64 --info JOGO                  # ficha do jogo no terminal
./mil-capivaras-0.9.0-linux-x86_64 --controls                   # controles detectados e botões apertados
./mil-capivaras-0.9.0-linux-x86_64 --memory-map                 # mapa de memória (formato RetroAchievements)
./mil-capivaras-0.9.0-linux-x86_64 JOGO --bench 3000            # velocidade, sem janela
./mil-capivaras-0.9.0-linux-x86_64 JOGO --shot 500 q.ppm --press-start --dump-ram ram.bin  # validação
./mil-capivaras-0.9.0-linux-x86_64 JOGO --shot 500 q.ppm --hardware                         # o mesmo, no modelo de hardware real
```
