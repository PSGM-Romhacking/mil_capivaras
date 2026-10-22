# Arquitetura e funcionamento

[English](architecture.md) · **Português** · [Español](architecture.es.md)

Como o código está organizado, como a emulação funciona e por que cada decisão foi tomada.

As referências entre colchetes ([N2], [M1]…) estão em [Referências](references.pt-BR.md).

## Crates

Um *workspace* Cargo com o núcleo do console separado da interface: a interface serve
para **outros emuladores**, e o núcleo vira **core libretro** sem reescrever nada.

```
crates/api/              contrato núcleo ↔ interface (sem dependências)
crates/core/             o console; só depende de emu-api, iz80 e serde/serde_json
crates/frontend/         a interface reutilizável; não conhece nenhum console
crates/app/              o executável: núcleo + interface + identidade do app
crates/libretro/         o núcleo atrás da API C do libretro (mil_capivaras_libretro.so)
vendor/iz80/             a CPU Z80, com ciclos e registrador R corrigidos
assets/                  dados embutidos: ícones, molduras, fontes, traduções, banco de jogos
docs/                    a documentação (lista no README)
reference/               o core do MiSTer simulado com o Verilator, para validar (fora do build)
```

```
                 ┌──────────────────────┐
                 │      crates/app      │  marca, traduções do app, linha de comando
                 └───────┬───────┬──────┘
                         │       │
        ┌────────────────▼─┐   ┌─▼─────────────────────┐
        │  emu-frontend    │   │  pv1000-core          │
        │  (egui, áudio,   │   │  machine.rs (console) │
        │  controles, …)   │   │  core.rs (adaptador)  │
        └────────┬─────────┘   └──┬──────────┬─────────┘
                 │                │          │
                 └──────┬─────────┘          │
                   ┌────▼────┐          ┌────▼────┐
                   │ emu-api │          │  iz80   │ (vendor/)
                   └─────────┘          └─────────┘
        libretro ──► pv1000-core + emu-api (sem egui)
```

## `emu-api`: o contrato

- O **trait `Core`**: `load`, `reset`, `run_frame(&[u16])`, `video()`,
  `take_audio()`, `save_state()`, `load_state()`, `system_ram()` / `system_ram_mut()`,
  `debug_views()`, `sound_channels()`, `options()` / `set_option()` e `fps()` (a taxa de quadros
  atual, que no PV-1000 depende do modelo).
- O **`SystemInfo`**: nome, extensões, botões, resolução, proporção, FPS como fração
  exata, endereço da RAM, identificação por hash e fichas.
- Segue as **convenções do libretro**: controle como bitmask RetroPad na mesma ordem de
  bits, imagem XRGB8888, áudio f32 estéreo intercalado, estado em bytes opacos. Por isso
  o core libretro ([libretro.pt-BR.md](libretro.pt-BR.md)) é só uma tradução fina, e trocar
  de interface não mexe no núcleo.

### O contrato em detalhe

| Aspecto | Decisão |
|---|---|
| Controles | bitmask **RetroPad** por jogador (B, Y, Select, Start, ↑ ↓ ← →, A, X, L, R, L2, R2, L3, R3); o sistema declara quais usa e o nome de cada um |
| Vídeo | `&[u32]` **XRGB8888** + largura/altura/proporção |
| Áudio | **f32 estéreo intercalado**, taxa escolhida pela interface |
| Save state | bytes opacos (`retro_serialize`); cabeçalho do arquivo (marca, versão, hash da ROM; a miniatura é um PNG à parte) é da interface |
| Memória | `system_ram()` (`RETRO_MEMORY_SYSTEM_RAM`) |
| Metadados | `SystemInfo`: nome, extensões, nome no libretro-thumbnails, botões, FPS, resolução, identificação por hash |
| Taxa de quadros | `Core::fps()` (padrão: a do `SystemInfo`); o PV-1000 a muda conforme o modelo (59,538 ou 59,299 Hz) |
| Opções | `Core::options()` / `set_option()`, como as *core options* do libretro (modelo, imagem) |
| Depuração | `debug_views()`: imagens (com grade opcional) e tabelas em grupos (`DebugGroup`, `DebugValue`: texto, liga/desliga, cor) |

### Como a interface usa o contrato

- **Save state em arquivo:** `EFST` + versão (u32) + FNV-1a 64 da ROM + bytes do núcleo.
  Arquivo sem esse cabeçalho vai direto ao núcleo, que reconhece o formato antigo.
