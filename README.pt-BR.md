# Mil Capivaras

[English](README.md) · **Português** · [Español](README.es.md)

**Mil Capivaras** é um emulador do **Casio PV-1000** (1983) escrito em **Rust**. Ele foi
reimplementado a partir do core do MiSTer FPGA e validado quadro a quadro contra a
simulação daquele hardware. Roda os **13 cartuchos lançados** e **26 homebrews de
PV-1000** (22 do Inufuto, os outros de joe_k, KashtPR e Péricles S.), com interface gráfica
completa:
- biblioteca com capas;
- save states e retroceder;
- gravação em GIF e MP4;
- depurador e trapaças;
- 3 idiomas.

Também vem como **core libretro**, para o RetroArch.

**Feito com IA:** o código e esta documentação foram escritos com o Claude Code (modelo
Claude Opus 5.5), com direção, testes e revisão do autor.

![Biblioteca com capas](screenshots/library-covers.png)

**Por que "Mil Capivaras"?** O nome junta as duas partes do nome do console:
- **Mil** é "1000" em português;
- **CaPiVara** tem, em ordem, as letras **C**, **P** e **V**: **C** de **C**asio e **PV** de
  **PV**-1000.

| | |
|---|---|
| Precisão | **Alta** na escala da Emulation General Wiki: imagem **idêntica pixel a pixel** ao core MiSTer simulado em todos os quadros testados, e RAM **idêntica byte a byte** no quadro 3000, em todos os jogos testados; o som segue a documentação do hardware real, uma oitava abaixo do MiSTer ([precisão](docs/accuracy.pt-BR.md)) |
| Compatibilidade | 13/13 jogos comerciais e 26/26 homebrews: imagem e RAM idênticas à referência no quadro 3000 ([lista](docs/compatibility.pt-BR.md)) |
| Velocidade | ~7.400 quadros/s num Ryzen 7 8700G (~125× o tempo real) |
| Plataformas | Linux (Wayland e X11, AppImage), Windows e FreeBSD; core libretro para o RetroArch no Linux, no Windows e no FreeBSD |
| Licença | GPL-3.0 ou posterior ([`LICENSE`](LICENSE)) |


## Baixar

