# Mil Capivaras como core do RetroArch (libretro)

[English](libretro.md) · **Português** · [Español](libretro.es.md)

Como o emulador vira o core `mil_capivaras_libretro.so`, que roda dentro do RetroArch, e
por que cada decisão foi tomada.


> **Estado:** `libretro` gera `mil_capivaras_libretro.so`
> (`make libretro`; `make install-libretro` copia `.so` + `.info` para a pasta de cores
> do RetroArch lida do `retroarch.cfg`). Testado no RetroArch Flatpak 1.22.2: carrega,
> 224×224 a 59,54 FPS, áudio 48000 Hz, e o quadro 600 do Dig Dug é idêntico byte a
> byte ao do Mil Capivaras. O teste `c_api_matches_the_core_and_states_round_trip` confere
> a API em C contra o núcleo direto (quadros, taxa de áudio, save state, RAM).
> A pasta de `.info` do Flatpak (`/app/share/libretro/info`) é só leitura: o `.info`
> fica ao lado do `.so`, e sem ele o RetroArch lista o core pelo nome do arquivo.
> Ainda não feito: a opção de som suave e `SET_MEMORY_MAPS`.

## Por que vale a pena

- O RetroArch passa a cuidar de **janela, vídeo, shaders, áudio, controles,
  remapeamento, save states, rewind, run-ahead, netplay e RetroAchievements**.
- O coração do emulador já está separado da interface no crate `crates/core`
  (console, mapa de memória, CPU `iz80`), que depende de `emu-api`, `iz80` e
  `serde`/`serde_json`, e de nada de interface. Nada ali usa egui, cpal, arquivos ou relógio — é exatamente o que um core precisa.

## O que é um core libretro

Uma biblioteca dinâmica (`.so`) que exporta funções C com nomes fixos (`retro_*`).
O RetroArch carrega a biblioteca, entrega a ROM e chama `retro_run()` uma vez por
quadro; o core devolve o quadro, o áudio e lê os botões por *callbacks* que o
RetroArch registrou antes. O core **não** abre janela, não toca som, não lê teclado
nem cria threads — só emula.

Referência: `libretro.h` em <https://github.com/libretro/libretro-common/blob/master/include/libretro.h>
e a documentação em <https://docs.libretro.com/development/cores/developing-cores/>.

## Diretrizes

### 1. Workspace

O projeto já está separado (ver [architecture.pt-BR.md](architecture.pt-BR.md)):

```
mil_capivaras/
├── Cargo.toml                 [workspace]
├── crates/api/                trait Core + SystemInfo, nas convenções do libretro
├── crates/core/               console + adaptador Core  (dep.: emu-api, iz80, serde)
├── crates/frontend/           a interface egui reutilizável
├── crates/app/                o executável
└── crates/libretro/           crate-type = ["cdylib"] → mil_capivaras_libretro.so
```

- O core libretro só precisa de `pv1000-core` (via o trait `emu_api::Core`): cada
  `retro_*` vira uma chamada do trait, e as convenções já batem — RetroPad em bits
  na ordem do libretro, quadro XRGB8888 (`RETRO_PIXEL_FORMAT_XRGB8888`), áudio f32
  estéreo (converter para i16 em `retro_audio_sample_batch`), estado em bytes
  opacos (`retro_serialize`) e `system_ram()` (`RETRO_MEMORY_SYSTEM_RAM`).
- `pv1000-core` não pode depender de egui, cpal, rfd, libc, arquivos ou `Instant`.
- A interface e o core libretro usam **o mesmo** `pv1000-core`: uma correção de
  emulação vale para os dois.

### 2. Ligação com a API C

- A API é pequena (~25 funções). Recomendado: gerar os tipos de `libretro.h` com
  `bindgen` uma vez e versionar o arquivo gerado, ou escrever à mão só o que for
  usado. O crate `libretro-sys` existe, mas está parado há anos.
- Todas as funções exportadas: `#[unsafe(no_mangle)] pub extern "C" fn retro_…`.
- O estado do core fica num `static` (ex.: `Mutex<Option<Core>>` ou `static mut`
  com cuidado): a API não passa ponteiro de contexto.
- Nunca deixar um `panic` atravessar a fronteira C: usar `std::panic::catch_unwind`
  nas funções exportadas, ou compilar com `panic = "abort"` no perfil do core.

### 3. Funções obrigatórias e como mapear

