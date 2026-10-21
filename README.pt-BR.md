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

Este documento é ao mesmo tempo o manual do emulador e uma referência técnica do
console. Tudo o que está aqui foi tirado do código, de medições ou das fontes listadas
em [Referências](#13-referências). Onde as fontes discordam, a divergência aparece.

| | |
|---|---|
| Precisão | **Alta** na escala da Emulation General Wiki: imagem **idêntica pixel a pixel** ao core MiSTer simulado em todos os quadros testados, e RAM **idêntica byte a byte** no quadro 3000, em todos os jogos testados; o som segue a documentação do hardware real, uma oitava abaixo do MiSTer ([seção 9](#9-precisão-e-validação)) |
| Compatibilidade | 13/13 jogos comerciais e 26/26 homebrews: imagem e RAM idênticas à referência no quadro 3000 ([seção 10](#10-compatibilidade)) |
| Velocidade | ~7.400 quadros/s num Ryzen 7 8700G (~125× o tempo real) |
| Plataformas | Linux (Wayland e X11, AppImage), Windows e FreeBSD; core libretro para o RetroArch no Linux, no Windows e no FreeBSD |
| Licença | GPL-3.0 ou posterior ([`LICENSE`](LICENSE)) |

---

## Índice

1. [O Casio PV-1000](#1-o-casio-pv-1000)
2. [Hardware em detalhe](#2-hardware-em-detalhe)
3. [Como as implementações diferem](#3-como-as-implementações-diferem)
4. [Recursos do emulador](#4-recursos-do-emulador)
5. [Instalação e uso](#5-instalação-e-uso)
6. [Arquitetura do código](#6-arquitetura-do-código)
7. [Como a emulação funciona](#7-como-a-emulação-funciona)
8. [O porquê de cada decisão](#8-o-porquê-de-cada-decisão)
9. [Precisão e validação](#9-precisão-e-validação)
10. [Compatibilidade](#10-compatibilidade)
11. [Core libretro (RetroArch)](#11-core-libretro-retroarch)
12. [Lições aprendidas](#12-lições-aprendidas)
13. [Referências](#13-referências)
14. [Licenças](#14-licenças)

---

## 1. O Casio PV-1000

### História

- **Lançamento:** outubro de 1983, só no Japão, por **¥14.800** [W1][W2]. Saiu no mesmo
  mês do **PV-2000** ("Rakugaki"), o modelo mais caro da Casio. Os dois são
  plataformas distintas, sem compatibilidade de software [W2].
- **Concorrência:** brigava em preço com o **Famicom** da Nintendo e o **SG-1000** da
  Sega, lançados no mesmo ano e na mesma faixa de preço, mas não chegou perto das
  vendas deles [W2]. Não há números de vendas publicados.
- **Fim:** foi retirado do mercado em menos de um ano. Segundo a retrogames.co.uk, saiu
  das lojas em poucas semanas por vender pouco [W1]. A Wikipédia em japonês aponta
  também a mudança da Casio para o **MSX** (o PV-7, de 1984) [W2].
- **Sucessor:** na linha de consoles da Casio, o próximo foi o **Loopy** (1995) [W1].
- **Biblioteca:** **13 cartuchos**, numerados de GPA-101 a GPA-115. Os números
  **GPA-108** e **GPA-113** ficaram vagos e nunca foram lançados; especula-se que
  seriam *Galaga* e *Front Line*, que saíram no PV-2000 [W1]. A lista do MAME anota
  "GPA-113: Front Line?" [M2].
- **Controle:** vinha junto um joystick em forma de manche de avião de caça [W2], o
  **TJ-1**. Na foto do Evan-Amos (Wikimedia Commons), ele tem um botão vermelho no topo
  do manche e os botões Start e Select na base.

### A biblioteca

Os 13 jogos, com o código do cartucho e a origem, segundo a lista de software do MAME
[M2] e as Wikipédias [W1][W2]:

| Cartucho | Jogo | Ano | Origem |
|---|---|---|---|
| GPA-101 | Pooyan (プーヤン) | 1982 | Konami (arcade, 1982) |
| GPA-102 | Super Cobra (スーパーコブラ) | 1981 | Konami (arcade, 1981) |
| GPA-103 | Tutankham (ツターンカーム) | 1982 | Konami (arcade, 1982) |
| GPA-104 | Amidar (アミダー) | 1982 | Konami (arcade, 1981) |
| GPA-105 | Dig Dug (ディグダグ) | 1982 | Namco (arcade, 1982) |
| GPA-106 | Warp & Warp (ワープ&ワープ) | 1981 | Namco (arcade, 1981) |
| GPA-107 | Turpin (ターピン) | 1981? | Konami (arcade "Turtles", 1981) |
| GPA-109 | Pachinko-UFO (パチンコUFO) | 1982 | original da Casio [W2] |
| GPA-110 | Fighting Bug (ファイティング バグ) | 1982 | Universal (arcade "Lady Bug", 1981) |
| GPA-111 | Space Panic (スペースパニック) | 1981? | Universal (arcade, 1980) |
| GPA-112 | Naughty Boy (ノーティーボーイ) | 198? | Jaleco (arcade, 1982) |
| GPA-114 | Dirty Chameleon (ダーティー カメレオン) | 198? | Jaleco [W2] |
| GPA-115 | Excite Mahjong (エキサイト麻雀) | 1982 | original da Casio, o último lançado [W2] |

A coluna "Ano" é a do MAME, inclusive as interrogações dele. O publisher de todos é a
Casio.

**Homebrews:** o desenvolvedor japonês **Inufuto** publica jogos novos para dezenas de
máquinas de 8 bits, incluindo o PV-1000, com ROMs gratuitas no site do Inufuto [I1] e,
para 12 dos jogos de PV-1000, o código-fonte. São **22 títulos para PV-1000** (lista completa na [seção 10](#10-compatibilidade)).
O MAME cataloga 19 deles [M2]. Outros autores também fizeram homebrews de PV-1000: PVong
(joe_k), Simon (KashtPR), Demônios e a PV-1000 Control Test (Péricles S.).

---

## 2. Hardware em detalhe

Esta seção descreve o console **como o core do MiSTer o implementa**, que é a
especificação que o Mil Capivaras segue. Ela foi conferida contra o MAME e o Obentou
([seção 3](#3-como-as-implementações-diferem)). Não existe documentação oficial
publicada do chip de vídeo e som: o autor do core diz que o fez "com base no MAME, em
vídeos e na força bruta" [F1]. As partes sobre o **aparelho** (a placa, o quadro, o
circuito do controle, o conector) vêm das medições do NESdev [N1]–[N7].

### Especificação

| Componente | Detalhe | Fonte |
|---|---|---|
| CPU | Zilog **Z80** (NEC D780C-1, compatível com Z80A) a **3,579545 MHz** | [W1][W2] |
| Vídeo e som | chip customizado **NEC D65010G031** | [W1][W2][F1] |
| RAM | **2 KB** em 0xB800–0xBFFF, compartilhada entre CPU e vídeo | [W1]; medido |
| Cartucho | ROM de **8 KB ou 16 KB** em 0x0000 (até 32 KB endereçáveis) | MAME [M2]; medido |
| Imagem | mapa de **32×24 tiles** de 8×8 pixels = 256×192; visíveis **224×192** (28 colunas) | [W1]; core [C1] |
| Cores | **8** (RGB de 1 bit por componente) | [W1][W2] |
| Padrões | 256 por banco; os tiles 0xE0–0xFF podem vir da RAM (**PCG**, 32 padrões) | core [C1] |
| Som | **3 canais de onda quadrada**, período de 6 bits | [W1] |
| Controle | 2 portas; joystick + 2 botões de tiro + Select + Start, lidos numa matriz | core [C1] |
| Quadro | **59,538 Hz** (263 linhas de 381 pixels) no core; **59,299 Hz** (262 linhas de 288 pixels) no aparelho | core [C1]; [N4] |

**Divergências entre as fontes:**
- **RAM:** a Wikipédia em japonês fala em "RAM 3 KB, VRAM 16 KB" [W2]; a em inglês, em
  2 KB compartilhados [W1]. O core e os 35 jogos testados usam os 2 KB de
  0xB800–0xBFFF (só o Battlot escreve também acima de 0xDC00, onde nada responde). Os "16 KB" provavelmente contam a ROM de padrões do cartucho.
- **Resolução:** a Wikipédia em inglês diz 224×192 no texto e 240×192 na ficha [W1]; a
  japonesa, 240×192 [W2]. No core do MiSTer a área visível é a do hc 17 ao 239, ou seja,
  **223 pixels** mais 1 coluna preta (224), sem as 2 colunas de tiles de cada lado.
- **Borda:** no aparelho real, a borda colorida que alguns jogos usam fica **em cima e
  embaixo**. O MAME a punha dos lados até a versão 267; o autor do core conferiu em vídeos
  do console real [F1].

### A placa e o chip

Dentro do console há pouca coisa [N1]: o **Z80**, uma **SRAM de 2 KB**, o chip
customizado, alguma lógica discreta, um **cristal de 17,897727 MHz** e a parte analógica
(vídeo e som).

- **O chip** é um *gate array* da NEC, do serviço µPD65010 de CMOS de 3 µm, com 1.368 × 2
  transistores de cada tipo (n-MOS e p-MOS). O número completo, **D65010G031**, indica a 31ª
  encomenda nesse formato: encapsulamento QFP de 64 pinos, 20 × 14 mm [N1].
- **Os pinos contam como ele funciona** [N6]:
  - **vídeo digital:** saídas separadas de vermelho, verde e azul (1 bit cada), mais
    sincronismo composto e *colorburst*. A cor NTSC é montada fora do chip;
  - **som analógico por fora:** cada canal quadrado sai num pino próprio (8, 9 e 10) e os
    três são somados por resistores na placa. Daí os volumes diferentes de cada canal;
  - **barramento:** o chip pede o barramento ao Z80 com **/BUSRQ** e espera o **/BUSACK**
    para ler a RAM durante a imagem (ver [Um quadro no aparelho](#um-quadro-no-aparelho));
  - **controles:** a porta FD liga 4 pinos de saída (52–55) e 4 de entrada (60–63) direto
    às duas portas de controle. Pelo contexto da placa, os pinos 50 e 51 seriam mais duas
    saídas, cortadas numa revisão.
- **Relógios a partir do cristal:** o Z80 recebe cristal ÷ 5 (3,579545 MHz), o pixel é
  cristal ÷ 4 (4,47 MHz) e o som conta em cristal ÷ 2 [N2][N4].

### Relógios

Tudo deriva de um oscilador de **71,590909 MHz** (20 × a frequência de cor NTSC):

| Sinal | Divisor | Frequência | Uso |
|---|---|---|---|
| `clk_sys` | 1 | 71,590909 MHz | máquina de estados do vídeo no FPGA |
| `clk_snd` | ÷4 | 17,897727 MHz | gerador de som |
| `clk_cpu` | ÷20 | **3,579545 MHz** | Z80 |
| `clk_vdp` | ÷12 | 5,965909 MHz | 1 pixel por ciclo |

- **Linha:** 381 pixels (hc 0..380) = 63,86 µs = **228,6 ciclos de CPU**.
- **Quadro:** 263 linhas (vc 0..262) = 100.203 pixels = **60.121,8 ciclos de CPU** =
  **71.590.909 / 1.202.436 ≈ 59,538 quadros/s**. Não são 60, e o Mil Capivaras mantém
  isso (ver [seção 8](#8-o-porquê-de-cada-decisão)).

### Um quadro no aparelho

O core do MiSTer usa outra temporização (acima). No aparelho, segundo as medições do
NESdev [N4][N6][N7], o quadro tem **288 pixels × 262 linhas**:

| Grandeza | Valor |
|---|---|
| Pixel | cristal ÷ 4 = **4,474 MHz**, proporção **48:35** (mais largo que alto) |
| Linha | 288 pixels = **230,4 ciclos de Z80**; 15.536 linhas/s |
| Quadro | 262 linhas = **60.364,8 ciclos**; **59,299 quadros/s** |

**Na horizontal** (pixel 0 = início do sincronismo):

| Pixels | Trecho |
|---|---|
| 0–21 | sincronismo horizontal |
| 22–35 | *colorburst* |
| 36–51 | preto (*back porch*) |
| **52–275** | **224 pixels**: imagem nas linhas ativas, cor da borda nas linhas de borda |
| 276–287 | preto (*front porch*) |

**Na vertical** (linha 0 = primeira linha de imagem):

| Linhas | Quantas | Trecho |
|---|---|---|
| **0–191** | 192 | **imagem** (tiles) |
| 192–217 | 26 | borda de baixo |
| 218–220 | 3 | apagamento |
| 221–223 | 3 | sincronismo vertical |
| 224–235 | 12 | apagamento |
| 236–261 | 26 | borda de cima |

```
linha   0 ┌────────────────────────────────┐  ◄ imagem: o chip toma o barramento
          │ 192 linhas de tiles            │    do pixel 20 ao 267 de cada linha
      191 │                                │
      192 ├────────────────────────────────┤
          │ 26 linhas de borda             │  ◄ 195: 1ª IRQ da matriz, depois 1 a cada 4 linhas
      217 ├────────────────────────────────┤
          │ 3 apagadas · 3 de sincronismo  │
          │ vertical · 12 apagadas         │
      236 ├────────────────────────────────┤
          │ 26 linhas de borda             │  ◄ 255: 16ª IRQ da matriz e IRQ de pré-render
      261 └────────────────────────────────┘
```

**O roubo de barramento.** O chip e a CPU dividem o mesmo barramento. Em cada uma das
192 linhas de imagem, o chip mantém **/BUSREQ** do pixel 20 ao 267 (248 pixels): o Z80
para, e o chip busca na RAM, a cada 8 pixels, o número do tile e os 3 bytes de cor
[N4][N6]. As contas:

| | Ciclos de Z80 | Livres para o jogo |
|---|---|---|
| Linha de imagem | 230,4, dos quais até **198,4 parados** | **32 a 37** |
| Quadro inteiro | 60.364,8 | **22.272 a 23.232 (37–38%)**: 6.144 a 7.104 nas linhas de imagem + 16.128 nas outras 70 |
| Tela desligada (bit 3 da porta FF) | 60.364,8 | todos: o chip não busca nada e o Z80 fica mais rápido |

Ou seja: **com a imagem ligada, o jogo tem pouco mais de um terço do Z80.** A faixa de 32 a
37 ciclos vem de o Z80 só ceder o barramento no início de um ciclo de máquina: a instrução
em andamento ainda avança até 5 ciclos depois do pedido [N7]. Isso torna o tempo das linhas
de borda, logo depois das interrupções, o mais valioso do quadro. O modelo de hardware do
Mil Capivaras emula esse roubo ([seção 7](#o-quadro-no-modelo-de-hardware-real)).

**Ligar a imagem no meio de uma linha** desenha de 8 a 16 pixels de lixo, de origem
desconhecida [N2][N6].

**A imagem quase não cabe na TV.** Os 224 pixels ocupam 50,06 µs, **96% da linha NTSC**
(52,15 µs). Os consoles da época ficavam em ~92% (NES, Master System, Mega Drive), e as TVs
domésticas mostravam no máximo uns 92%: na prática, **uns 8 pixels de cada lado se perdem
no overscan** [N7]. Por isso o MAME corta as laterais desde a versão 268, de propósito. O
Mil Capivaras mostra os 224 pixels inteiros nas duas opções de imagem.

**Por que SRAM.** Fora da imagem, o chip passa 70 linhas sem ler a memória: 4,5 ms. Uma
DRAM precisaria aguentar esse tempo sem refresh; a do SG-1000 (MB8118-12), por exemplo, é
garantida só por 2 ms [N7].

### Notas de quem programa no aparelho

Do tópico do fórum do NESdev onde o hardware foi estudado [N7]:
- **Porta FE com valores baixos:** escrever 0 a 3 faz o chip ler o mapa de tiles da
  **ROM**, em 0x0000–0x02FF, e o PCG de 0x0C00–0x0FFF. O mapa pode começar em qualquer
  bloco de 1 KB dentro de um bloco de 4 KB, mas o PCG fica sempre no último 1 KB desse bloco
  (com 0xB8: mapa em 0xB800, PCG em 0xBC00).
- **Atraso da interrupção:** do pedido até o efeito visível no tratador (trocar a cor da
  borda) passam **55 ciclos**, uns 70 pixels.
- **Interrupção sem reconhecimento:** com as interrupções da CPU ligadas, um tratador que
  não reconhece a de pré-render (escrevendo na porta FD) é chamado de novo para sempre.
- **Quando rodar a lógica do jogo:** na primeira interrupção da matriz depois da de
  pré-render, perto do início das linhas sem imagem. O PVong marca uma variável na de
  pré-render e outra na da matriz, e roda o jogo quando as duas estão marcadas.
- **Cores lavadas:** no aparelho, as cores saem visivelmente menos saturadas que o RGB puro
  (vermelho e amarelo principalmente). O chip gera RGB digital (1 bit por cor) em 5 V, e a
  cor de TV é montada depois, na placa.
- **Relato não confirmado:** em julho de 2025, um programador viu no MAME 0.278 o aviso de
  quadro (bit 0 da porta FC) não aparecer com a FC em 1, só em 3. A ROM de teste dos
  controles usa a FC em 1 e roda no MAME 0.289.

### Mapa de memória

| Endereço | Tamanho | Conteúdo |
|---|---|---|
| 0x0000–0x7FFF | 32 KB | **ROM do cartucho** (só leitura; escritas são ignoradas). Também guarda os padrões de tiles. |
| 0x8000–0xB7FF | — | **espelhos** dos 2 KB de RAM: no aparelho, 0x8000–0xBFFF são 8 cópias de 0xB800–0xBFFF [N1]. O core do MiSTer não espelha; nenhum dos 35 jogos usa esses endereços |
| 0xB800–0xBAFF | 768 B | **mapa de tiles**: 32 colunas × 24 linhas, 1 byte = número do tile |
| 0xBB00–0xBBFF | 256 B | RAM de trabalho (variáveis, pilha) |
| 0xBC00–0xBFFF | 1 KB | **PCG**: padrões dos tiles 0xE0–0xFF (32 × 32 bytes), ou RAM comum se o jogo não usar PCG |
| 0xC000–0xFFFF | — | **aberto** no aparelho: nada responde. No modelo de hardware, escritas se perdem e leituras dão `0xFF`; no modelo MiSTer, o que é escrito fica guardado, como no core (o Battlot escreve ali, além do mapa) |

O mapa de tiles e o PCG são RAM comum: a CPU e o vídeo leem e escrevem os mesmos
endereços. Medido nos 35 jogos: só o Battlot escreve fora de 0xB800–0xBFFF (acima de 0xDC00).

### Portas de E/S

O chip decodifica só o byte baixo do endereço de E/S. Ler uma porta fora de F8–FF
devolve o conteúdo da RAM naquele endereço: é um efeito do barramento do core.

| Porta | Escrita | Leitura |
|---|---|---|
| **F8, F9, FA** | período dos canais de som 1, 2 e 3 (6 bits; 63 = mudo) | o último valor escrito |
| **FB** | **som:** bit 1 = ligado (0 emudece os 3 canais); bit 0 = modo XOR (canal 1 toca 1 ⊕ 2, canal 2 toca 2 ⊕ 3) [N2]. O core do MiSTer ignora esta porta | o último valor escrito |
| **FC** | — | **status:** bit 0 = "quadro novo", vai a 1 na linha Y=195 e zera ao ler; bit 1 = há linhas da matriz do joystick ainda não lidas |
| **FD** | **seleciona as linhas** da matriz do joystick (bits 0–3) e marca as 4 como pendentes | **lê a linha selecionada** (4 bits) e a marca como lida |
| FE | guardado, sem efeito no core nem no MAME; no aparelho, endereço do mapa e do PCG ([seção 3](#3-como-as-implementações-diferem)) | o último valor escrito |
| **FF** | **vídeo:** bit 5 = banco de padrões da ROM; bit 4 = forçar padrões da ROM também nos tiles 0xE0–0xFF; bits 2–0 = cor da borda (B, G, R) | o último valor escrito |

### Vídeo

**Como um pixel é formado.** Para cada pixel visível, o chip:
1. lê o **número do tile** em `0xB800 + (Y/8)·32 + (X/8)`;
2. calcula o endereço do **padrão**:
   - tiles 0x00–0xDF, ou qualquer tile com o bit 4 da porta FF ligado: na ROM, em
     `(tile + 0x100·banco) × 32`, onde o banco é o bit 5 da porta FF;
   - tiles 0xE0–0xFF sem o bit 4: no **PCG**, em `0xBC00 + (tile − 0xE0) × 32`;
3. lê **3 bytes** do padrão, um por componente de cor, na linha `Y mod 8`: vermelho em
   +8, verde em +16 e azul em +24 (os bytes +0 a +7 não são usados);
4. o bit `7 − (X mod 8)` de cada byte acende ou apaga aquele componente. Por isso há
   8 cores (preto, as 3 primárias, as 3 secundárias e branco), e cada pixel de um tile
   pode ter qualquer uma delas.

**Exemplo real.** A letra "A" da ROM de teste dos controles é o tile **0x41**, nos 32
bytes a partir de 0x41 × 32 = **0x0820**. Os 8 primeiros bytes não são usados; vermelho,
verde e azul são iguais, então o pixel aceso é branco:

```
0x0820  00 00 00 00 00 00 00 00   (não usados)
0x0828  38 44 44 7C 44 44 44 00   vermelho
0x0830  38 44 44 7C 44 44 44 00   verde
0x0838  38 44 44 7C 44 44 44 00   azul

linha 0  38 = 0011 1000   . . █ █ █ . . .
linha 1  44 = 0100 0100   . █ . . . █ . .
linha 3  7C = 0111 1100   . █ █ █ █ █ . .
linha 7  00 = 0000 0000   . . . . . . . .
```

O mesmo "A" aceso (amarelo sobre vermelho) é o tile **0xC1**: vermelho `FF FF FF FF FF FF
FF 00` (todos os pixels), verde `38 44 44 7C 44 44 44` (só a letra), azul tudo 0. Onde há
vermelho e verde, sai amarelo; onde só há vermelho, sai o fundo vermelho. **Não há
paleta**: cada combinação de cores é outro tile, e por isso o PV-1000 nunca tem mais do
que 256 padrões diferentes na tela.

**Leitura ao vivo.** O chip lê a RAM **durante o próprio pixel**: no core, uma máquina
de estados de ~11 ciclos de `clk_sys` lê o tile e os 3 bytes de cor a cada pixel. Uma
escrita da CPU no meio de uma linha muda a imagem dali em diante, na mesma linha.
Vários jogos fazem isso (Dig Dug, Excite Mahjong, Pachinko-UFO e Dirty Chameleon logo no
quadro 2).

**Geometria (no core):**
- **Horizontal:** o apagamento (HBlank) termina em hc 17 e começa em hc 240. Os pixels
  visíveis vão do X=17 ao X=239 (223 pixels) e mostram as colunas de tiles 2 a 29.
  O Mil Capivaras usa 224 de largura, com a última coluna preta.
- **Vertical:** vc 0..223 são visíveis. As linhas Y = vc − 16 de 0 a 191 têm os tiles;
  as 16 linhas de cima e as 16 de baixo mostram a **cor da borda**.
- **Sincronismo:** sincronismo horizontal de hc 341 a 360; o sincronismo vertical começa
  na linha 259.

**Proporção:** o Mil Capivaras mostra os pixels quadrados (1:1), como o core do MiSTer. A
opção **Imagem: TV** usa a do aparelho, 48:35 ([seção 3](#os-dois-modelos)).

### Som

Esta é a parte em que o Mil Capivaras segue o **hardware real** e não o core do MiSTer.

- **3 canais de onda quadrada**, um em cada porta F8, F9 e FA. Os 6 bits baixos do valor
  escrito (n) dão o período; **n = 63 é mudo**.
- **Frequência:** `f = 17.897.727 / 2 / 1024 / (63 − n) ≈ 8.739 / (63 − n)` Hz, de
  **139 Hz** (n = 0) a **8,7 kHz** (n = 62) [N2][M1]. No Mil Capivaras: um passo do gerador a
  cada 1024 ciclos do relógio de 17,9 MHz, e a onda inverte a cada 63 − n passos.
- **Volumes diferentes por canal**, pela mistura com resistores traçada na placa: F8 em
  −6 dB, F9 em −3 dB, FA em 0 dB (0,5 / 0,75 / 1,0) [N2][M1].
- **Porta FB:** o bit 1 liga o som (no reset ele está desligado, e todos os jogos o ligam
  com 0x02 no primeiro quadro); o bit 0 ativa o **modo XOR**, uma modulação em anel que o
  **Fighting Bug** e o **Space Panic** usam. Dirty Chameleon, Naughty Boy, Super Cobra e
  Warp & Warp emudecem o som por ela em alguns momentos.
- **Filtros passa-alta** de ~50, 72 e 103 Hz nas saídas [N2]: inaudíveis no uso normal,
  audíveis com o modo XOR. Emulados como filtros de primeira ordem em cada canal, antes da
  mistura; tiram o nível contínuo sem mudar o volume (um teste confere 380 Hz com n = 40).

**Como se chegou a isso.** O core do MiSTer gera o som uma **oitava acima** (um passo a
cada 513 ciclos, não 1024), com os canais em volume igual e sem a porta FB. A dúvida foi
resolvida em três etapas:
1. **Afinação:** registrando os valores que os 35 jogos tocam, as notas caem na escala
   musical (Lá ≈ 436 Hz) com a fórmula do MiSTer ou com a do MAME, que diferem
   exatamente uma oitava, e não com a do ePV-1000 (Lá ≈ 428 Hz, pior encaixe), que fica
   descartada.
2. **Oitava:** a faixa das melodias é plausível nas duas oitavas, então isso não decidiu.
3. **Documentação do hardware:** a wiki do NESdev, escrita a partir de ROMs de teste
   rodadas no aparelho e do traçado da placa, dá a fórmula de 8.739 Hz / (63 − n), os
   volumes e a porta FB [N2]. O MAME implementa o mesmo desde a versão 254, com a análise
   do plgDavid (Plogue, autor do *chipsounds*) e os testes do lidnariq [M1].

Medido no Mil Capivaras: o bipe do controle 1 da ROM de teste (n = 40) sai a **380,1 Hz**, e a fórmula dá
8.739 / 23 = 380,0 Hz.

### Controles: a matriz do joystick

Os dois controles são lidos como uma **matriz de 4 linhas × 4 bits**:
1. o jogo escreve na porta **FD** os bits das linhas que quer ler (bits 0 a 3);
2. lê a porta FD: vem a linha selecionada de menor número.

| Linha | Bit selecionado | bit 3 | bit 2 | bit 1 | bit 0 |
|---|---|---|---|---|---|
| 0 | bit 0 | J2 Start | J2 Select | J1 Start | J1 Select |
| 1 | bit 1 | J2 direita | J2 baixo | J1 direita | J1 baixo |
| 2 | bit 2 | J2 cima | J2 esquerda | J1 cima | J1 esquerda |
| 3 | bit 3 | J2 Fire 1 | J2 Fire 2 | J1 Fire 1 | J1 Fire 2 |

A porta **FC** (bit 1) avisa se ainda há linhas pendentes desde a última escrita em FD.
O Dirty Chameleon dependia de um detalhe dessa ordem: o core corrigiu isso em março de
2023 [F1], e o Mil Capivaras veio dessa versão já corrigida.

**O circuito.** O controle é passivo: botões e diodos ligados a um conector
**Mini-DIN-8** [N6]. A linha escrita em FD sai por 4 pinos do chip, passa por resistores
de 10 kΩ, atravessa o diodo e o botão apertado e volta carregando um capacitor de
330 pF; um resistor de 56 kΩ o descarrega quando o botão está solto [N5]:

```
chip ──10kΩ── conector ──|>|── botão ── conector ──┬────┬──10kΩ── chip
                                                 330pF 56kΩ
                                                   │    │
                                                  terra terra
```

Por isso a leitura **não é instantânea** [N5]: depois de selecionar uma linha, o botão
apertado leva **0 a 41 ciclos** de Z80 para aparecer; depois de desselecionar, a ausência
dele leva **126 a 147 ciclos** para ser vista. Um jogo que lê rápido demais vê a linha
anterior. O MAME e o MiSTer leem na hora; o Mil Capivaras simula o atraso no modelo de
hardware real (ver [o que os emuladores erravam](#o-que-os-emuladores-erravam)).

**Os botões de ataque não têm ordem fixa:** uma chave de duas posições no controle troca
os dois, então "Fire 1" e "Fire 2" são só nomes [N5].

### Interrupções

- O Z80 roda em **modo de interrupção 1** (IM 1, desvio para 0x0038). Contando os
  opcodes nas ROMs, nenhum dos 35 jogos usa IM 0 ou IM 2. O emulador também aceita os dois,
  supondo o barramento em 0xFF durante o reconhecimento: o IM 0 vira `RST 38h` e o IM 2 lê o
  vetor em (I × 256) + 0xFF. É uma suposição, não medida no aparelho.
- O chip ativa a interrupção (nível baixo) nas linhas **Y = 195, 199, 203, … 243**: são
  **13 interrupções por quadro**, uma a cada 4 linhas, no fim da imagem. Cada uma fica
  ativa **enquanto hc ≤ 190** (~115 ciclos de CPU), sem reconhecimento.
- **Para que serve:** vários jogos usam esse intervalo para atualizar a tela fora da
  área visível, e alguns contam voltas de um laço até a próxima interrupção para gerar
  números aleatórios (o Excite Mahjong sorteia as peças assim).
- **Divergência:** no MAME são 16 interrupções por quadro, com habilitação na porta FC e
  reconhecimento pela porta FD [M1]; no ePV-1000, também 16 [T1]. Os 13 jogos rodam
  nos dois modelos.

### Cartuchos

- **8 KB** (a maioria dos jogos de 1982) ou **16 KB** (Dig Dug, Pachinko-UFO, Excite
  Mahjong).
- Os padrões de tiles ficam na própria ROM. O bit 5 da porta FF escolhe entre o banco
  0x0000–0x1FFF e o 0x2000–0x3FFF, o que dá **dois conjuntos de 256 padrões** num
  cartucho de 16 KB.
- **Homebrews:** circulam sem o preenchimento do cartucho (ex.: Cacorm com 11.485
  bytes). O MAME os cataloga completados com zeros até 8 ou 16 KB, e o Mil Capivaras
  reconhece os dois formatos.

### O conector do cartucho

O cartucho usa um conector de **36 contatos** (18 de cada lado, passo de 0,1"), com um
**rasgo obrigatório entre os contatos 10 e 11** [N6]. Quase todos os sinais vão direto ao
Z80 e ao chip: endereços A0–A15, dados D0–D7, /RD, /WR, /MREQ, /IORQ e também a
**/IRQ**. Duas exceções:

- **/M07:** não vem direto do Z80 nem do chip; pelo nome, deve ser a seleção da faixa
  0x0000–0x7FFF, gerada pela lógica da placa (a página não detalha);
- **áudio:** um som extra vindo do cartucho é misturado por um resistor de 3,3 kΩ e um
  capacitor de 47 nF, o que forma um **passa-alta de ~1 kHz**.

### Programando o PV-1000: o mínimo

Não há BIOS: o Z80 começa a executar o cartucho no endereço 0. O começo da ROM de teste
dos controles, resumido e comentado, mostra o que um programa precisa fazer. Ela espera o
quadro pela interrupção, como os jogos:

```z80
NEW_FRAME: equ 0xBB12       ; RAM de trabalho: a interrupção marca cada quadro novo

        org 0x0000
        di                  ; sem interrupções até tudo estar configurado
        ld sp,0xBC00        ; pilha logo abaixo dos padrões do PCG (0xBC00–0xBFFF)
        jp start

        ds 0x0038-$
        jp irq              ; IM 1: toda interrupção salta para 0x0038

start:  xor a
        out (0xFF),a        ; borda preta, banco de padrões 0
        ld a,63
        out (0xF8),a        ; os 3 canais mudos (63 = silêncio)
        out (0xF9),a
        out (0xFA),a
        ld a,0x02
        out (0xFB),a        ; liga o som: ele começa desligado
        ld a,0x01
        out (0xFC),a        ; habilita o aviso e a interrupção de "quadro novo" (desligados
                            ; ao ligar no aparelho e no MAME; o core do MiSTer
                            ; dá o aviso de qualquer jeito)
        im 1

main:   ei
        halt                ; espera uma interrupção
        ld a,(NEW_FRAME)    ; foi um quadro novo?
        or a
        jr z,main
        xor a
        ld (NEW_FRAME),a
        ld a,1              ; seleciona a linha 0 da matriz (Select e Start)...
        out (0xFD),a        ; ...o que também reconhece a interrupção no aparelho
        call settle         ; espera o circuito do controle estabilizar (até 41 ciclos)
        in a,(0xFD)         ; lê os botões: bits 0–1 = controle 1, 2–3 = controle 2
        ; ...
        jp main

irq:    push af
        in a,(0xFC)         ; bit 0 = começou um quadro novo
        and 1
        jr z,other          ; o core do MiSTer também interrompe em outras linhas
        ld (NEW_FRAME),a    ; marca o quadro...
        pop af
        reti                ; ...e volta com as interrupções desligadas
other:  pop af
        ei
        reti
```

Depois disso, o programa escreve números de tile no mapa (0xB800 em diante) e os padrões
saem da própria ROM (tile *N* em *N* × 32). O código completo está no
[repositório da ROM](https://github.com/PSGM-Romhacking/pv-1000_control_test_rom).

---

## 3. Como as implementações diferem

Comparação feita lendo o código de cada uma [C1][M1][T1][O1][V1], para escolher a
referência e entender as divergências:

| Aspecto | Core MiSTer | MAME | ePV-1000 (Takeda) | Obentou | Mil Capivaras |
|---|---|---|---|---|---|
| CPU | 3,58 MHz, sem espera | 17,897725/5 MHz | 3,579545 MHz | 3,579545 MHz | igual ao MiSTer |
| CPU parada durante a imagem | não | **sim** (BUSREQ na área ativa) | **sim** (BUSREQ de 800 ciclos em 51 de 67 "linhas") | não | não |
| Quadro | 59,538 Hz; 228,6 ciclos × 263 linhas | 230,4 × 262 | **60 Hz**, modelo simplificado de 67 eventos de linha | 228 × 262 | igual ao MiSTer |
| Interrupções | 13 pulsos por quadro, sem reconhecimento | 16, habilitação em FC, reconhecimento em FD | 16 por quadro (eventos 51–66) | nível nas linhas 196–256, limpa ao ler FC | igual ao MiSTer |
| Matriz do joystick | prioridade à primeira linha selecionada | OU das linhas selecionadas | OU das linhas selecionadas | OU das linhas | igual ao MiSTer |
| PCG / banco | bit 5 de FF = 1 bit de banco; PCG fixo em 0xBC00 | bits 7–5 de FF; FE ignorada (mapa e PCG fixos em 0xB800) | bit 5 de FF = banco; **FE** = endereço do mapa (e do PCG, +0x400) | bits 7–5; FE | igual ao MiSTer |
| Imagem | lida pixel a pixel | por linha | **desenhada de uma vez por quadro**, colunas 2–29 | — | pixel a pixel, como o MiSTer |
| Borda | em cima e embaixo | em cima e embaixo (dos lados até a 267) | sem borda | sem borda | igual ao MiSTer |
| Som | inverte a cada 63−n passos de 513: **17.444/(63−n) Hz** | divisor 1024, mixer em FB: ~uma oitava abaixo | **≈ 9.089/(63−n) Hz**: ~uma oitava abaixo | mixer em FB | **8.739/(63−n) Hz**, volumes e porta FB do hardware [N2] |
| Jogos sem problema conhecido | 13/13 | 13/13 no MAME 0.289 (a lista ainda marca 6 como `supported="no"`, desatualizada: ver abaixo) | não avaliado | — | 13/13 |

**Por que o MiSTer como referência:** é uma descrição do circuito (Verilog) que dá para
simular ciclo a ciclo e comparar byte a byte, e o autor conferiu detalhes em vídeos do
aparelho real (a borda, por exemplo) [F1]. Quando o projeto começou, a lista do MAME
marcava 6 cartuchos como não suportados (Dig Dug, Pachinko-UFO, Fighting Bug, Space Panic,
Naughty Boy, Dirty Chameleon) [M2]. **A marca está desatualizada:** testados em 2026-10-17
no MAME 0.289 (a versão mais recente), os 6 chegam à partida e pontuam; os consertos de
interrupção, leitura de portas e roubo de barramento de julho e agosto de 2024 os fizeram
rodar, mas a lista não foi corrigida. Entre os outros emuladores, o MAME hoje é o mais fiel
ao hardware ([abaixo](#o-core-do-mister-e-o-hardware-real)); o modelo de hardware do Mil
Capivaras chega mais perto nas medições feitas no aparelho
([seção 9](#validação-contra-o-aparelho-as-medições-do-par)).
O fv1000 [V1] ficou fora do quadro: é um experimento em FPGA, em SpinalHDL (wel97459, 2023),
só com o vídeo, sem CPU nem E/S. O único dado útil dele é o quadro que assume, 227,2 ciclos
× 263 linhas, com uma imagem de teste na borda.

### O core do MiSTer e o hardware real

A wiki do NESdev documenta o PV-1000 a partir do aparelho (ROMs de teste rodadas nele e o
traçado da placa) [N1][N2][N3][N4]. Comparando com o core do MiSTer:

| | Hardware real (NESdev) | Core do MiSTer | Mil Capivaras (modelo MiSTer / **hardware real**) |
|---|---|---|---|
| Som | 8.739/(63−n) Hz; volumes −6/−3/0 dB; porta FB (liga, XOR) | uma oitava acima; volumes iguais; sem FB | **como o hardware** |
| RAM | espelhada 8× em 0x8000–0xBFFF | memória plana de 64 KB: só 0xB800–0xBFFF é usada, sem espelhos | como o MiSTer / **como o hardware** |
| CPU durante a imagem | **parada** (BUSREQ) 248 pixels por linha ativa | nunca para | como o MiSTer / **como o hardware** |
| Quadro | 288 pixels (relógio ÷ 4) × 262 linhas = **59,299 Hz** | 381 × 263 = 59,538 Hz | como o MiSTer / **como o hardware** |
| Momento da interrupção | pixel 276 da linha, logo depois da imagem | início da linha, ativa até hc 190 | como o MiSTer / **como o hardware** |
| Interrupções | habilitadas pela porta FC: 16 por quadro (linhas 195–255) + 1 de "pré-render"; reconhecidas pela FD | 13 por quadro, sempre ativas, sem reconhecimento | como o MiSTer / **como o hardware** |
| Matriz do joystick | junta (OU) as linhas selecionadas | só a primeira linha selecionada | como o MiSTer / **como o hardware** |
| Atraso da matriz | o botão solto continua lido como apertado por 126–147 ciclos | instantâneo | como o MiSTer / **como o hardware** |
| Porta FE | endereço do mapa de tiles e do PCG | ignorada | como o MiSTer / **como o hardware** |
| Porta FF | bit 3 desliga a tela; bits 5–7 escolhem entre 8 bancos de padrões | 1 bit de banco | como o MiSTer / **como o hardware** |
| Borda | 26 linhas em cima e embaixo | 16 | 16 / **26 com a opção Imagem: TV** |
| Pixel | proporção **48:35** (não quadrado) | quadrado | quadrado / **48:35 com a opção Imagem: TV** |

O MAME incorporou a maior parte disso entre as versões 254 e 269 (2023–2024) [N3].

**O que isso significa.** Os 39 programas rodam bem com o modelo do MiSTer, que acerta o
comportamento que eles usam. Mas ele é mais permissivo que o aparelho: um programa novo
pode rodar nele e travar no console real. Foi o que aconteceu com a ROM de teste dos
controles, que esperava o aviso de quadro da porta FC sem habilitá-lo.

### O que os emuladores erravam

Em maio de 2023, a página *Emulation* do NESdev listou o que o MAME e o ePV-1000 faziam
errado [N3]. A mesma lista, com a situação do MAME segundo a página (atualizada até 2025)
e a do Mil Capivaras:

| Ponto | MAME | Mil Capivaras (MiSTer / hardware real) |
|---|---|---|
| Volumes diferentes por canal (F8–FA) | corrigido na 254 | sim |
| Porta FB: mudo e modo XOR | corrigido na 254 | sim |
| Porta FC: interrupções habilitáveis e reconhecidas | corrigido na 268 | não / **sim** |
| Porta FD: atraso elétrico da matriz | não | não / **sim** |
| Porta FE: mover o mapa de tiles | não | não / **sim** |
| Porta FE: endereço do PCG | não | não / **sim** |
| Porta FF: ordem dos bits da borda (vermelho e azul) | corrigido na 267 | sim |
| Porta FF: desligar a imagem | corrigido na 268 | não / **sim** |
| Porta FF: mais de 2 bancos de padrões | em parte, na 268 | não / **sim** (8) |
| Roubo de barramento (BUSREQ) | corrigido na 269 | não / **sim** |
| Borda em cima e embaixo, não dos lados | corrigido na 268 | sim (16 linhas, ou 26 com **Imagem: TV**) |
| Pixel 48:35, não quadrado | corrigido na 269 | com **Imagem: TV** |
| RAM espelhada em 0x8000–0xBFFF | não | não / **sim** |
| Efeitos de *raster* (escrever no meio da linha) | não | sim: o vídeo é lido pixel a pixel |

Com a temporização do aparelho e o atraso da matriz (outubro de 2026), o modelo de hardware
do Mil Capivaras cobre os 14 pontos da lista.

### As ROMs de teste do lidnariq no Mil Capivaras

As 7 ROMs de teste do tópico [N7] rodam no Mil Capivaras. Comparando com as fotos do
aparelho publicadas lá:

| Teste | No aparelho | Mil Capivaras | Por quê |
|---|---|---|---|
| `scanwfe` (porta FE) | dígitos grandes em preto sobre tiles "0F", com faixas brancas | a mesma estrutura no modelo de hardware | a porta FE está implementada |
| `wff-v2` (porta FF, bits altos) | a sequência 0R 00 2R 22 4R… | mostra "4R", um dos passos esperados | os 8 bancos e o "forçar" estão implementados |
| `dither-no-ei` | barras de pontilhado | as mesmas barras | só depende dos padrões |
| `irqtiming` (FC = 1) | uma barra branca à esquerda da borda de cima, a cada quadro | **a mesma barra, no mesmo lugar** (antes: nenhuma) | a barra começa 55 ciclos depois da interrupção, o que no aparelho cai no começo da linha seguinte. Na temporização do MiSTer, o apagamento horizontal é mais longo (26,5 µs contra 14,3 µs) e a barra sumia dentro dele |
| `toggle` (liga e desliga a imagem) | o fractal com faixas horizontais em cima e uma escada diagonal embaixo, à esquerda | **o fractal, as faixas e a escada** (antes: outro desenho); a inclinação da escada ainda difere um pouco | depende do tamanho exato da linha e do roubo de barramento |
| `joystick` (com um botão apertado) | as linhas seguintes ainda mostram "1" por um tempo; na de 21 ciclos por coluna, seis | **seis colunas** na linha de 21 ciclos (~126 ciclos), como o lidnariq mediu (antes: o botão sumia na hora) | o atraso elétrico da matriz |

**Conclusão:** antes da temporização do aparelho, as portas FE, FF e FC passavam, e o que
falhava era exatamente a temporização: o tamanho da linha, o roubo de barramento e o atraso
da matriz. Com as duas mudanças de outubro de 2026, os 6 testes comparáveis batem com as
fotos. O `twinkle` toca música e troca a borda ao longo do tempo, então uma captura só não
basta para compará-lo.

### Os dois modelos

A opção **Emulação › Modelo** (e, no RetroArch, *Opções do núcleo › Model*) escolhe:

| Modelo | O que segue | Para quê |
|---|---|---|
| **Core do MiSTer (validado)**, o padrão | o core do MiSTer, byte a byte ([seção 9](#9-precisão-e-validação)) | jogar; é o modelo comprovado nos 39 programas |
| **Hardware real (NESdev / MAME)** | o aparelho, onde o NESdev e o MAME 0.289 documentam: **quadro de 288 × 262 a 59,299 Hz**, **CPU parada enquanto a imagem é desenhada**, interrupções no pixel 276, só quando habilitadas na porta FC e reconhecidas pela FD; matriz em OU e com o **atraso elétrico**; portas FE e FF completas (endereço do mapa e do PCG, 8 bancos de padrões, tela desligada); portas sem dono lidas como `0xFF` | desenvolver e testar ROMs novas: o que trava ou fica lento no console trava ou fica lento aqui também |

**Validação do modelo de hardware** (com a temporização do aparelho, outubro de 2026):
- **todos os programas do banco de jogos** (13 jogos, 22 homebrews do Inufuto, PVong, Simon,
  Demônios e a ROM de teste dos controles) rodam nele, com Start e tiros automáticos até o quadro 3000;
- **os jogos ficam mais lentos onde a CPU não dá conta**, como esperado com ~37% do Z80:
  no Amidar, o *game over* do jogador automático chega no quadro ~2900 em vez de ~2600, e
  a tela é apagada aos poucos, em vários quadros;
- **as ROMs de teste do lidnariq** batem com as fotos do aparelho
  ([acima](#as-roms-de-teste-do-lidnariq-no-mil-capivaras));
- **a ROM de teste dos controles** roda nele, e uma cópia sem a habilitação da porta FC
  trava, como no MAME;
- **testes automáticos** cobrem o aviso de quadro, o `HALT`, a matriz em OU, o atraso da
  matriz, o roubo de barramento (um laço tem 36–40% das voltas com a imagem ligada em
  relação à desligada) e a taxa de 59,299 Hz;
- **as medições do Par em dois PV-1000** batem com o emulador em todos os 8 cenários do
  teste de ciclos e nas 3 posições do teste de divisão de tela
  ([seção 9](#validação-contra-o-aparelho-as-medições-do-par));
- **o Tutankham** entra no jogo nos dois emuladores; no MAME 0.289, que fica ~190–200 ciclos
  por quadro abaixo do aparelho, a partida automática chega ao *game over* antes.

**A imagem** é uma opção à parte, **Emulação › Imagem** (no RetroArch, *Picture*), que só
muda a apresentação e vale para os dois modelos:

| Imagem | Quadro | Na tela |
|---|---|---|
| **MiSTer**, o padrão | 224 × 224: 192 linhas de jogo e 16 de borda em cima e embaixo | pixel quadrado |
| **TV** | 224 × 244: 192 linhas de jogo e 26 de borda em cima e embaixo, como no aparelho e no MAME | pixel 48:35, mais largo que alto: a imagem fica uns 26% mais larga que alta, como numa TV da época |

Na imagem TV, as faixas de borda repetem a cor da primeira e da última linha de borda
emuladas. As capturas de tela e gravações saem no tamanho do quadro (224 × 244), sem
esticar.

**O modelo MiSTer não mudou:** a temporização do aparelho só vale no modelo de hardware. No
modelo MiSTer, a CPU nunca para e o quadro é de 59,538 Hz, e a comparação byte a byte com
o core continua valendo: as comparações de imagem e RAM da regressão (quadros 300 e 3000)
saíram idênticas depois da mudança.

**A taxa de quadros acompanha o modelo:** 59,538 Hz no modelo MiSTer e 59,299 Hz no de
hardware. O ritmo da janela, o som, o contador de velocidade, as gravações em MP4 e, no
RetroArch, a taxa informada ao frontend (com `SET_SYSTEM_AV_INFO` ao trocar de modelo)
seguem o modelo escolhido.

---

## 4. Recursos do emulador

![Biblioteca: tela de jogo](screenshots/library-screens.png)
*Biblioteca: tela de jogo*

![Ficha do jogo](screenshots/game-info.png)
*Ficha do jogo*

![Moldura: console e controles](screenshots/game-console.png)
*Moldura: console e controles*

![Moldura: TV](screenshots/game-tv.png)
*Moldura: TV*

![Depurador: memória](screenshots/debugger-memory.png)
*Depurador: memória*

![Depurador: CPU](screenshots/debugger-cpu.png)
*Depurador: CPU*

![Depurador: tiles](screenshots/debugger-tiles.png)
*Depurador: tiles*

![Trapaças](screenshots/cheats.png)
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
[`docs/sheets.pt-BR.md`](docs/sheets.pt-BR.md). Os títulos japoneses usam uma fonte embutida (46 KB,
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

![Biblioteca no modo sofá](screenshots/couch-library.png)
*Biblioteca no modo sofá*

![Menu rápido](screenshots/couch-menu.png)
*Menu rápido sobre o jogo*

### Interface

- Barra de menus no padrão de desktop (Arquivo, Emulação, Áudio, Vídeo, Gravação,
  Ferramentas, Ajuda), cores cinza-escuro do Blender, sem cantos arredondados.
- **Idiomas:** inglês (padrão), português e espanhol, trocados ao vivo.
- Preferências, pasta de ROMs, jogos recentes, teclas e controles ficam salvos.
- Ícone próprio, atalho no menu de aplicativos e lançador na pasta (`make install`).

---

## 5. Instalação e uso

### Baixar

Os binários estão nos [releases](https://github.com/PSGM-Romhacking/mil_capivaras/releases):

| Arquivo | Sistema |
|---|---|
| `mil-capivaras-0.9.0-linux-x86_64.AppImage` | Linux x86-64 (glibc 2.27 ou mais nova: Debian, Fedora, Arch, openSUSE e derivadas) |
| `mil-capivaras-0.9.0-windows-x86_64.exe` | Windows 10 e 11, 64 bits |
| `mil-capivaras-0.9.0-freebsd-x86_64.tar.gz` | FreeBSD 14 ou mais novo, amd64 (precisa de alguns pacotes, veja abaixo) |
| `mil-capivaras-0.9.0-linux-x86_64.AppImage.zsync` | o mapa de atualização do AppImage (usado pelo Gear Lever e pelo AppImageUpdate; não precisa baixar) |
| `mil_capivaras_libretro-0.9.0-linux-x86_64.zip` / `-windows-x86_64.zip` / `-freebsd-x86_64.zip` | core libretro (`.so` / `.dll`) e o `.info`, para a pasta de cores do RetroArch |
| `SHA256SUMS` | as somas SHA-256 de todos os arquivos, para conferir o download (`sha256sum -c SHA256SUMS`) |

No Linux: `chmod +x mil-capivaras-0.9.0-linux-x86_64.AppImage` e rodar. Não precisa
instalar nada. Os releases são montados pelo GitHub Actions a cada tag de versão
(`.github/workflows/release.yml`), e as notas de cada versão vêm do
[`CHANGELOG.md`](CHANGELOG.md).

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

### Compilar

```bash
make                    # cargo build --release + copia para ./mil-capivaras-0.9.0-linux-x86_64
make install            # atalho "Mil Capivaras" no menu de aplicativos + ícones
make install-libretro   # core libretro na pasta de cores do RetroArch
packaging/build_linux.sh      # dist/…AppImage e o core .so (precisa de cargo-zigbuild e zig)
packaging/build_windows.sh    # dist/…exe e .dll, compilados no Linux com MinGW
```

Requisitos: Rust (cargo) e a biblioteca de áudio ALSA (`alsa-lib-devel` no Fedora e
openSUSE, `libasound2-dev` no Debian e Ubuntu, `alsa-lib` no Arch).

Opcionais em tempo de execução: `ffmpeg` (gravar MP4), `curl` (baixar capas) e
`xdg-open` (abrir pastas). Sem eles, só esses recursos ficam indisponíveis.

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
[`docs/translating.pt-BR.md`](docs/translating.pt-BR.md).

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

### Controles padrão

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

---

## 6. Arquitetura do código

Um *workspace* Cargo com o núcleo do console separado da interface. Detalhes e decisões
em [`docs/architecture.pt-BR.md`](docs/architecture.pt-BR.md).

```
crates/api/              contrato núcleo ↔ interface (sem dependências)
crates/core/             o console; só depende de emu-api, iz80 e serde/serde_json
crates/frontend/         a interface reutilizável; não conhece nenhum console
crates/app/              o executável: núcleo + interface + identidade do app
crates/libretro/         o núcleo atrás da API C do libretro (mil_capivaras_libretro.so)
vendor/iz80/             a CPU Z80, com ciclos e registrador R corrigidos
assets/                  dados embutidos: ícones, molduras, fontes, traduções, banco de jogos
docs/                    arquitetura, banco de jogos, core libretro, precisão, tradução
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

### `emu-api`: o contrato

- O **trait `Core`**: `load`, `reset`, `run_frame(&[u16])`, `video()`,
  `take_audio()`, `save_state()`, `load_state()`, `system_ram()` / `system_ram_mut()`,
  `debug_views()`, `sound_channels()`, `options()` / `set_option()` e `fps()` (a taxa de quadros
  atual, que no PV-1000 depende do modelo).
- O **`SystemInfo`**: nome, extensões, botões, resolução, proporção, FPS como fração
  exata, endereço da RAM, identificação por hash e fichas.
- Segue as **convenções do libretro**: controle como bitmask RetroPad na mesma ordem de
  bits, imagem XRGB8888, áudio f32 estéreo intercalado, estado em bytes opacos. Por isso
  o core libretro é só uma tradução fina, e trocar de interface não mexe no núcleo.

### `pv1000-core`: o console

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

### `emu-frontend`: a interface

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

### Testes

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

---

## 7. Como a emulação funciona

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
([Um quadro no aparelho](#um-quadro-no-aparelho)).

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

**Portas e vídeo** como antes: a porta FE acha o mapa e o PCG, os 3 bits altos da FF
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
e a porta FB liga o som e escolhe o modo XOR ([seção 2](#som)). As amostras saem na taxa da placa de som (48 kHz) e viram estéreo no
adaptador.

### Save states

O estado é **completo e determinístico**: CPU, RAM 0x8000–0xFFFF, E/S e som, ~33 KB.
Salvar, rodar e restaurar dá exatamente a mesma imagem e RAM; um teste confere. Por
isso o retroceder e, no RetroArch, o run-ahead e o netplay funcionam.

---

## 8. O porquê de cada decisão

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

---

## 9. Precisão e validação

### Como a comparação é feita

**1. Os dois "consoles"**
- **A referência:** o core do MiSTer é a descrição do circuito do PV-1000 (em Verilog).
  Um programa (Verilator) simula esse circuito no PC, fio por fio, relógio por relógio. É
  lento, mas é o próprio hardware do MiSTer rodando.
- **O Mil Capivaras:** o emulador, escrito do zero em Rust, rápido.

Se o Mil Capivaras estiver certo, os dois têm que fazer **exatamente a mesma coisa**.

**2. O mesmo jogo, a mesma "mão"**

Os dois rodam a mesma ROM, sem ninguém jogando. Um "jogador automático" aperta os
botões nos mesmos momentos nos dois:
- aperta **Start** do quadro 120 ao 129 (o jogo sai da tela de título);
- a partir do quadro 200, alterna tiro + direita a cada 20 quadros.

É como dois músicos tocando a mesma partitura: se os dois forem fiéis, o som sai igual.

**3. Pausa e foto no mesmo instante**

Depois de N quadros (o PV-1000 desenha ~60 por segundo, então o quadro 3000 é uns 50
segundos de jogo), os dois param e salvam:
- **a imagem da tela:** 224 × 224 pontos;
- **a memória RAM inteira:** tudo o que o jogo está "pensando" (pontos, vidas, posição
  dos inimigos…).

**4. Comparar, ponto por ponto**

Um script compara os arquivos:
- **imagem:** conta quantos pontos têm cor diferente. 0 = telas idênticas;
- **RAM:** conta quantos bytes têm valor diferente. 0 = o jogo está no mesmo estado nos
  dois.

A RAM é a prova mais forte: dois jogos podem mostrar a mesma tela e ainda estar
"pensando" diferente por dentro.

**5. Repetir em vários momentos e jogos**

Isso foi feito nos quadros 2, 50, 300, 1000 e 3000, para os 39 programas. Se uma diferença
aparecesse num momento e sumisse depois, ou só aparecesse com o jogo rodando há muito
tempo, a comparação pegaria.

**6. Quando dá diferente: achar onde começou**

Testa-se o meio do caminho (quadro 1500, depois 750…) até achar o primeiro quadro em
que os dois se separam. Daí, um rastreio compara os dois **instrução por instrução**,
até o ponto exato onde a CPU dos dois fez coisas diferentes. Foi assim que se descobriu,
por exemplo, que a biblioteca do Z80 contava ciclos errados.

**Resultado:** no quadro 3000, os 39 programas têm tela e memória **idênticas** nos dois. É
a prova de que o Mil Capivaras se comporta como o hardware do MiSTer.

### Metodologia (detalhes técnicos)

1. **Referência:** o Verilog do core MiSTer simulado ciclo a ciclo com o Verilator
   (fora do repositório), com a CPU TV80, que é o T80 do MiSTer em Verilog.
2. **Mesma entrada:** com `--press-start`, os dois apertam Start nos quadros 120–129 e
   alternam Fire 1 + direita a cada 20 quadros a partir do 200.
3. **Mesma saída:** imagem 224×224 e RAM de 64 KB no quadro N.
4. **Comparação:** pixels e bytes diferentes.
5. **Bisseção:** havendo diferença, acha o primeiro quadro, e daí o rastreio instrução a
   instrução (PC, ciclo e R a cada busca de opcode) acha a primeira instrução divergente.
6. **Ferramentas de diagnóstico.** No núcleo: `Pv1000::log_interrupts` registra cada
   interrupção aceita (linha, posição, PC) e `Pv1000::trace`, cada instrução iniciada (linha,
   posição, PC, A, R). Na simulação do core (fora do repositório; o Verilog vem do MiSTer),
   as mesmas medidas mais a RAM a cada quadro e um "vigia" de endereço. Dois cuidados
   aprendidos ao comparar:
   - o TV80 grava o resultado no acumulador já durante a instrução seguinte, então o A lido
     no início dela está atrasado; compare a sequência de PCs, não o A instrução a
     instrução;
   - instruções com prefixo (DD, ED, FD, CB) fazem duas buscas de opcode no TV80 e uma no
     emulador; a interrupção aparece como uma busca a mais no TV80.
7. **Mesma foto.** O emulador roda instruções inteiras; o core escreve no ciclo de máquina.
   Para comparar a RAM no fim de um quadro, a simulação termina a instrução em andamento
   antes de gravar.

### Resultado

- **39 programas** (13 comerciais e 26 homebrews), imagem nos quadros 2, 50, 300, 1000 e
  3000: **0 pixels diferentes em todos**. RAM no quadro 3000: **idêntica nos 39**. No
  quadro 1000, idêntica em 34; nos outros 5, 1 ou 2 bytes de pilha diferem por um artefato da
  comparação, não da emulação (ver "Restos de pilha" abaixo). Detalhes na
  [seção 10](#10-compatibilidade).
- O PC, o ciclo e o R batem **a cada instrução** nas primeiras 40.000 linhas (~152
  quadros) dos 16 jogos rastreados.

**O que foi preciso corrigir para chegar aí:**

| Problema | Onde | Efeito antes da correção |
|---|---|---|
| Ciclos errados no `iz80` (DD/FD, `JR cc`, `JP cc`) | CPU | Excite Mahjong 2,5% diferente; RNG sorteava outras peças |
| R nunca incrementado; `LD A,R` sem S/Z | CPU | Super Cobra, Space Panic e Excite Mahjong usam o R como aleatório |
| Interrupção vista na hora, não 2 ciclos depois | temporização | RAM divergindo depois de ~150 quadros |
| Primeira instrução após o reset 2 ciclos longa | temporização | fase errada para sempre (os laços `JR $` a mantêm) |
| Linha desenhada de uma vez | vídeo | até 80 pixels errados por quadro em telas que mudam durante a imagem |
| Última coluna desenhada | vídeo | 1 coluna diferente |

**A própria referência tinha defeitos**, corrigidos nela antes das comparações:
- o TV80 era compilado sem `TV80_REFRESH` (R sempre 0) e calculava S e Z do `LD A,R` a
  partir do I;
- o quadro terminava no VSync (linha 259), e não na volta à linha 0, o que defasava
  controles e capturas em ~3 linhas;
- a fase entre o relógio da CPU e o do vídeo dependia do tamanho da ROM.

**O que ainda é diferente:**
- **"Restos de pilha" (resolvido: não é erro de emulação):** no Dig Dug, Ruptus, Uchotos e
  Yewdow, 1 ou 2 bytes diferiam em alguns quadros. A explicação antiga ("uma interrupção
  aceita uma instrução antes ou depois") estava errada. A investigação de 2026-10-18 mostrou:
  - as **interrupções** são aceitas no mesmo quadro, na mesma linha e na mesma instrução
    nos dois, em todos os 4 jogos (11.916 a 14.209 interrupções por jogo em 1.000–1.100
    quadros);
  - a **sequência de instruções** é idêntica (comparada busca a busca no quadro em que a
    primeira diferença aparecia);
  - a diferença vem **do instante da comparação**: a RAM é gravada na virada do quadro, e
    uma instrução que está na metade (por exemplo, `LD (IY+0),A` começando no fim da
    linha 262) já escreveu no emulador, que roda instruções inteiras, mas ainda não no
    core, que escreve no ciclo de máquina dela;
  - **prova:** gravando a RAM do core depois de terminar a instrução em andamento, o Dig
    Dug, o Uchotos e o Yewdow dão **zero diferenças em 1.100 quadros**. No Ruptus restam 2
    em 1.100 quadros, ambas de uma instrução que começa exatamente na virada (posição 0 ou
    380), que cada lado põe num quadro por ±1 pixel de diferença na medida do tempo.
  - o **Simon**, comparado depois, tem a mesma sobra: 1 byte em 0xBFE6, o mesmo endereço
    do Uchotos.

  As ferramentas usadas (registro de interrupções e rastreio de instruções no núcleo,
  `log_interrupts`/`trace`, e as mesmas opções na simulação do core) estão descritas na
  [seção 9](#metodologia-detalhes-técnicos).
- **Flags não documentados:** os bits 3 e 5 do registrador F depois de `BIT n,(HL)`. O
  `iz80` os tira do endereço, o TV80 zera e o T80 do MiSTer usa o registrador interno
  MEMPTR. Também só aparece como resto na pilha (Amidar, no quadro 1500).

### Classificação

Na escala da **Emulation General Wiki** [G1]:

| Nível | Definição (resumida) | Mil Capivaras |
|---|---|---|
| Baixa | muitas falhas de imagem e som; velocidade acima de tudo | — |
| Média | boa compatibilidade com os jogos comerciais, sem cobrir todas as nuances | — |
| **Alta** | reproduz os componentes de perto, com mais custo de processamento; poucas falhas, casos-limite tratados; não precisa ser ciclo a ciclo | **sim** |
| Ciclo a ciclo | cada componente anda pelo seu comportamento a cada ciclo, sincronizado no nível de um ciclo de relógio | não, por construção |
| Subciclo | considera as duas bordas de cada ciclo de relógio | não |

**Por que "alta" e não "ciclo a ciclo":**
- a CPU executa **instruções inteiras** (com o número exato de ciclos de cada uma), não
  ciclo a ciclo;
- o momento de cada escrita dentro da instrução vem de uma **regra**, não do barramento
  simulado;
- o vídeo não simula a máquina de estados de ~11 ciclos de `clk_sys` por pixel, só o
  resultado dela.

**O que justifica o topo do nível "alta":** o resultado é **idêntico** ao de uma
simulação ciclo a ciclo do hardware em todos os jogos e quadros testados. Na prática,
o Mil Capivaras é **ciclo-exato nos limites de instrução** (o PC, o ciclo e o R batem a
cada busca de opcode).

**Ressalvas honestas:**
- a referência é o **core do MiSTer**, não um aparelho real. "Idêntico" quer dizer
  idêntico ao core, e o core difere do aparelho em temporização, interrupções, portas FE e
  FF, borda e proporção do pixel ([seção 3](#3-como-as-implementações-diferem)). Nesses
  pontos, o modelo de hardware segue o aparelho, conferido com as medições do Par (abaixo),
  mas sem uma referência para comparar byte a byte;
- o som é a exceção: segue a documentação do hardware, não o core;
- a referência usa o TV80, não o T80 do MiSTer, com a E/S no relógio da CPU em vez do
  `clk_sys`.

### Validação contra o aparelho: as medições do Par

O Par (team-europe) mediu o PV-1000 em **dois consoles** com programas de teste próprios,
publicados com o código-fonte [N9]. Como eles mostram números na tela, dá para rodá-los no
emulador e comparar com o que ele anotou, **sem ter o console**. Os programas ficam em
`_referencias/par/` (fora do git), e dois testes automáticos fazem a comparação
(`par_cycle_counts_match_hardware` e `par_split_lines_match_hardware`, pulados se a pasta
faltar).

**1. Ciclos entre interrupções** (*Cycle Count Before Interrupt*). O programa conta quantos
ciclos o laço principal consegue rodar entre duas interrupções. Há duas versões: uma conta
em passos de 11 ciclos (`ADD HL,DE`), outra em passos de 4 e 6 (`INC E`/`INC DE`). Valores
em hexadecimal, como na tela:

| Versão | Interrupções | Imagem | Par (PV-1000 real) | Mil Capivaras (hardware) |
|---|---|---|---|---|
| ADD | só INT1 | ligada | $57AD / $57B8 | **$57AD** |
| ADD | só INT1 | desligada | $EBBF / $EBCA | **$EBBF** |
| INC | só INT1 | ligada | $56FE / $5702 / $5706 | **$5702** |
| INC | só INT1 | desligada | $EBBE / $EBC2 | **$EBBE** |
| ADD | INT1 + INT16 | ligada | $21B5; demais $038B / $0396 | **$21B5; $038B / $0396** |
| ADD | INT1 + INT16 | desligada | $B5BC / $B5C7; demais $038B / $0396 | **$B5BC; $038B / $0396** |
| INC | INT1 + INT16 | ligada | $2100; demais $038A / $038E | **$2100; $038A / $038E** |
| INC | INT1 + INT16 | desligada | $B5BE / $B5C2; demais $038A / $038E | **$B5BE; $038A / $038E** |

**Todos os valores aparecem entre os que o Par viu no aparelho.** As versões ADD e INC
dão números diferentes com a imagem ligada porque o roubo de barramento corta instruções
de tamanhos diferentes em pontos diferentes, e o emulador reproduz isso.

**2. Onde cai uma divisão de tela** (*Screen Split Tool*). Três rotinas de teste trocam a
configuração do vídeo depois de ~3.520 ciclos, cada uma esperando com instruções diferentes.
No aparelho, a versão com `NOP` dividiu a tela **3 linhas abaixo** da versão com laço em BC,
e a versão com `DJNZ`, **2 linhas abaixo**. No Mil Capivaras: linhas 66, 63 e 65, ou seja,
**3 e 2 linhas abaixo**, exatamente.

**3. A folga do roubo de barramento.** O emulador deixa terminar inteira a instrução que
termina até 4 ciclos depois do pedido de barramento. Variando esse valor, só 4 e 5 ciclos
passam no teste de divisão (2, 3 e 6 falham), o que confirma o que o lidnariq escreveu no
fórum ("até 5 ciclos a mais antes do BUSACK") [N7].

**4. Comparação com o MAME 0.289**, no mesmo teste de ciclos (só INT1, imagem ligada):

| Versão | PV-1000 real | Mil Capivaras | MAME 0.289 |
|---|---|---|---|
| ADD | $57AD (22.445) | **$57AD (22.445)** | $56F2 (22.258) |
| INC | $5702 (22.274) | **$5702 (22.274)** | $563A (22.074) |

O MAME fica ~190–200 ciclos por quadro abaixo do aparelho. Isso explica por que as
partidas automáticas divergem com o tempo entre os dois. O antigo "ponto em aberto" do
Tutankham era isso: com o Start sozinho, o MAME também entra no jogo; o título que aparecia
no quadro 3000 era a volta ao título depois de um *game over* que lá chegou antes.

### O que falta

A lista completa, com a situação de cada item, está em [`docs/accuracy.pt-BR.md`](docs/accuracy.pt-BR.md).
Já resolvidos: os números do Par, o roubo de barramento (a aproximação basta), o Tutankham no
MAME, IM 0/IM 2, a faixa 0xC000–0xFFFF, o ZEXALL, os filtros do som e os "restos de pilha"
(eram um artefato da comparação). Pendentes: detalhes do vídeo durante a imagem, a fase dos
canais no modo XOR e as cores de TV.

---

## 10. Compatibilidade

**Como foi testado:** cada jogo rodou no Mil Capivaras e na referência do MiSTer com a
mesma entrada (Start nos quadros 120–129; Fire 1 + direita alternando a cada 20 quadros a
partir do 200). A imagem foi comparada nos quadros 2, 50, 300, 1000 e 3000, e a RAM de 64
KB nos quadros 1000 e 3000. Primeiro os 35 jogos originais, depois os outros 4 homebrews,
com o mesmo método. **No quadro 3000, imagem e RAM são idênticas nos 39 programas**; a
coluna de RAM abaixo mostra o quadro 1000, o único com alguma diferença.

**Status:** "Perfeito" = imagem idêntica à referência em todos os quadros testados e
RAM idêntica, ou diferente só em posições **já descartadas da pilha** (abaixo do topo
dela; ver [seção 9](#9-precisão-e-validação)).

As ROMs dos homebrews do Inufuto são as do site do Inufuto [I1] (sem o preenchimento do cartucho).
Todos os jogos abaixo têm ficha. Awass, Svellas e Uchotos não estão na lista do MAME:
o banco de jogos os reconhece pelo CRC32 do arquivo distribuído pelo Inufuto.

**Jogos comerciais (13/13)**

| Cartucho | Jogo | Ano (MAME) | ROM | Imagem (5 quadros) | RAM no quadro 1000 | Status |
|---|---|---|---|---|---|---|
| GPA-101 | Pooyan | 1982 | 8 KB | 0 px | idêntica | ✅ Perfeito |
| GPA-102 | Super Cobra | 1981 | 8 KB | 0 px | idêntica | ✅ Perfeito |
| GPA-103 | Tutankham | 1982 | 8 KB | 0 px | idêntica | ✅ Perfeito |
| GPA-104 | Amidar | 1982 | 8 KB | 0 px | idêntica | ✅ Perfeito |
| GPA-105 | Dig Dug | 1982 | 16 KB | 0 px | 2 B em 0xBC5F (pilha) | ✅ Perfeito |
| GPA-106 | Warp & Warp | 1981 | 8 KB | 0 px | idêntica | ✅ Perfeito |
| GPA-107 | Turpin | 1981? | 8 KB | 0 px | idêntica | ✅ Perfeito |
| GPA-109 | Pachinko - UFO | 1982 | 16 KB | 0 px | idêntica | ✅ Perfeito |
| GPA-110 | Fighting Bug | 1982 | 8 KB | 0 px | idêntica | ✅ Perfeito |
| GPA-111 | Space Panic | 1981? | 8 KB | 0 px | idêntica | ✅ Perfeito |
| GPA-112 | Naughty Boy | 198? | 8 KB | 0 px | idêntica | ✅ Perfeito |
| GPA-114 | Dirty Chameleon | 198? | 8 KB | 0 px | idêntica | ✅ Perfeito |
| GPA-115 | Excite Mahjong | 1982 | 16 KB | 0 px | idêntica | ✅ Perfeito |

**Homebrews (26/26)**

| Jogo | Autor | Ano (MAME) | ROM | Imagem (5 quadros) | RAM no quadro 1000 | Status | Download |
|---|---|---|---|---|---|---|---|
| [Aerial](https://inufuto.web.fc2.com/8bit/aerial/) | Inufuto | 2022 | 12.705 B | 0 px | idêntica | ✅ Perfeito | [aerial.rom](https://inufuto.web.fc2.com/8bit/pv1000/aerial.rom) |
| [AntiAir](https://inufuto.web.fc2.com/8bit/antiair/) | Inufuto | 2024 | 11.747 B | 0 px | idêntica | ✅ Perfeito | [antiair_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/antiair_rom.zip) |
| [Ascend](https://inufuto.web.fc2.com/8bit/ascend/) | Inufuto | 2022 | 15.955 B | 0 px | idêntica | ✅ Perfeito | [ascend.rom](https://inufuto.web.fc2.com/8bit/pv1000/ascend.rom) |
| [Awass](https://inufuto.web.fc2.com/8bit/awass/) | Inufuto | — | 13.781 B | 0 px | idêntica | ✅ Perfeito | [awass_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/awass_rom.zip) |
| [Battlot](https://inufuto.web.fc2.com/8bit/battlot/) | Inufuto | 2022 | 13.234 B | 0 px | idêntica | ✅ Perfeito | [battlot.rom](https://inufuto.web.fc2.com/8bit/pv1000/battlot.rom) |
| [Bootskell](https://inufuto.web.fc2.com/8bit/bootskell/) | Inufuto | 2022 | 13.529 B | 0 px | idêntica | ✅ Perfeito | [bootskell.rom](https://inufuto.web.fc2.com/8bit/pv1000/bootskell.rom) |
| [Cacorm](https://inufuto.web.fc2.com/8bit/cacorm/) | Inufuto | 2022 | 11.485 B | 0 px | idêntica | ✅ Perfeito | [cacorm.rom](https://inufuto.web.fc2.com/8bit/pv1000/cacorm.rom) |
| [Cavit](https://inufuto.web.fc2.com/8bit/cavit/) | Inufuto | 2022 | 14.783 B | 0 px | idêntica | ✅ Perfeito | [cavit.rom](https://inufuto.web.fc2.com/8bit/pv1000/cavit.rom) |
| [Cracky](https://inufuto.web.fc2.com/8bit/cracky/) | Inufuto | 2023 | 10.256 B | 0 px | idêntica | ✅ Perfeito | [cracky_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/cracky_rom.zip) |
| [Demônios](https://github.com/PSGM-Romhacking/pv-1000_demonios) | Péricles S. | — | 32 KB | 0 px | idêntica | ✅ Perfeito | [demonios_en/pt/es.zip](https://github.com/PSGM-Romhacking/pv-1000_demonios/releases/latest) |
| [Guntus](https://inufuto.web.fc2.com/8bit/guntus/) | Inufuto | 2022 | 14.512 B | 0 px | idêntica | ✅ Perfeito | [guntus.rom](https://inufuto.web.fc2.com/8bit/pv1000/guntus.rom) |
| [Hopman](https://inufuto.web.fc2.com/8bit/hopman/) | Inufuto | 2023 | 11.577 B | 0 px | idêntica | ✅ Perfeito | [hopman_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/hopman_rom.zip) |
| [Lift](https://inufuto.web.fc2.com/8bit/lift/) | Inufuto | 2022 | 11.174 B | 0 px | idêntica | ✅ Perfeito | [lift.rom](https://inufuto.web.fc2.com/8bit/pv1000/lift.rom) |
| [Mazy](https://inufuto.web.fc2.com/8bit/mazy/) | Inufuto | 2022 | 10.437 B | 0 px | idêntica | ✅ Perfeito | [mazy.rom](https://inufuto.web.fc2.com/8bit/pv1000/mazy.rom) |
| [Mazy2](https://inufuto.web.fc2.com/8bit/mazy2/) | Inufuto | 2025 | 12.765 B | 0 px | idêntica | ✅ Perfeito | [mazy2_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/mazy2_rom.zip) |
| [Mieyen](https://inufuto.web.fc2.com/8bit/mieyen/) | Inufuto | 2025 | 10.572 B | 0 px | idêntica | ✅ Perfeito | [mieyen_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/mieyen_rom.zip) |
| [Neuras](https://inufuto.web.fc2.com/8bit/neuras/) | Inufuto | 2022 | 12.113 B | 0 px | idêntica | ✅ Perfeito | [neuras.rom](https://inufuto.web.fc2.com/8bit/pv1000/neuras.rom) |
| [Osotos](https://inufuto.web.fc2.com/8bit/osotos/) | Inufuto | 2024 | 14.798 B | 0 px | idêntica | ✅ Perfeito | [osotos_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/osotos_rom.zip) |
| [PV-1000 Control Test](https://github.com/PSGM-Romhacking/pv-1000_control_test_rom) | Péricles S. | — | 8 KB | 0 px | idêntica | ✅ Perfeito | [pv-1000_control_test.zip](https://github.com/PSGM-Romhacking/pv-1000_control_test_rom/releases/latest) |
| [PVong](https://github.com/joffb/pvong) | joe_k | — | 8 KB | 0 px | idêntica | ✅ Perfeito | [pvong.bin](https://github.com/joffb/pvong/releases/download/0.1/pvong.bin) |
| [Ruptus](https://inufuto.web.fc2.com/8bit/ruptus/) | Inufuto | 2022 | 15.768 B | 0 px | 2 B em 0xBFEE (pilha) | ✅ Perfeito | [ruptus.rom](https://inufuto.web.fc2.com/8bit/pv1000/ruptus.rom) |
| [Simon](https://forums.nesdev.org/viewtopic.php?p=303835#p303835) | KashtPR | — | 16 KB | 0 px | 1 B em 0xBFE6 (pilha) | ✅ Perfeito | [simon.zip](https://forums.nesdev.org/viewtopic.php?p=303835#p303835) |
| [Svellas](https://inufuto.web.fc2.com/8bit/svellas/) | Inufuto | — | 11.342 B | 0 px | idêntica | ✅ Perfeito | [svellas_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/svellas_rom.zip) |
| [SwordWork](https://inufuto.web.fc2.com/8bit/sword/) | Inufuto | 2026 | 12.735 B | 0 px | idêntica | ✅ Perfeito | [sword_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/sword_rom.zip) |
| [Uchotos](https://inufuto.web.fc2.com/8bit/uchotos/) | Inufuto | — | 10.550 B | 0 px | 1 B em 0xBFE6 (pilha) | ✅ Perfeito | [uchotos_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/uchotos_rom.zip) |
| [Yewdow](https://inufuto.web.fc2.com/8bit/yewdow/) | Inufuto | 2023 | 13.671 B | 0 px | 2 B em 0xBFEA (pilha) | ✅ Perfeito | [yewdow_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/yewdow_rom.zip) |

### Origem dos homebrews

A maioria é do **Inufuto**, desenvolvedor japonês que publica jogos novos para dezenas de
máquinas de 8 bits, com ROMs gratuitas no site do Inufuto [I1]. Os mais novos têm
o código no GitHub, em <https://github.com/inufuto/Cate_examples>, escritos em **Cate**,
a linguagem e compilador do próprio Inufuto (MIT, <https://github.com/inufuto/Cate>); os outros têm o
código num zip no site. Não há página no itch.io. As datas são as da versão de PV-1000 na
tabela de cada página; para os outros homebrews, a data de lançamento.

| Jogo | Versão PV-1000 | Código-fonte |
|---|---|---|
| Aerial | 2021-11-01 | não publicado para PV-1000 (versões de outras máquinas na [página](https://inufuto.web.fc2.com/8bit/aerial/)) |
| AntiAir | 2024-03-29 | [zip](https://inufuto.web.fc2.com/8bit/pv1000/antiair_src.zip) |
| Ascend | 2022-04-12 | [zip](https://inufuto.web.fc2.com/8bit/pv1000/ascend_src.zip) |
| Awass | 2026-01-11 | só para outras máquinas, no [Cate_examples](https://github.com/inufuto/Cate_examples) (a pasta de PV-1000 que o site indica não existe) |
| Battlot | 2022-02-09 | não publicado para PV-1000 (versões de outras máquinas na [página](https://inufuto.web.fc2.com/8bit/battlot/)) |
| Bootskell | 2022-02-09 | não publicado para PV-1000 (versões de outras máquinas na [página](https://inufuto.web.fc2.com/8bit/bootskell/)) |
| Cacorm | 2022-07-05 | [zip](https://inufuto.web.fc2.com/8bit/pv1000/cacorm_src.zip) |
| Cavit | 2022-02-09 | não publicado para PV-1000 (versões de outras máquinas na [página](https://inufuto.web.fc2.com/8bit/cavit/)) |
| Cracky | 2023-03-25 | [zip](https://inufuto.web.fc2.com/8bit/pv1000/cracky_src.zip) |
| Demônios | 2026-10-20 | [GitHub](https://github.com/PSGM-Romhacking/pv-1000_demonios) |
| Guntus | 2022-10-29 | [zip](https://inufuto.web.fc2.com/8bit/pv1000/guntus_src.zip) |
| Hopman | 2023-08-08 | [zip](https://inufuto.web.fc2.com/8bit/pv1000/hopman_src.zip) |
| Lift | 2021-12-30 | não publicado para PV-1000 (versões de outras máquinas na [página](https://inufuto.web.fc2.com/8bit/lift/)) |
| Mazy | 2021-11-08 | não publicado para PV-1000 (versões de outras máquinas na [página](https://inufuto.web.fc2.com/8bit/mazy/)) |
| Mazy2 | 2025-05-12 | [GitHub](https://github.com/inufuto/Cate_examples/tree/main/pv1000/mazy2) |
| Mieyen | 2025-01-05 | [GitHub](https://github.com/inufuto/Cate_examples/tree/main/pv1000/mieyen) |
| Neuras | 2022-02-09 | não publicado para PV-1000 (versões de outras máquinas na [página](https://inufuto.web.fc2.com/8bit/neuras/)) |
| Osotos | 2024-08-17 | [zip](https://inufuto.web.fc2.com/8bit/pv1000/osotos_src.zip) |
| PV-1000 Control Test | 2026-10-20 | [GitHub](https://github.com/PSGM-Romhacking/pv-1000_control_test_rom) |
| PVong | 2025-07-07 | [GitHub](https://github.com/joffb/pvong) |
| Ruptus | 2021-11-07 | não publicado para PV-1000 (versões de outras máquinas na [página](https://inufuto.web.fc2.com/8bit/ruptus/)) |
| Simon | 2025-08-22 | não publicado (o anexo só tem a ROM) |
| Svellas | 2025-08-16 | só para outras máquinas, no [Cate_examples](https://github.com/inufuto/Cate_examples) (a pasta de PV-1000 que o site indica não existe) |
| SwordWork | 2026-04-25 | [GitHub](https://github.com/inufuto/Cate_examples/tree/main/pv1000/sword) |
| Uchotos | 2026-08-21 | [GitHub](https://github.com/inufuto/Cate_examples/tree/main/pv1000/uchotos) |
| Yewdow | 2023-11-19 | [zip](https://inufuto.web.fc2.com/8bit/pv1000/yewdow_src.zip) |

Na versão de PV-1000, as páginas indicam "mover: controle L" (o do jogador 1) e "insira o
cartucho e ligue o console".

**Outros homebrews:**
- **[PVong](https://github.com/joffb/pvong)**, de joe_k: Pong para 2 jogadores, com a bola movida pixel a pixel por
  padrões em RAM (licença MIT). Usa o **[pvbanjo](https://github.com/joffb/pvbanjo)**, um driver de som do mesmo autor
  que toca músicas compostas no Furnace.
- **Simon**, de KashtPR: o jogo da memória de 1978, escrito em C com o z88dk e testado no
  MAME ([anexo no tópico](https://forums.nesdev.org/viewtopic.php?p=303835#p303835)).
- **[Demônios](https://github.com/PSGM-Romhacking/pv-1000_demonios)**, de Péricles S. (2026): uma aventura em texto
  ilustrada, baseada no conto de 1893 do escritor brasileiro Aluísio Azevedo, em inglês,
  português e espanhol.
- **[PV-1000 Control Test](https://github.com/PSGM-Romhacking/pv-1000_control_test_rom)**, de Péricles S. (2026): uma
  ROM de teste que mostra cada botão dos dois controles enquanto ele é segurado.

O PVong e o Simon foram achados no fórum do NESdev em 2025 [N7]. Os quatro rodam no Mil
Capivaras, nos dois modelos, e estão no banco de jogos (nenhum está na lista do MAME).

**Fora da lista:**
- **GPA-108** e **GPA-113** nunca foram lançados.
- **Impetus**, do Inufuto, não tem versão para PV-1000.
- Hacks e traduções não foram testados. A identificação os trata como jogos
  desconhecidos (sem ficha), mas eles rodam se forem ROMs comuns de PV-1000.

---

## 11. Core libretro (RetroArch)

`libretro` gera o **`mil_capivaras_libretro.so`**: o mesmo `pv1000-core`
atrás das ~25 funções `retro_*` da API C do libretro.

- **Instalar:** `make install-libretro` copia o `.so` e o `.info` para a pasta de cores
  do RetroArch, lida do `retroarch.cfg` (Flatpak ou nativo).
- **Testado** no RetroArch Flatpak 1.22.2: 224×224, 59,54 FPS, áudio a 48 kHz. O quadro
  600 do Dig Dug é **idêntico byte a byte** ao do Mil Capivaras.
- **O RetroArch traz de graça:** shaders, save states, rewind, run-ahead, netplay e
  trapaças (pela RAM exposta em `RETRO_MEMORY_SYSTEM_RAM`). As conquistas dependem de o
  PV-1000 ser aceito no RetroAchievements.
- **Segurança:** nenhuma função deixa um *panic* atravessar para o C.
- Guia completo em [`docs/libretro.pt-BR.md`](docs/libretro.pt-BR.md).

### RetroAchievements

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
- um user agent próprio (`MilCapivaras/0.9.0 (Linux)`);
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

---

## 12. Lições aprendidas

### Abordagem
- **Um core MiSTer não é um programa, é hardware descrito em Verilog.** Para rodar no
  PC, ou se simula o HDL (Verilator) ou se reescreve como emulador usando o HDL como
  especificação.
- **Valeu a pena simular um core para ter uma referência exata:** o MiSTer rodava os 13
  jogos e permitiu comparar byte a byte. A marca de "não suportados" do MAME, que pesou
  na escolha, depois se mostrou desatualizada (no MAME 0.289 os 13 rodam).
- **Modos sem janela (`--shot`, `--press-start`, `--dump-ram`, `--bench`)** foram
  essenciais para medir sem jogar manualmente.
- **Antes de caçar um bug, confira a referência.** Três das divergências eram da própria
  referência (R desligado, fim de quadro no VSync, fase dos relógios).

### Precisão
- **Poucos ciclos importam.** O Excite Mahjong conta voltas de um laço até a próxima
  interrupção para sortear peças; `POP IX` com 10 ciclos em vez de 14 bastou para mudar
  o jogo.
- **Um crate de CPU que passa no ZEXALL pode estar errado nos ciclos:** o ZEXALL confere
  resultados e flags, não tempo.
- **O registrador R é usado como gerador aleatório** (Super Cobra, Space Panic, Excite
  Mahjong). Uma CPU que não o incrementa muda o jogo.
- **Escritas no meio da linha** aparecem já no quadro 2 em quatro jogos: o chip lê a RAM
  pixel a pixel.
- **Medir em vez de supor:** o momento da escrita do `LDIR` foi achado variando o
  parâmetro até a diferença zerar, e conferido com a temporização documentada do Z80.

### Verilator
- **O Verilator não lê VHDL:** o T80 foi trocado pelo TV80, o mesmo T80 em Verilog.
- **Atribuição bloqueante (`=`) lida em outro bloco `always` gera corrida na
  simulação.** Trocar por `<=` reproduz o hardware sintetizado.
- **O custo é o número de `eval()`:** eliminar o relógio de 71 MHz e avaliar só nas
  bordas dos relógios usados levou de 30% para 200% do tempo real.
- **Mover a pasta quebra o `obj_dir`:** os `.d` guardam caminhos absolutos.

### Interface (Linux)
- **No Wayland, o KWin ignora pedidos de tamanho** de janela maximizada ou *tiled*.
  Contorno: travar mínimo = máximo por ~20 quadros, e nunca na abertura (o KWin
  desabilitaria o maximizar).
- **Não use o vsync para cadenciar a emulação:** num monitor de 144 Hz o jogo correria
  2,4× mais rápido.
- **`/dev/input/jsX` não é só controle:** remapeadores de mouse criam dispositivos
  virtuais. Filtrar pelos códigos dos botões.
- **Gatilhos analógicos repousam em −32767:** "apertado" se mede a partir do valor de
  repouso.
- **Redesenhar a cada quadro parado gasta CPU:** a biblioteca caiu de 25% para 6% de um
  núcleo redesenhando só quando algo muda. Enviar o título da janela a cada quadro
  também força redesenho.

### Gravação
- **Taxa como fração:** passando 59,5382, o `ffmpeg` arredondou para 59,94 fps;
  `71590909/1202436` dá a taxa exata.
- **GIF:** atrasos menores que 2 cs viram 10 cs nos navegadores; por isso ~30 fps.

### Modelo de hardware
- **Conferir as contas escritas com o código.** O README dizia 59,527 Hz para o quadro do
  MiSTer desde o início; a fração no código sempre foi a certa (71.590.909 / 1.202.436 =
  59,538 Hz). O erro só apareceu quando um teste automático comparou os dois números.
- **ROMs de teste com fotos do aparelho valem mais que tabelas.** A falta da barra do
  `irqtiming` mostrou, sem ambiguidade, que o problema era o tamanho do apagamento
  horizontal, coisa que nenhuma descrição de porta revelaria.
- **Uma mudança grande num modelo separado preserva a validação.** A temporização do
  aparelho mudou a velocidade dos jogos; como ela vive só no modelo de hardware, o modelo
  MiSTer continua batendo byte a byte com o core, e a regressão prova isso a cada mudança.
- **O fórum tem o que a wiki não tem.** Detalhes como os 5 ciclos a mais antes do BUSACK,
  o atraso de 55 ciclos da interrupção e a peculiaridade da porta FE só estavam nas
  conversas do tópico [N7].
- **Medições de terceiros também validam.** Sem ter o console, os programas de teste do
  Par, com os números que ele anotou em dois aparelhos, permitiram conferir o modelo de
  hardware ciclo a ciclo. E mostraram que a aproximação do roubo de barramento já era
  suficiente: o item que parecia o maior da lista de precisão não precisava de mudança.
- **Comparar no mesmo instante lógico.** Desde a validação inicial, 1 ou 2 bytes diferentes foram
  atribuídos a uma interrupção fora de lugar. O rastreio mostrou interrupções e instruções
  idênticas: a "foto" da RAM cortava uma instrução ao meio de um lado e não do outro.
- **Uma hipótese que parece certa precisa de teste.** O "Tutankham parado no título" do MAME
  era um *game over*; com um roteiro de entrada mais simples, a dúvida se desfez.
- **Simular o circuito, não o resultado.** O atraso da matriz foi modelado como o
  capacitor que ele é (cada entrada lembra quando deixou de ser acionada), e não como uma
  regra por jogo; o teste `joystick` passou sem ajuste.

---

## 13. Referências

### O console
- [W1] Wikipédia (inglês), *PV-1000*: <https://en.wikipedia.org/wiki/PV-1000>
- [W2] Wikipédia (japonês), *PV-1000*: <https://ja.wikipedia.org/wiki/PV-1000>
  (cita *Core Mook* nº 682, 『電子ゲーム なつかしブック』, p. 63)
- old-computers.com, *Casio PV-1000* (arquivado): <https://web.archive.org/web/20151225064754/http://www.old-computers.com/museum/computer.asp?st=2&c=849>
- Centre for Computing History, *Casio PV-1000*: <https://www.computinghistory.org.uk/det/67122/Casio-PV-1000/>
- Video Game Console Library, *Casio PV-1000* (arquivado): <https://web.archive.org/web/20250821122204/http://www.videogameconsolelibrary.com/pg80-pv1000.htm>
- Plogue, *A Complete Guide to Plogue chipsounds* (o som do PV-1000): <https://s3.amazonaws.com/chipsounds/chipsounds_guide.pdf>
- Internet Archive, digitalizações de caixas e cartuchos: <https://archive.org/search?query=subject:%22Casio+PV+1000%22+AND+mediatype:image>

### Implementações
- [C1] Core MiSTer, Flandango (GPL-2.0): <https://github.com/MiSTer-devel/Casio_PV-1000_MiSTer>
- [F1] Fórum do MiSTer, tópico do core: <https://misterfpga.org/viewtopic.php?t=5147>
- [M1] MAME, driver `casio/pv1000.cpp` (BSD-3-Clause): <https://github.com/mamedev/mame/blob/master/src/mame/casio/pv1000.cpp>
- [M2] MAME, lista de software `hash/pv1000.xml` (CC0): <https://github.com/mamedev/mame/blob/master/hash/pv1000.xml>
- [N1] NESdev Obscure Wiki, *Casio PV-1000* (CC0): <https://obscure.nesdev.org/wiki/Casio_PV-1000>
- [N2] NESdev Obscure Wiki, *Casio PV-1000/ASIC registers*: <https://obscure.nesdev.org/wiki/Casio_PV-1000/ASIC_registers>
- [N3] NESdev Obscure Wiki, *Casio PV-1000/Emulation* (o que cada emulador ainda erra): <https://obscure.nesdev.org/wiki/Casio_PV-1000/Emulation>
- [N4] NESdev Obscure Wiki, *Casio PV-1000/Video*: <https://obscure.nesdev.org/wiki/Casio_PV-1000/Video>
- [N5] NESdev Obscure Wiki, *Casio PV-1000/Controller* (a matriz, a chave que troca os dois botões de ataque e o atraso elétrico da leitura): <https://obscure.nesdev.org/wiki/Casio_PV-1000/Controller>
- [N6] NESdev Obscure Wiki, *Casio PV-1000/Rendering*, */ASIC pinout*, */Cartridge connector*, */Controller port pinout*: <https://obscure.nesdev.org/w/index.php?title=Special:PrefixIndex/Casio_PV-1000/>
- [N7] Fórum do NESdev, *PV-1000 test and characterization ROMs* (lidnariq, 2023–2025): a fonte primária da engenharia reversa, com 7 ROMs de teste e o código delas (`scanwfe`, `toggle`, `wff-v2`, `twinkle`, `joystick`, `irqtiming`, `dither-no-ei`), fotos do console real rodando cada uma e diagramas de temporização do vídeo: <https://forums.nesdev.org/viewtopic.php?t=24671>
- [N8] lidnariq, esquema elétrico do PV-1000 redesenhado no KiCad: <https://github.com/lidnariq/Casio-PV-1000-Schematic>
- [N9] Par (team-europe), anotações sobre o hardware (Google Doc citado em [N7]): <https://docs.google.com/document/d/1l5k50xIb5mwGNqktSkat8LCJfIQACc36e89Lqs8GDXg>

- [T1] Takeda Toshiya, *Common Source Code Project*: o ePV-1000 (2006, GPL-2.0 ou posterior), em `source/src/vm/pv1000/`: <https://takeda-toshiya.my.coocan.jp/common/index.html>
- [O1] Obentou, yughias (GPL-2.0): <https://github.com/yughias/Obentou>
- [V1] fv1000, wel97459 (sem licença): <https://github.com/wel97459/fv1000>

### CPU Z80
- T80, Daniel Wallner (a CPU do core MiSTer): <https://opencores.org/projects/t80>
- TV80, Guy Hutchison (o T80 em Verilog, MIT): <https://github.com/hutch31/tv80>
- iz80, Iván Izaguirre (BSD-3-Clause): <https://github.com/ivanizag/iz80>
- Zilog, *Z80 CPU User Manual* (UM0080): <https://www.zilog.com/docs/z80/um0080.pdf>
- Sean Young, *The Undocumented Z80 Documented*: <http://www.z80.info/zip/z80-documented.pdf>
- Frank Cringle, ZEXALL / ZEXDOC (exercitador de instruções)
- Patrik Rak, z80test: <https://github.com/raxoft/z80test>

### Jogos e dados
- [I1] Inufuto, jogos de 8 bits (ROMs e fontes): <https://inufuto.web.fc2.com/8bit/>
- libretro-thumbnails, *Casio - PV-1000*: <https://github.com/libretro-thumbnails/Casio_-_PV-1000>
- Noto Sans CJK (OFL): <https://github.com/notofonts/noto-cjk>

### Emulação
- [G1] Emulation General Wiki, *Emulation accuracy*: <https://emulation.gametechwiki.com/index.php/Emulation_accuracy>
- Emulation General Wiki, *High and low-level emulation*: <https://emulation.gametechwiki.com/index.php/High_and_low-level_emulation>
- libretro, *Developing cores*: <https://docs.libretro.com/development/cores/developing-cores/>
- libretro, `libretro.h`: <https://github.com/libretro/libretro-common/blob/master/include/libretro.h>
- RetroArch, `cheat_manager.c` (formato `.cht`): <https://github.com/libretro/RetroArch/blob/master/cheat_manager.c>
- Verilator: <https://www.veripool.org/verilator/>

---

## 14. Licenças

| Parte | Licença |
|---|---|
| Mil Capivaras | GPL-3.0 ou posterior (`LICENSE`) |
| [iz80](https://github.com/ivanizag/iz80), de Iván Izaguirre (`vendor/iz80/`, cópia modificada) | BSD-3-Clause |
| Banco de jogos (`assets/games/`), a partir da lista de software do MAME | CC0-1.0 (lista do MAME) |
| Recorte da Noto Sans CJK JP | SIL Open Font License 1.1 |

As ROMs não fazem parte do projeto. Os homebrews do Inufuto são gratuitos no site do Inufuto
[I1], sem licença de redistribuição.