Os binários estão nos [releases](https://github.com/PSGM-Romhacking/mil_capivaras/releases):

| Arquivo | Sistema |
|---|---|
| `mil-capivaras-0.9.1-linux-x86_64.AppImage` | Linux x86-64 (glibc 2.27 ou mais nova: Debian, Fedora, Arch, openSUSE e derivadas) |
| `mil-capivaras-0.9.1-windows-x86_64.exe` | Windows 10 e 11, 64 bits |
| `mil-capivaras-0.9.1-freebsd-x86_64.tar.gz` | FreeBSD 14 ou mais novo, amd64 (precisa de alguns pacotes, veja o [manual](docs/manual.pt-BR.md#freebsd)) |
| `mil-capivaras-0.9.1-linux-x86_64.AppImage.zsync` | o mapa de atualização do AppImage (usado pelo Gear Lever e pelo AppImageUpdate; não precisa baixar) |
| `mil_capivaras_libretro-0.9.1-linux-x86_64.zip` / `-windows-x86_64.zip` / `-freebsd-x86_64.zip` | core libretro (`.so` / `.dll`) e o `.info`, para a pasta de cores do RetroArch |
| `SHA256SUMS` | as somas SHA-256 de todos os arquivos, para conferir o download (`sha256sum -c SHA256SUMS`) |

No Linux: `chmod +x mil-capivaras-0.9.1-linux-x86_64.AppImage` e rodar. Não precisa
instalar nada. Os releases são montados pelo GitHub Actions a cada tag de versão
(`.github/workflows/release.yml`), e as notas de cada versão vêm do
[`CHANGELOG.md`](CHANGELOG.md).

ROMs: o emulador abre uma pasta de ROMs (`.pv1`, `.bin`, `.rom`, também dentro de `.zip` ou
`.7z`) e reconhece os jogos pelo conteúdo. Na primeira execução, a tela de boas-vindas traz
links para jogos gratuitos e legais, como os do Inufuto.

## Controles padrão

| Ação | Teclado | Controle |
|---|---|---|
| Direcional | setas | d-pad e analógico esquerdo |
| Fire 1 / Fire 2 | Z / X | botão de baixo / da direita (A / B no Xbox) |
| Select / Start | Tab / Enter | Back / Start |
| J2 (direcional, Fire 1, Fire 2, Select, Start) | WASD, F, G, Q, E | 2º controle |
| Biblioteca: mover / abrir o jogo | setas / Enter | d-pad ou analógico esquerdo / botão de baixo |
| Imagem da biblioteca (capa, título, jogo) | as abas no alto | Select |
| Pausar / reiniciar | P / F1 | — |
| Menu rápido (abrir e fechar) | Esc (no modo sofá) | Home/Guide; Select + Start segurados por meio segundo (opção) |
| Salvar / carregar estado; slot −/+ | F2 / F4; F6 / F7 | — |
| Avanço rápido / retroceder (segurar) | Espaço / Backspace | R2 / L2 |
| Som; volume +/− | M; = / − | — |
| Tela cheia (Esc sai); captura; GIF; MP4 | F11; F12; F9; F10 | — |
| Abrir / fechar jogo / sair | Ctrl+O / Ctrl+W / Ctrl+Q | — |
| Atualizar lista de jogos (relê a pasta de ROMs) | F5 | — |

Tudo é remapeável em **Emulação › Configurar controles e atalhos…**. Sem configuração,
os botões do controle vêm do código que o driver informa (posicional, como no
RetroArch), não de números fixos.

## Compilar

```bash
make                    # cargo build --release + copia para ./mil-capivaras-0.9.1-linux-x86_64
make install            # atalho "Mil Capivaras" no menu de aplicativos + ícones
make install-libretro   # core libretro na pasta de cores do RetroArch
packaging/build_linux.sh      # dist/…AppImage e o core .so (precisa de cargo-zigbuild e zig)
packaging/build_windows.sh    # dist/…exe e .dll, compilados no Linux com MinGW
```

Requisitos: Rust (cargo) e a biblioteca de áudio ALSA (`alsa-lib-devel` no Fedora e
openSUSE, `libasound2-dev` no Debian e Ubuntu, `alsa-lib` no Arch).

Opcionais em tempo de execução: `ffmpeg` (gravar MP4), `curl` (baixar capas) e
`xdg-open` (abrir pastas). Sem eles, só esses recursos ficam indisponíveis.

## Documentação

| Documento | Conteúdo |
|---|---|
| [Manual](docs/manual.pt-BR.md) | todos os recursos; FreeBSD, modo portátil, pastas, capas próprias, linha de comando |
| [Compatibilidade](docs/compatibility.pt-BR.md) | os 13 jogos e 26 homebrews testados, com links para os homebrews |
| [Hardware](docs/hardware.pt-BR.md) | o Casio PV-1000 em detalhe e como cada emulador o reproduz; os dois modelos de emulação |
| [Precisão e validação](docs/accuracy.pt-BR.md) | como o emulador é comparado com o core do MiSTer e com o aparelho; o que falta |
| [Arquitetura](docs/architecture.pt-BR.md) | o código, como a emulação funciona e o porquê de cada decisão |
| [Core libretro](docs/libretro.pt-BR.md) | o core para o RetroArch e o caminho para o RetroAchievements |
| [Banco de jogos](docs/sheets.pt-BR.md) | as fichas dos jogos: formato e regras |
| [Tradução](docs/translating.pt-BR.md) | como traduzir a interface para um idioma novo |
| [Lições aprendidas](docs/lessons.pt-BR.md) | o que o projeto ensinou sobre emulação, Verilator e interface |
| [Referência do MiSTer](reference/README.pt-BR.md) | o core do MiSTer simulado com o Verilator: refaça a validação com as suas ROMs |
| [Referências](docs/references.pt-BR.md) | as fontes citadas nos documentos |
| [Changelog](CHANGELOG.md) | as mudanças de cada versão |

## Licenças

| Parte | Licença |
|---|---|
| Mil Capivaras | GPL-3.0 ou posterior ([`LICENSE`](LICENSE)) |
| [iz80](https://github.com/ivanizag/iz80), de Iván Izaguirre (`vendor/iz80/`, cópia modificada) | BSD-3-Clause |
| Banco de jogos (`assets/games/`), a partir da lista de software do MAME | CC0-1.0 (lista do MAME) |
| Recorte da Noto Sans CJK JP | SIL Open Font License 1.1 |

As ROMs não fazem parte do projeto. Os homebrews do Inufuto são gratuitos no site do Inufuto
([inufuto.web.fc2.com](https://inufuto.web.fc2.com/8bit/)), sem licença de redistribuição.
