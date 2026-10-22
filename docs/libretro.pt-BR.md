# Core libretro (RetroArch)

[English](libretro.md) · **Português** · [Español](libretro.es.md)

O Mil Capivaras também vem como **core libretro**: `mil_capivaras_libretro.so`
(`.dll` no Windows), o mesmo `pv1000-core` da interface própria atrás da API C do libretro.
O RetroArch carrega o core, entrega a ROM e chama `retro_run()` uma vez por quadro; o core
devolve imagem e som e lê os controles por *callbacks*. Ele não abre janela, não toca som e
não cria threads: só emula.

Testado no RetroArch Flatpak 1.22.2: 224×224 a 59,54 FPS, áudio a 48 kHz, e o quadro 600
do Dig Dug é idêntico byte a byte ao do Mil Capivaras.

Referência da API: [`libretro.h`](https://github.com/libretro/libretro-common/blob/master/include/libretro.h)
e [*Developing cores*](https://docs.libretro.com/development/cores/developing-cores/).

## Instalar

Os releases trazem o core pronto (`mil_capivaras_libretro-<versão>-<sistema>.zip`, com o
`.so` ou `.dll` e o `.info`): descompacte na pasta de cores do RetroArch. Para compilar:

```bash
make libretro           # target/release/libmil_capivaras_libretro.so
make install-libretro   # copia o .so e o .info para a pasta de cores lida do retroarch.cfg
```

O `make install-libretro` acha a pasta no `retroarch.cfg` do Flatpak ou do RetroArch
nativo. No Flatpak, a pasta de `.info` do sistema (`/app/share/libretro/info`) é só leitura,
então o `.info` vai ao lado do `.so`; sem ele, o RetroArch lista o core pelo nome do
arquivo. Para rodar direto:

```bash
flatpak run org.libretro.RetroArch -L ~/.var/app/org.libretro.RetroArch/config/retroarch/cores/mil_capivaras_libretro.so jogo.bin
```

## O que o RetroArch traz

Janela, shaders, remapeamento de controles, save states, rewind, run-ahead, netplay e
trapaças (pela RAM exposta), sem nada a mais no core. Escala, filtros (scanlines, CRT),
volume e mudo também ficam com o RetroArch.

## Como funciona

### O mesmo núcleo

`crates/libretro` (`crate-type = ["cdylib"]`) depende só de `emu-api` e `pv1000-core`
([arquitetura](architecture.pt-BR.md)). O `pv1000-core` não usa egui, áudio, arquivos nem
relógio, e o contrato `emu-api` já segue as convenções do libretro. Por isso cada `retro_*`
é uma chamada do trait `Core`, e uma correção de emulação vale para as duas interfaces.

### A ligação com a API C

- Os tipos de `libretro.h` que o core usa estão escritos à mão em `lib.rs` (`#[repr(C)]`),
  sem `bindgen` nem o crate `libretro-sys`.
- A API não passa ponteiro de contexto: o core fica num `static Mutex<Option<…>>`.
- Toda função exportada passa por `catch_unwind`: nenhum *panic* atravessa para o C.

### As funções

| Função libretro | No Mil Capivaras |
|---|---|
| `retro_api_version` | `RETRO_API_VERSION` (1) |
| `retro_set_environment` | guarda o callback e anuncia as opções do core (`SET_VARIABLES`) |
| `retro_set_video_refresh`, `retro_set_audio_sample_batch`, `retro_set_input_poll`, `retro_set_input_state` | guardam os callbacks |
| `retro_init` / `retro_deinit` | `retro_init` não faz nada; `retro_deinit` libera o core |
| `retro_get_system_info` | nome "Mil Capivaras", versão, extensões `pv1\|bin\|rom`, `need_fullpath = false`, `block_extract = false` (o RetroArch descompacta `.zip` e `.7z`) |
| `retro_get_system_av_info` | geometria do quadro atual (224×224, ou 224×244 com a imagem TV), FPS = `Core::fps()` (59,538 no modelo MiSTer, 59,299 no de hardware), áudio a 48000 Hz |
| `retro_load_game` | pede o formato XRGB8888 (`SET_PIXEL_FORMAT`) e anuncia os botões (`SET_INPUT_DESCRIPTORS`); cria o núcleo, aplica as opções e carrega a ROM |
| `retro_run` | relê as opções se mudaram; lê os controles das portas 0 e 1; roda um quadro; envia imagem e som |
| `retro_reset` | `reset()` |
| `retro_serialize_size` / `retro_serialize` / `retro_unserialize` | `save_state()` / `load_state()` |
| `retro_get_memory_data` / `retro_get_memory_size` | `RETRO_MEMORY_SYSTEM_RAM`: os 2 KB de RAM |
| `retro_get_region` | NTSC |
| `retro_unload_game`, `retro_set_controller_port_device`, `retro_cheat_*`, `retro_load_game_special` | vazias |

### Vídeo

O quadro do núcleo já é `0x00RRGGBB`, o formato `RETRO_PIXEL_FORMAT_XRGB8888`: vai direto
para o RetroArch, sem conversão. A proporção do quadro vai em `aspect_ratio`.

### Áudio

O núcleo entrega `f32` estéreo intercalado; o core converte para `i16` com
`(s.clamp(-1.0, 1.0) * 32767.0) as i16` e envia por `retro_audio_sample_batch`, a 48000 Hz.

### Controles

Portas 0 e 1 = jogadores 1 e 2, como RetroPad (`RETRO_DEVICE_JOYPAD`):

| RetroPad | PV-1000 |
|---|---|
| direcional | direcional |
| B (botão de baixo) | Fire 1 |
| A (botão da direita) | Fire 2 |
| Select / Start | Select / Start |

Os nomes ("Fire 1", "Fire 2"…) aparecem no menu de controles do RetroArch. A conversão
para os bits do joystick do MiSTer fica em `crates/core/src/core.rs`.

### Save states, rewind e netplay

O RetroArch usa `retro_serialize` também para rewind, run-ahead e netplay, o que exige um
estado **completo, determinístico e de tamanho fixo**. O estado do núcleo guarda CPU, RAM,
E/S e som (`crates/core/src/machine/state.rs`, com cabeçalho `PVST` e versão), e o teste
`save_state_is_deterministic` confere que salvar, rodar e restaurar dá a mesma imagem e
RAM. Um campo novo no estado do console precisa entrar no `Snapshot` e mudar
`STATE_VERSION`.

### Memória

`RETRO_MEMORY_SYSTEM_RAM` expõe 0xB800–0xBFFF (2 KB), que é o espaço plano 0x000–0x7FF de
`crates/core/src/memory.rs`: os endereços de trapaças e conquistas são os mesmos no core e
no `--memory-map` da interface própria. O cartucho não tem bateria, então não há
`RETRO_MEMORY_SAVE_RAM`.

### Opções do core

Por `SET_VARIABLES`, a forma que todo frontend entende; os rótulos ficam em inglês:

| Variável | Valores | Efeito |
|---|---|---|
| `mil_capivaras_model` | *MiSTer core (validated)* / *Real hardware (NESdev/MAME)* | o modelo de emulação: o do core do MiSTer (59,538 Hz, validado byte a byte) ou o do aparelho (59,299 Hz, roubo de barramento, portas FC/FD/FE/FF completas, atraso da matriz) |
| `mil_capivaras_picture` | *MiSTer (224x224, square pixels)* / *TV (26-line border, 48:35 pixels)* | só a apresentação: 224 × 224 com pixel quadrado, ou 224 × 244 com a borda do aparelho e pixel 48:35 |

O core lê as opções no `retro_load_game` e quando `GET_VARIABLE_UPDATE` avisa. Se a taxa de
quadros mudou (troca de modelo), chama `SET_SYSTEM_AV_INFO`, para o RetroArch ajustar o
ritmo e o som; senão, só `SET_GEOMETRY`.

### O arquivo `.info`

`crates/libretro/mil_capivaras_libretro.info` diz ao RetroArch o nome do core, o sistema e
os recursos (`savestate_features = "deterministic"`, `cheats`, `cheevos_option`). O campo
`database = "Casio - PV-1000"` bate com o `Casio - PV-1000.rdb` do libretro-database, então
o scanner do RetroArch reconhece os jogos.

### Compilação para distribuição

- **Linux:** a CI (`.github/workflows/release.yml`, `packaging/build_linux.sh`) compila com
  `cargo-zigbuild` para glibc 2.27, e o core roda em sistemas antigos e no runtime do
  Flatpak.
- **Windows:** `.dll` compilada de forma cruzada com MinGW (`packaging/build_windows.sh`).
- **FreeBSD:** `.so` compilado pelo job `freebsd` do `release.yml`.

O arquivo instalado tem de terminar em `_libretro.so` (`_libretro.dll` no Windows). Como o
`pv1000-core` é só Rust, o core depende apenas da libc.

## Testes

O teste `c_api_matches_the_core_and_states_round_trip` chama a API em C como o RetroArch
chamaria e compara com o núcleo direto: quadros, taxa de áudio, save state e RAM. Os testes
do `pv1000-core` (save state determinístico, mapa de memória) valem para o core sem mudança.

## O que falta

- **Som suave** como opção do core: o passa-baixa com bloqueio de DC da interface própria
  (`soften`, em `crates/frontend/src/app/emulation.rs`) teria de ir para o `pv1000-core`.
- **`SET_MEMORY_MAPS`**, com um descritor por região do `memory::MAP`, que o RetroArch e o
  rcheevos usam para traduzir endereços. Hoje só há a RAM em `RETRO_MEMORY_SYSTEM_RAM`.

## RetroAchievements

**Ainda não há conquistas para o PV-1000**, e o bloqueio não está no emulador: o
RetroAchievements (RA) não suporta o console. A lista oficial de consoles do `rcheevos`, a
biblioteca do RA, vai até o ID 82 (PS3) e não tem o PV-1000 ([rc_consoles.h](https://github.com/RetroAchievements/rcheevos/blob/develop/include/rc_consoles.h)). Sem
esse ID não há como cadastrar jogos nem conquistas.

**O que o RA pede para aceitar um console novo** ([System X wen](https://docs.retroachievements.org/orphaned/system-x-wen.html)):
1. um emulador, de preferência um core libretro, que **exponha a memória** e rode no
   **RALibretro** (a ferramenta em que se criam as conquistas);
2. uma **identificação única** de cada jogo: o MD5 do arquivo, sem cabeçalho;
3. o **servidor** do RA configurado para o sistema (trabalho da equipe do RA, junto com o
   ID novo no `rcheevos`).

**Situação de cada parte:**

| Parte | Situação |
|---|---|
| Memória exposta | pronta: o core entrega os 2 KB de RAM em `RETRO_MEMORY_SYSTEM_RAM` |
| Identificação dos jogos | simples: a ROM do PV-1000 não tem cabeçalho, o MD5 é o do arquivo inteiro |
| Mapa de memória | pronto: `--memory-map` gera no formato do RA |
| RALibretro | falta testar (é um programa de Windows; no Linux, pelo Wine) |
| Pedido do console | falta: pelo fórum ou Discord do RA; um PR ao `rcheevos` com o ID, o mapa de memória e o método de hash acelera |
| Servidor e conquistas | depende do RA aceitar e de alguém criar conquistas para os 13 jogos |

**O caminho previsto é o core libretro:** aceito o console, o RetroArch já cuida de login,
conquistas e modo hardcore, sem nada a mais no Mil Capivaras.

**Conquistas no próprio emulador** são um projeto à parte. Os pontos de ligação já estão
marcados em `achievements.rs`, e as regras de "hardcore compliance" do RA
([Hardcore Compliance Requirements](https://docs.retroachievements.org/general/hardcore-compliance-requirements.html)) pedem:
- integrar o `rcheevos` (C, via FFI): login, lista de conquistas, notificações, rich
  presence, placares e fila de desbloqueios feitos offline;
- no modo hardcore, bloquear trapaças, retroceder, câmera lenta, avanço de quadro, carregar
  save states, o depurador e o editor de memória; passar de casual para hardcore reinicia o
  jogo;
- um user agent próprio (`MilCapivaras/0.9.1 (Linux)`);
- **6 meses de lançamento público**, no mínimo, antes de pedir a aprovação para hardcore.

**Plano (ainda não feito): notas de memória e rascunhos de conquistas automáticos.**
No RA, uma conquista é uma lógica sobre endereços da RAM (ex.: `0xH027de4=3`), escrita a
partir das **notas de memória** do jogo: endereço, tamanho e significado. As notas de
qualquer jogo podem ser lidas sem login pela mesma API do RAIntegration,
`https://retroachievements.org/dorequest.php?r=codenotes2&g=<ID>` (a página `codenotes.php` do site é bloqueada pelo Cloudflare). O Vandal Hearts II
(PS1, jogo 11356), por exemplo, tem 2.810 notas.

Com 2 KB de RAM e o emulador próprio, boa parte disso pode ser automatizada no PV-1000:
1. **Achar os endereços:** rodar cada jogo sem janela com entradas programadas (morrer,
   pontuar, passar de fase) e comparar a RAM com `--dump-ram`, para localizar pontuação,
   vidas, fase e estado de jogo;
2. **Gerar as notas** de cada um dos 13 jogos, no formato do RA;
3. **Escrever rascunhos de conquistas** ("chegar à fase 3", "10.000 pontos") e
   conferi-los reproduzindo as mesmas partidas no emulador.

**Limites:** um rascunho não substitui o teste jogando no RALibretro (a conquista precisa
disparar no momento certo e nunca fora dele, com `Delta`, `ResetIf` e contadores contra
disparos falsos); notas erradas geram lógica errada; e só desenvolvedores do RA publicam
conquistas, com reivindicação do set e revisão. O objetivo é chegar ao pedido do console
com o core, as notas e os rascunhos dos 13 jogos prontos. Um bom primeiro teste é o
Pooyan, que já tem a pontuação mapeada (BB22/BB23).
