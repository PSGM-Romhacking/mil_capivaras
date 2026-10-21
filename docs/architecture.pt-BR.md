# Arquitetura: núcleo × interface

[English](architecture.md) · **Português** · [Español](architecture.es.md)

Objetivo: a interface do Mil Capivaras servir para **outros emuladores**, e o núcleo do
PV-1000 virar **core libretro** sem reescrever nada. Plano aprovado em 2026-10-17;
passos 1–6 concluídos no mesmo dia, incluindo o core libretro.

## Estrutura (workspace Cargo)

```
crates/api/             contrato núcleo ↔ interface (sem dependências)
crates/core/            o console PV-1000 (machine, memory, banco de jogos, testes)
crates/frontend/        interface reutilizável (egui, menus, biblioteca, controles,
                        save states, rewind, gravação, capas, idiomas, config)
crates/app/             executável: pv1000-core + emu-frontend + marca (nome, ícone, Sobre)
crates/libretro/        mil_capivaras_libretro.so = pv1000-core + API libretro (feito em 2026-10-17)
```

- `pv1000-core` → `emu-api`, `iz80` e `serde`/`serde_json`; nada de interface.
- `emu-frontend` → dos crates do workspace, só `emu-api` (além de bibliotecas como egui, cpal…); não conhece nenhum console.
- `crates/app` liga os dois; outro emulador = outro app com o mesmo `emu-frontend`.

## Contrato (`emu-api`), nas convenções do libretro

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

## Passos (o programa funciona ao fim de cada um)

1. Workspace + mover arquivos sem mudar comportamento.
2. `emu-api` + adaptador do PV-1000 (RetroPad → bits do MiSTer, mono → estéreo, estado em bytes).
3. `app.rs` usa o trait (resolução, FPS, controles por RetroPad, estado em bytes).
4. Marca e textos do PV-1000 no app; um catálogo por idioma.
5. Conferir tudo (testes, quadros, controles, gravação, biblioteca, interface) e documentar.
6. (Separado) `libretro` — ver [libretro.pt-BR.md](libretro.pt-BR.md).

## Efeitos colaterais

- Nomes das ações de controle no `config.json` mudam (ex.: `fire1` → `p1_b`).
- Save states: novo cabeçalho; leitura do formato antigo mantida.

## Como ficou (passos 1–5)

- **Verificação:** os testes passam; quadros de Ascend, Cacorm e Mazy idênticos byte a byte
  ao binário anterior; save state antigo (`PVST`) carrega pelo caminho novo.
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

## Novo emulador com a mesma interface

1. Crate `crates/core` implementando `emu_api::Core` + um `static INFO: SystemInfo`
   (pasta de nome neutro; o pacote no `Cargo.toml` leva o nome real).
2. App `crates/app` com `brand::set(…)` e
   `eframe::run_native(…, App::new(cc, &INFO, rom, scale))`.
3. Ainda específico do PV-1000 no frontend: só as medidas das capas na biblioteca
   (`COVER_W`/`COVER_H`, caixas em retrato) — virar campo do `SystemInfo` quando
   aparecer um sistema com caixas em outro formato.

## Organização dos arquivos (revisão de 2026-10-18)

Conferido: as dependências seguem o desenho acima (o núcleo não usa egui nem nada da
interface; a interface não depende do `pv1000-core`; o core libretro não usa a interface).
Os dois arquivos que tinham crescido demais foram divididos, sem mudar comportamento:

**`crates/frontend/src/app.rs`** e os módulos-filhos em `app/`, cada um
um bloco `impl App` (um módulo-filho enxerga os campos privados do pai, então a estrutura
`App` não precisou abrir nada):

| Arquivo | Conteúdo |
|---|---|
| `app.rs` | estado do app, criação, abrir e fechar jogo, preferências, o ciclo de cada quadro da interface |
| `app/emulation.rs` | laço de emulação: ritmo, entrada, retroceder, avanço rápido, som |
| `app/input.rs` | atalhos, teclado e controle, mapeamentos |
| `app/nav.rs` | navegação pelo controle, menu rápido e modo sofá |
| `app/menus.rs` | a barra de menus |
| `app/screen.rs` | a tela do jogo, molduras e filtros |
| `app/library.rs` | biblioteca, identificação dos jogos, fichas |
| `app/state.rs` | save states (slots, miniaturas) |
| `app/recording.rs` | capturas, GIF e MP4 |
| `app/controls.rs` | o diálogo de controles e atalhos |
| `app/dialogs.rs` | os demais diálogos |
| `app/folders.rs` | o diálogo de pastas |
| `app/docshots.rs` | o roteiro de capturas da documentação |

**`crates/core/src/machine.rs`** e os filhos em `machine/`. Aqui o
pai usa os campos dos filhos, então eles são `pub(super)`:

| Arquivo | Conteúdo |
|---|---|
| `machine.rs` | o console: criação, carga, reset, modelo, controles, interrupções e os dois laços de quadro |
| `machine/bus.rs` | memória e IO como a CPU os vê, a matriz do joystick, registradores de vídeo (portas FE e FF) |
| `machine/video.rs` | o desenho das linhas enquanto a CPU roda |
| `machine/sound.rs` | o gerador de som (3 ondas quadradas, porta FB, volumes, passa-altas) |
| `machine/timing.rs` | base de tempo e constantes de temporização dos dois modelos |
| `machine/debug.rs` | depuração: registro de interrupções, rastro de instruções, registradores, memória, tiles |
| `machine/state.rs` | `Snapshot` e o formato do save state |
| `machine/tests.rs` | os testes do console |

Verificação: os testes passam, o `cargo clippy --workspace --all-targets` não dá nenhum
aviso, as 152 comparações da regressão do modelo MiSTer saíram idênticas e as capturas da
documentação também (menos a aba CPU do depurador, que mostra registradores ao vivo).