| Função libretro | O que fazer no Mil Capivaras |
|---|---|
| `retro_api_version` | devolver `RETRO_API_VERSION` (1) |
| `retro_set_environment` | guardar o callback; anunciar as opções do core (`SET_VARIABLES`) |
| `retro_set_video_refresh`, `retro_set_audio_sample_batch`, `retro_set_input_poll`, `retro_set_input_state` | guardar os callbacks |
| `retro_init` / `retro_deinit` | `retro_init` não faz nada; `retro_deinit` libera o core |
| `retro_get_system_info` | nome "Mil Capivaras", versão, extensões `pv1\|bin\|rom`, `need_fullpath = false`, `block_extract = false` (o RetroArch descompacta o `.zip`) |
| `retro_get_system_av_info` | geometria do quadro atual (224×224, ou 224×244 com a imagem TV), FPS = `Core::fps()` (59,538 no modelo MiSTer, 59,299 no de hardware), taxa de áudio fixa (48000) |
| `retro_load_game` | definir formato de pixel (`SET_PIXEL_FORMAT` = XRGB8888) e os descritores de entrada; `(INFO.create)(48000)` + opções do core + `load(data)` |
| `retro_run` | `input_poll()`; ler as portas 0 e 1 como RetroPad; `run_frame(&pads)`; enviar vídeo e áudio |
| `retro_reset` | `reset()` |
| `retro_serialize_size` / `retro_serialize` / `retro_unserialize` | `save_state()` / `load_state()` (tamanho fixo, ver item 7) |
| `retro_get_memory_data` / `retro_get_memory_size` | `RETRO_MEMORY_SYSTEM_RAM` → `system_ram_mut()`, `mem[0xB800]`, tamanho 0x800 (ver item 8) |
| `retro_unload_game`, `retro_get_region` (NTSC), `retro_set_controller_port_device`, `retro_cheat_*`, `retro_load_game_special` | mínimos / vazios |

### 4. Vídeo

- O quadro do core (`Core::video()`) já é `0x00RRGGBB` — **é exatamente o formato
  `RETRO_PIXEL_FORMAT_XRGB8888`**. `video_refresh(f.pixels, f.width, f.height, f.pitch * 4)`,
  sem conversão.
- Proporção: o `aspect` do quadro vai em `aspect_ratio`. A opção `mil_capivaras_picture`
  (item 9) escolhe MiSTer (224×224, pixel quadrado) ou TV (224×244, pixel 48:35).
- Escala, filtros (scanlines/CRT) e tela cheia **saem do core**: o RetroArch tem
  shaders para isso.

### 5. Áudio

- `retro_audio_sample_batch` recebe **i16 estéreo intercalado**. O trait já entrega `f32`
  estéreo intercalado (`take_audio()`): o core só converte com
  `(s.clamp(-1.0, 1.0) * 32767.0) as i16`.
- Taxa: fixa em 48000, passada ao `(INFO.create)` e informada em `av_info.timing.sample_rate`.
- "Som suave" (passa-baixa + bloqueio de DC, hoje `App::soften` em
  `crates/frontend/src/app/emulation.rs`) vira opção do core;
  mover a função para o `pv1000-core` para as duas interfaces usarem.
- Volume e mudo são do RetroArch.

### 6. Entrada (RetroPad)

- Portas 0 e 1 = jogadores 1 e 2, dispositivo `RETRO_DEVICE_JOYPAD`.
- Sugestão de mapa: D-pad → direções; **B** (botão de baixo) → Fire 1; **A**
  (direita) → Fire 2; Select → Select; Start → Start. O core libretro só lê o RetroPad;
  a conversão para os bits do joystick do MiSTer (0 dir, 1 esq, 2 baixo, 3 cima, 4 Fire 1,
  5 Fire 2, 6 Select, 7 Start) fica em `crates/core/src/core.rs`.
- Anunciar os nomes com `SET_INPUT_DESCRIPTORS` ("Fire 1", "Fire 2"…), para aparecerem
  no menu de controles do RetroArch.
- `gamepad.rs` e `keys.rs` **não** vão para o core: o remapeamento é do RetroArch.

### 7. Save states, rewind e determinismo

- O RetroArch usa `retro_serialize` também para **rewind, run-ahead e netplay**: o
  core ganha esses recursos de graça se o estado for **completo, determinístico e de
  tamanho fixo**.
- Já atendido: `Snapshot` guarda CPU, RAM, IO e som; o teste
  `save_state_is_deterministic` prova que salvar → rodar → restaurar → rodar dá
  imagem e RAM idênticas. O tamanho de `to_bytes` é fixo para um mesmo build.
- Cuidado ao evoluir: qualquer campo novo de estado do console precisa entrar no
  `Snapshot` (`crates/core/src/machine/state.rs`) e mudar `STATE_VERSION`.
- O cabeçalho do arquivo (`PVST`, versão, hash da ROM) pode continuar: o RetroArch
  trata o buffer como opaco.

### 8. Memória e RetroAchievements

- `retro_get_memory_data(RETRO_MEMORY_SYSTEM_RAM)` → `mem[0xB800..0xC000]` (2 KB,
  contíguos no array do core). Esse bloco **é o espaço plano 0x000–0x7FF** de
  `crates/core/src/memory.rs`: os endereços das conquistas ficam iguais nas duas versões.
- Melhor ainda: `SET_MEMORY_MAPS` com um descritor por região do `memory::MAP`
  (o RetroArch e o rcheevos usam isso para traduzir endereços).
- Com o core no RetroArch, a integração com o RetroAchievements é **do RetroArch**
  (login, hardcore, notificações) — o `achievements.rs` da interface própria fica
  só para quem usar o Mil Capivaras sem RetroArch. Conquistas oficiais ainda dependem do
  PV-1000 ser aceito como console pelo RetroAchievements.