- **Controles:** o teclado e cada controle geram um `u16` RetroPad por jogador. O 1º
  controle é o jogador 1, o 2º o jogador 2…; ações "J2" de qualquer controle vão
  para o jogador 2. A tela de configuração mostra só os botões que o sistema declara.
- **Marca:** `emu_frontend::brand::set(Brand { name, id, version, icon_png, about, repo_url, bezels, free_games, thumbnails_url })`
  no início do `main` — `id` dá a pasta do app (`~/.config/<id>`, com tudo dentro; ver
  `config::Folder`), `name` as de Imagens/Vídeos e o título da janela.
- **Idiomas:** um catálogo por idioma, `assets/i18n/<código>.json`, com todos os textos da
  interface, do app e do sistema (botões, mapa de memória, "Sobre", linha de comando). A
  interface e o app têm cada um um teste que confere os próprios textos nele. O inglês é o
  texto-fonte, no código, sem arquivo (só `pt.json` e `es.json`).
- **Proporção e resolução** vêm do `SystemInfo` (`aspect` = largura/altura da tela).

## `pv1000-core`: o console

| Arquivo | Conteúdo |
|---|---|
| `machine.rs` | o PV-1000: estado, laço da CPU, interrupções e os dois laços de quadro (MiSTer e hardware); portado do `CasioPV1000.v` |
| `machine/bus.rs` | memória e portas como a CPU as vê, a matriz do joystick e os registradores de vídeo das portas FE e FF |
| `machine/video.rs` | o desenho: a linha atual é desenhada enquanto a CPU roda |
| `machine/timing.rs` | base de tempo e constantes de temporização dos dois modelos |
| `machine/sound.rs` | o gerador de som: 3 ondas quadradas, porta FB, volumes e filtros passa-alta |
| `machine/state.rs` | o save state e o formato do arquivo |
| `machine/debug.rs` | depuração: registro de interrupções, rastreio de instruções, registradores, memória e tiles |
| `machine/tests.rs` | os testes do console, inclusive contra as medições do Par |
| `core.rs` | adaptador para o `Core`: RetroPad → bits do MiSTer, mono → estéreo, visões de depuração, `INFO` |
| `memory.rs` | mapa de memória e leitura "plana" no formato do RetroAchievements |
| `games.rs` | o banco de jogos: identificação por CRC32, ano, editora, cartucho, jogadores |
| `assets/games/` (na raiz) | o banco (`games.json`) e os textos dos jogos (um JSON por idioma) |

Não abre janela, não toca som e não lê arquivos: é o que um core libretro exige.

## `emu-frontend`: a interface

| Arquivo | Conteúdo |
|---|---|
| `app.rs` | a janela: estado do app, criação, abrir e fechar jogo, preferências, o ciclo de cada quadro da interface |
| `app/emulation.rs` | o laço de emulação: ritmo, entrada, retroceder, avanço rápido, som |
| `app/input.rs` | teclado e controle: atalhos e mapeamentos |
| `app/nav.rs` | navegação pelo controle, menu rápido e modo sofá |
| `app/menus.rs`, `app/screen.rs` | a barra de menus; a tela do jogo com molduras e filtros |
| `app/library.rs` | a biblioteca, a identificação dos jogos e as fichas |
| `app/state.rs`, `app/recording.rs` | save states; capturas e gravações |
| `app/controls.rs`, `app/dialogs.rs` | o diálogo de controles; os demais diálogos |
| `app/folders.rs` | a janela **Emulação › Pastas…** |
| `app/docshots.rs` | o roteiro das capturas da documentação (`EMU_DOCSHOTS`) |
| `debugger.rs`, `cheats.rs` | as ferramentas, em janelas separadas |
| `gamepad.rs`, `keys.rs` | controles numa thread (`/dev/input/jsX` no Linux, `gilrs` no Windows), e teclas |
| `gamepad/evdev.rs` | controles no FreeBSD (evdev) |
| `audio.rs`, `recorder.rs` | saída de som (cpal) e gravação GIF/MP4 |
| `covers.rs`, `sheets.rs`, `rom.rs` | capas, fichas, leitura de `.zip`/`.7z` |
| `config.rs`, `i18n.rs`, `brand.rs` | preferências, idiomas, identidade do app |
| `platform.rs` | o que muda entre sistemas: pastas base, hora local, abrir uma pasta |
| `theme.rs`, `widgets.rs`, `menu.rs` | tema e componentes da interface |
| `achievements.rs` | ganchos para o RetroAchievements (ainda sem efeito) |

