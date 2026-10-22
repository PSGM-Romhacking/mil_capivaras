# Referência: o core do MiSTer simulado com o Verilator

[English](README.md) · **Português** · [Español](README.es.md)

A ferramenta que valida o Mil Capivaras. Ela simula ciclo a ciclo, com o Verilator, o
Verilog do [core do MiSTer para o PV-1000](https://github.com/MiSTer-devel/Casio_PV-1000_MiSTer),
roda uma ROM com a mesma entrada do emulador e grava a imagem e a RAM, para comparar os dois
byte a byte. O método e os resultados estão em
[Precisão e validação](../docs/accuracy.pt-BR.md).

Não faz parte do emulador: nada aqui é compilado pelo `cargo build`.

## Uso

Requisitos: Verilator 5 e um compilador C++17; o `7z`, para o `compare.sh` ler ROMs em
`.zip`/`.7z`.

```bash
make -C reference                         # gera reference/pv1000-ref (alguns segundos)
cargo build --release                     # gera o emulador
reference/compare.sh target/release/mil_capivaras roms/*.bin
```

O `compare.sh` roda cada ROM nos dois, com `--press-start` (Start nos quadros 120–129 e,
a partir do 200, Fire 1 + direita alternando a cada 20 quadros), e mostra os pixels e os
bytes de RAM diferentes em cada quadro (0 = idênticos). Os quadros são 300 e 3000 por
padrão; outra lista vai em `FRAMES`, por exemplo `FRAMES="2 50 300 1000 3000"`. A referência leva
cerca de 30 s para 3000 quadros (o emulador, menos de 1 s): com os quadros padrão, uns 35 s
por jogo.

Só a referência:

```bash
reference/pv1000-ref JOGO.bin --shot 3000 saida.ppm --press-start --dump-ram ram.bin
```

O emulador aceita as mesmas opções (`--shot`, `--press-start`, `--dump-ram`) e, por
padrão, roda o modelo MiSTer, que é o comparado aqui.

## O que esperar

No quadro 3000, imagem e RAM idênticas em todos os jogos da
[lista de compatibilidade](../docs/compatibility.pt-BR.md) (os homebrews nas versões
indicadas lá). Em outros quadros, alguns jogos diferem em 1 ou 2 bytes da pilha (o Amidar no
quadro 300, por exemplo): um efeito de gravar a RAM no meio de uma instrução que
atravessa o fim do quadro, explicado em [Precisão e validação](../docs/accuracy.pt-BR.md).
As ROMs não vêm junto.

## Arquivos

| Arquivo | O que é |
|---|---|
| `rtl/CasioPV1000.v` | o core do MiSTer (Flandango), com os ajustes para a simulação listados em `rtl/CasioPV1000.sim.patch` |
| `rtl/sim_top.v` | o módulo de topo, no lugar do framework do MiSTer: relógios, RAM de 64 KB, carga do cartucho |
| `rtl/cpu_z80.v` | a interface da CPU do core, com o TV80 no lugar do T80 (VHDL, que o Verilator não lê) |
| `rtl/tv80/` | o TV80, o Z80 em Verilog de Guy Hutchison |
| `sim.h` | a simulação: relógios, captura da imagem, um quadro |
| `ref_main.cpp` | a linha de comando (`pv1000-ref`) |
| `compare.sh` | roda a referência e o emulador e compara os dois |

A simulação dispensa o `clk_sys` de 71,59 MHz do core para rodar rápido o bastante: E/S e
interrupções rodam na descida do `clk_cpu`, o som no `clk_vdp`, e a imagem é desenhada pelo
`sim.h` a partir da RAM, com a mesma decodificação da máquina de estados de vídeo do core.
Três detalhes da simulação foram corrigidos antes das comparações: o TV80 é compilado com
`TV80_REFRESH` (sem isso, o registrador R é sempre 0), o quadro termina quando a varredura
volta à linha 0 (e não no VSync), e o reset é solto numa fase fixa entre os relógios da CPU
e do vídeo.

## Licenças

| Parte | Licença |
|---|---|
| Core do MiSTer (`rtl/CasioPV1000.v`) | GPL-2.0 (`rtl/LICENSE`) |
| TV80 (`rtl/tv80/`) | MIT (`rtl/tv80/LICENSE`) |
| O resto desta pasta | GPL-2.0 ou posterior |

O resto da pasta é GPL-2.0 ou posterior, e não GPL-3.0 como o emulador, porque é compilado
num programa só com o core do MiSTer, que é GPL-2.0. A referência é um programa à parte: não
é ligada ao emulador.