- Sem bateria no cartucho: `RETRO_MEMORY_SAVE_RAM` = vazio (tamanho 0).

### 9. Opções do core

Implementadas com `SET_VARIABLES` (a forma mais simples, que todo frontend entende); os
rótulos ficam em inglês no RetroArch:

| Variável | Valores | Efeito |
|---|---|---|
| `mil_capivaras_model` | *MiSTer core (validated)* / *Real hardware (NESdev/MAME)* | o modelo de emulação: o do core do MiSTer (59,538 Hz, validado byte a byte) ou o do aparelho (59,299 Hz, roubo de barramento, portas FC/FD/FE/FF completas, atraso da matriz) |
| `mil_capivaras_picture` | *MiSTer (224x224, square pixels)* / *TV (26-line border, 48:35 pixels)* | só a apresentação: 224 × 224 com pixel quadrado, ou 224 × 244 com a borda do aparelho e pixel 48:35 |

Lidas com `GET_VARIABLE` no `retro_load_game` e quando `GET_VARIABLE_UPDATE` avisar. Ao
mudar:
- **se a taxa de quadros mudou** (troca de modelo), o core chama `SET_SYSTEM_AV_INFO` com a
  geometria e a taxa novas (`Core::fps()`), para o RetroArch ajustar o ritmo e o som;
- **senão**, só `SET_GEOMETRY`, porque a imagem pode ter mudado de tamanho ou proporção.

### 10. Arquivo `.info`

O RetroArch precisa de `mil_capivaras_libretro.info` ao lado do core (ou na pasta de info):

```ini
# Mil Capivaras as a libretro core (RetroArch reads this next to the .so or in its info folder)
display_name = "Casio - PV-1000 (Mil Capivaras)"
authors = "Péricles S."
supported_extensions = "pv1|bin|rom"
corename = "Mil Capivaras"
categories = "Emulator"
license = "GPLv3"
permissions = ""
display_version = "0.9.0"
manufacturer = "Casio"
systemname = "PV-1000"
systemid = "pv1000"
database = "Casio - PV-1000"
supports_no_game = "false"
savestate = "true"
savestate_features = "deterministic"
needs_fullpath = "false"
cheats = "true"
cheevos_option = "true"
description = "Casio PV-1000 emulator in Rust, reimplemented from the MiSTer core (Flandango, GPL)."
```

O nome em `database` precisa bater com o banco do RetroArch (`libretro-database`)
para o scanner reconhecer os jogos — conferir se existe `Casio - PV-1000.rdb`.

### 11. Build e instalação local

```bash
cargo build --release -p mil_capivaras_libretro      # gera target/release/libmil_capivaras_libretro.so
C=~/.var/app/org.libretro.RetroArch/config/retroarch/cores
install -m 755 target/release/libmil_capivaras_libretro.so $C/mil_capivaras_libretro.so
install -m 644 crates/libretro/mil_capivaras_libretro.info $C/
flatpak run org.libretro.RetroArch -L $C/mil_capivaras_libretro.so "jogo.bin"
```

- O RetroArch daqui é **Flatpak** (runtime `org.kde.Platform 6.11`, glibc 2.42). Um
  core compilado no Fedora 43 (exige glibc ≥ 2.39) roda nele. Para distribuir (feito),
  a CI (`.github/workflows/release.yml`, `packaging/build_linux.sh`) compila com
  `cargo-zigbuild` para glibc 2.27, e o core roda também em sistemas mais antigos.
- O core de Windows (`.dll`) é compilado de forma cruzada com MinGW
  (`packaging/build_windows.sh`), e o de FreeBSD (`.so`), pelo job `freebsd` do `release.yml`.
- O nome do arquivo instalado tem que terminar em `_libretro.so` (`_libretro.dll` no Windows).
- O core não pode depender de bibliotecas que o runtime do Flatpak não tenha: com
  `pv1000-core` só em Rust (`iz80`, `serde`), depende apenas da libc.

### 12. Testes

- Os testes de `pv1000-core` (save state determinístico, mapa de memória, leitura
  plana) valem para o core sem mudança.
- Comparar quadros: rodar o mesmo jogo no Mil Capivaras (`--shot`) e no core
  (`retroarch --max-frames=N` + captura) — devem ser idênticos.
- Ferramenta útil: o "libretro-test" / `retroarch --verbose` para ver as chamadas.

## Ordem sugerida

1. Workspace + `pv1000-core` (só mover arquivos; a interface continua funcionando).
2. Core mínimo: `load_game`, `run` (vídeo + áudio + RetroPad), `reset`, `.info`.
3. Save states (`serialize`) → rewind/run-ahead/netplay do RetroArch.
4. Mapa de memória (`SYSTEM_RAM`: feito; `SET_MEMORY_MAPS`: pendente) → base para conquistas.
5. Opções do core (modelo e imagem: feito; som suave: pendente) e traduções.
6. Build para distribuição (feito: CI com `cargo-zigbuild`, glibc 2.27).