A interface não conhece o PV-1000: outro emulador seria outro app com o mesmo
`emu-frontend` e um núcleo que implemente `Core`.

## Testes

`cargo test --release --workspace` cobre, por crate:
- **API:** CRC32 padrão; bits do RetroPad na ordem do libretro.
- **Núcleo:** o console (aviso de quadro, `HALT`, modos de interrupção, barramento aberto,
  tom e passa-alta, matriz em OU e seu atraso, roubo de barramento, medições do Par); save
  state determinístico; o adaptador igual ao console direto; mapeamento RetroPad; FPS
  exato; imagem TV; o banco de jogos (lista do MAME, consistência, textos, homebrews sem
  preenchimento); RAM plana para o RetroAchievements.
- **Interface:** catálogos de tradução completos e idiomas do usuário; formato `.cht` de
  ida e volta; fichas nos 3 idiomas; glifos japoneses; gravação MP4 com som; mapeamento de
  controles, também por evdev; teclas; mover pastas e modo portátil; o cabeçalho do save
  state.
- **App:** catálogo do app completo; textos dos jogos válidos; molduras transparentes onde
  o jogo entra.
- **Core libretro:** a API em C igual ao núcleo direto (quadros, taxa de áudio, save
  state, RAM).

A cópia do `iz80` passa no ZEXALL.

## Novo emulador com a mesma interface

1. Crate `crates/core` implementando `emu_api::Core` + um `static INFO: SystemInfo`
   (pasta de nome neutro; o pacote no `Cargo.toml` leva o nome real).
2. App `crates/app` com `brand::set(…)` e
   `eframe::run_native(…, App::new(cc, &INFO, rom, scale))`.
3. Ainda específico do PV-1000 no frontend: só as medidas das capas na biblioteca
   (`COVER_W`/`COVER_H`, caixas em retrato) — virar campo do `SystemInfo` quando
   aparecer um sistema com caixas em outro formato.

## Como a emulação funciona

### Base de tempo

Tudo é contado em **unidades de 1/17,897727 MHz** (um tick de `clk_snd`):
- **ciclo de CPU = 5 unidades**;
- **pixel = 3 unidades**;
- **linha = 1.143 unidades**.

Com inteiros, CPU, vídeo e som andam juntos sem erro de arredondamento.

### Um quadro

Para cada uma das 263 linhas:
1. marca o início da linha e zera o contador de pixels desenhados;
2. na linha Y=195, liga o bit "quadro novo" da porta FC;
3. nas linhas de interrupção, roda a CPU até 2 ciclos depois do início da linha, ativa
   a interrupção, roda até hc 190 (+2 ciclos) e a desativa;
4. roda a CPU até o fim da linha, **instrução por instrução**, com os ciclos exatos de
   cada uma;
5. desenha o resto da linha.

### O quadro no modelo de hardware real

O modelo de hardware tem um laço próprio (`run_frame_hw`), com as medidas do aparelho
([Um quadro no aparelho](hardware.pt-BR.md#um-quadro-no-aparelho)).

**Base de tempo.** A mesma unidade (1/17,897727 MHz, o cristal): o ciclo de CPU continua
valendo 5 unidades, mas o pixel passa a valer **4** (cristal ÷ 4). Uma linha tem 288 pixels
= 1.152 unidades = 230,4 ciclos; o quadro, 262 linhas = 301.824 unidades = 59,299 Hz.

**Onde o quadro começa.** As linhas são numeradas como no NESdev (0 = primeira linha de
imagem). O laço começa na linha **246**, de modo que as 224 primeiras linhas do quadro
(16 de borda, as 192 de imagem e 16 de borda) caem nas mesmas linhas da imagem de saída que
no modelo MiSTer. As outras 38 linhas (o resto da borda, o apagamento e o sincronismo) são
emuladas mas não aparecem.

**Para cada linha:**
1. se a linha é de imagem (0–191), marca a janela do **BUSREQ**: do pixel 20 ao 267;
2. se é uma linha de interrupção (195, 199 … 255), roda a CPU até o **pixel 276** e guarda
   o pedido: bit 1 (matriz) nas 16 linhas, bit 0 (pré-render) também na 255, só se
   habilitados na porta FC;
3. roda a CPU até o fim da linha e desenha o que faltar.

**O roubo de barramento** fica dentro do laço da CPU (`run_until`):
- antes de cada instrução: se a CPU está dentro da janela do BUSREQ e a imagem está ligada
  (bit 3 da porta FF em 0), ela pula para o fim da janela;
- uma instrução que começa antes da janela e termina mais de 4 ciclos depois do início
  dela é interrompida ali e termina depois da janela (o Z80 cede o barramento no início de
  um ciclo de máquina). Uma que termina até 4 ciclos depois passa inteira. Isso dá de 32 a
  36 ciclos livres por linha de imagem, perto dos 32 a 37 medidos;
- com a imagem desligada, não há janela: a CPU roda o quadro inteiro.

**A interrupção** fica ativa **enquanto houver pedido guardado**, até o programa
reconhecer: ler a porta FD limpa o bit 1, escrever nela limpa o bit 0, e desabilitar na FC
também limpa.

**O desenho.** A coluna *c* da imagem é o pixel 52 + *c* da linha, e mostra as colunas de
tiles 2 a 29 inteiras (no modelo MiSTer, a imagem começa 1 pixel depois e a última coluna
é preta). Escritas no meio da linha continuam valendo: nas linhas de borda, onde a CPU
roda livre, uma troca de cor da borda aparece a partir do pixel em que aconteceu (é o que
desenha a barra do `irqtiming`).

**O atraso da matriz.** Cada uma das 4 entradas da porta FD tem um capacitor no aparelho.
O emulador guarda, para cada entrada, quando ela deixou de ser acionada (uma linha
desselecionada ou um botão solto) e continua lendo-a como 1 por **136 ciclos** (o meio da
faixa medida, 126–147). A subida, abaixo de ~15 ciclos, é tratada como instantânea. O
estado entra no save state (versão 3).

**Portas e vídeo:** a porta FE acha o mapa e o PCG, os 3 bits altos da FF
escolhem o banco e o bit 3 desliga a imagem; as portas sem dono leem `0xFF`. A RAM é
espelhada em 0x8000–0xBFFF, como no aparelho (no modelo MiSTer, a memória é plana, como no
core).

### A CPU

O Z80 vem do crate **[`iz80`](https://github.com/ivanizag/iz80)**, de Iván Izaguirre, numa cópia em `vendor/iz80` com cinco correções (as quatro
primeiras medidas contra o TV80):
- **ciclos:** prefixos DD/FD (+4 cada; operandos `(IX+d)` +8), `JR cc` não tomado (7) e
  `JP cc` (10);
- **registrador R:** soma 1 a cada busca de opcode, mantendo o bit 7;
- **`LD A,R` e `LD A,I`:** passam a ajustar os flags S e Z;
- o **HALT** anda em passos de 4 ciclos, somando 1 ao R em cada um, e só aceita a
  interrupção no fim de um passo. O `iz80` original nunca saía do HALT, mesmo com a
  interrupção pendente; na cópia corrigida, a interrupção o encerra (os jogos comerciais
  e os do Inufuto não usam HALT; o Demônios e a PV-1000 Control Test usam, para esperar o
  quadro);
- **IM 0 e IM 2**, que o `iz80` não implementava, com o barramento suposto em 0xFF (IM 0 =
  `RST 38h`; IM 2 lê o vetor em (I × 256) + 0xFF).

**A CPU nas baterias de teste padrão:** a cópia corrigida passa nos **67 testes do ZEXALL** e
no teste do 8080. No `z80test` (Patrik Rak), que confere até os comportamentos não
documentados, passa em 137 de 152: as 15 falhas (flags 3 e 5 do SCF/CCF, o registrador
interno MEMPTR em `BIT n,(IX+d)`, as flags das instruções `IN` e o `LD A,I`/`LD A,R`) são
**as mesmas do `iz80` original**, ou seja, as correções do projeto não estragaram nenhuma
instrução. Nenhum jogo depende desses detalhes; corrigi-los exige conferir antes o que o
TV80, base do modelo MiSTer, faz.

Detalhes finos da temporização:
- a interrupção é vista **2 ciclos depois** de mudar, como no TV80;
- a **primeira instrução após o reset** é 2 ciclos mais curta.

### O vídeo, pixel a pixel

O chip lê a RAM durante cada pixel. O Mil Capivaras reproduz isso **desenhando a linha por
partes**:
1. cada escrita no mapa de tiles, no PCG (0xB800–0xBFFF) ou na porta FF é guardada com
   o valor antigo e o novo;
2. ao fim da instrução, a escrita é datada: no último ciclo de máquina da instrução (3
   ciclos por escrita, terminando 2 antes do fim), ou no **ciclo 12** em
   `LDI`/`LDD`/`LDIR`/`LDDR`, cuja gravação vem antes dos ciclos de repetição;
3. a linha é desenhada até aquele instante com o valor antigo, e daí em diante com o
   novo.

O custo é baixo: de ~7.800 para ~7.400 quadros/s.

### O som

Os 3 canais avançam um passo a cada 1024 ciclos do relógio de 17,9 MHz, sincronizados
com a CPU a cada escrita nas portas F8–FB. Cada canal tem o seu volume (0,5 / 0,75 / 1,0)
e a porta FB liga o som e escolhe o modo XOR ([Som](hardware.pt-BR.md#som)). As amostras saem na taxa da placa de som (48 kHz) e viram estéreo no
adaptador.

### Save states

O estado é **completo e determinístico**: CPU, RAM 0x8000–0xFFFF, E/S e som, ~33 KB.
Salvar, rodar e restaurar dá exatamente a mesma imagem e RAM; um teste confere. Por
isso o retroceder e, no RetroArch, o run-ahead e o netplay funcionam.

## O porquê de cada decisão

| Decisão | Por quê |
|---|---|
| **Partir do core MiSTer, não do MAME** | um circuito em Verilog dá para simular e comparar byte a byte, e a lista do MAME marcava 6 jogos como não suportados [M2] (marca desatualizada: no MAME 0.289 os 13 rodam). O preço: o MiSTer simplifica o hardware onde o MAME não simplifica |
| **Reescrever em Rust em vez de simular o Verilog** | um core FPGA é hardware descrito em Verilog; simulá-lo (Verilator) rodou a só ~200% do tempo real e exige o compilador do Verilog. Reescrito como emulador, roda a ~12.500% e vira um binário só |
| **Manter a simulação Verilator como referência** | cada diferença vira um número (pixels e bytes) em vez de impressão visual; foi assim que todas as divergências foram achadas e zeradas |
| **Usar o crate `iz80` e corrigi-lo** | escrever um Z80 do zero é muito trabalho e muito risco; o `iz80` passa no ZEXALL. Os erros de ciclos e do R foram corrigidos numa cópia (BSD-3) com a licença junto, e documentados para mandar ao autor |
| **224×224 e não 256×192** | é a área que o core mostra: 223 pixels úteis por linha e a borda em cima e embaixo, como no aparelho [F1] |
| **59,538 Hz e não 60** | é a taxa do console; a música e a velocidade dos jogos ficam como no original. Num monitor de 60 Hz, um quadro se repete a cada ~2 s, o que quase ninguém percebe |
| **Pixels quadrados (1:1) por padrão** | é como o core do MiSTer exibe; a proporção do aparelho, 48:35 [N4], fica na opção **Imagem: TV** |
| **Lista do MAME (CC0) para identificar, não o DAT da No-Intro** | o MAME reconhece também os homebrews e é domínio público; o DAT da No-Intro era de 2020 e não tinha licença clara |
| **Tudo embutido no executável** | um arquivo só, que roda em qualquer pasta; nada para perder ou desencontrar de versão |
| **Fichas só embutidas; traduções da interface embutidas e também numa pasta de idiomas** | as fichas entram pelo projeto, passam pelos testes e não ficam desatualizadas em silêncio; um idioma novo da interface pode ser testado sem recompilar, e um texto que falta aparece em inglês |
| **Inglês como texto-fonte** | é a língua comum para quem contribuir; português e espanhol são catálogos |
| **Interface separada do núcleo, nas convenções do libretro** | a mesma interface serve a outros emuladores, e o core libretro sai quase de graça |
| **egui** | uma interface nativa em Rust, sem Qt nem GTK, que cabe no binário |
| **Controles por `/dev/input/jsX` (Linux) e evdev (FreeBSD)** | funciona com qualquer controle que o kernel reconheça, sem SDL. No Windows, `gilrs` (XInput) |
| **`curl` e `ffmpeg` externos** | estão em qualquer distro e não aumentam o binário; sem eles, só capas e MP4 ficam indisponíveis |
| **Som pelo hardware, não pelo MiSTer** | a afinação dos jogos não decide a oitava, mas a documentação do NESdev, feita no aparelho, decide: o MiSTer toca uma oitava acima e ignora a porta FB [N2]. Mudar o som não altera imagem nem RAM, então a comparação com o MiSTer continua valendo |
| **Som começa mudo** | a onda quadrada cheia é alta; melhor o usuário ligar |
| **Escritas datadas dentro da instrução, em vez de um Z80 ciclo a ciclo** | dá o mesmo resultado nos 35 jogos, com 5% de custo; um Z80 ciclo a ciclo seria reescrever a CPU |
