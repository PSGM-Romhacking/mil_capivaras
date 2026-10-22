# O Casio PV-1000: hardware e implementações

[English](hardware.md) · **Português** · [Español](hardware.es.md)

O console, o hardware em detalhe e como cada implementação (MiSTer, MAME, ePV-1000, Obentou e Mil Capivaras) o reproduz.

As referências entre colchetes ([N2], [M1]…) estão em [Referências](references.pt-BR.md).

## O Casio PV-1000

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
para 12 dos jogos de PV-1000, o código-fonte. São **22 títulos para PV-1000** (lista completa em [Compatibilidade](compatibility.pt-BR.md)).
O MAME cataloga 19 deles [M2]. Outros autores também fizeram homebrews de PV-1000: PVong
(joe_k), Simon (KashtPR), Demônios e a PV-1000 Control Test (Péricles S.).

## Hardware em detalhe

Esta seção descreve o console **como o core do MiSTer o implementa**, que é a
especificação que o Mil Capivaras segue. Ela foi conferida contra o MAME e o Obentou
([Como as implementações diferem](#como-as-implementações-diferem)). Não existe documentação oficial
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
  isso (ver [O porquê de cada decisão](architecture.pt-BR.md#o-porquê-de-cada-decisão)).

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
Mil Capivaras emula esse roubo ([O quadro no modelo de hardware real](architecture.pt-BR.md#o-quadro-no-modelo-de-hardware-real)).

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
| FE | guardado, sem efeito no core nem no MAME; no aparelho, endereço do mapa e do PCG ([Como as implementações diferem](#como-as-implementações-diferem)) | o último valor escrito |
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
opção **Imagem: TV** usa a do aparelho, 48:35 ([Os dois modelos](#os-dois-modelos)).

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

## Como as implementações diferem

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
aparelho real (a borda, por exemplo) [F1]. A lista do MAME marca 6 cartuchos como não
suportados (Dig Dug, Pachinko-UFO, Fighting Bug, Space Panic, Naughty Boy, Dirty Chameleon)
[M2]. **A marca está desatualizada:** no MAME 0.289, os 6 chegam à partida e pontuam; os
consertos de interrupção, leitura de portas e roubo de barramento de 2024 os fizeram rodar,
mas a lista não foi corrigida. Entre os outros emuladores, o MAME hoje é o mais fiel
ao hardware ([abaixo](#o-core-do-mister-e-o-hardware-real)); o modelo de hardware do Mil
Capivaras chega mais perto nas medições feitas no aparelho
([as medições do Par](accuracy.pt-BR.md#validação-contra-o-aparelho-as-medições-do-par)).
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

O modelo de hardware do Mil Capivaras cobre os 14 pontos da lista.

### As ROMs de teste do lidnariq no Mil Capivaras

As 7 ROMs de teste do tópico [N7] rodam no Mil Capivaras. Comparando com as fotos do
aparelho publicadas lá:

| Teste | No aparelho | Mil Capivaras | Por quê |
|---|---|---|---|
| `scanwfe` (porta FE) | dígitos grandes em preto sobre tiles "0F", com faixas brancas | a mesma estrutura no modelo de hardware | a porta FE está implementada |
| `wff-v2` (porta FF, bits altos) | a sequência 0R 00 2R 22 4R… | mostra "4R", um dos passos esperados | os 8 bancos e o "forçar" estão implementados |
| `dither-no-ei` | barras de pontilhado | as mesmas barras | só depende dos padrões |
| `irqtiming` (FC = 1) | uma barra branca à esquerda da borda de cima, a cada quadro | **a mesma barra, no mesmo lugar** | a barra começa 55 ciclos depois da interrupção, o que no aparelho cai no começo da linha seguinte. Na temporização do MiSTer, o apagamento horizontal é mais longo (26,5 µs contra 14,3 µs) e a barra sumia dentro dele |
| `toggle` (liga e desliga a imagem) | o fractal com faixas horizontais em cima e uma escada diagonal embaixo, à esquerda | **o fractal, as faixas e a escada**; a inclinação da escada ainda difere um pouco | depende do tamanho exato da linha e do roubo de barramento |
| `joystick` (com um botão apertado) | as linhas seguintes ainda mostram "1" por um tempo; na de 21 ciclos por coluna, seis | **seis colunas** na linha de 21 ciclos (~126 ciclos), como o lidnariq mediu | o atraso elétrico da matriz |

**Conclusão:** os 6 testes comparáveis batem com as fotos. Os das portas FE, FF e FC
dependem só das portas; o `irqtiming`, o `toggle` e o `joystick` dependem da temporização do
aparelho: o tamanho da linha, o roubo de barramento e o atraso da matriz. O `twinkle` toca música e troca a borda ao longo do tempo, então uma captura só não
basta para compará-lo.

### Os dois modelos

A opção **Emulação › Modelo** (e, no RetroArch, *Opções do núcleo › Model*) escolhe:

| Modelo | O que segue | Para quê |
|---|---|---|
| **Core do MiSTer (validado)**, o padrão | o core do MiSTer, byte a byte ([Precisão e validação](accuracy.pt-BR.md)) | jogar; é o modelo comprovado nos 39 programas |
| **Hardware real (NESdev / MAME)** | o aparelho, onde o NESdev e o MAME 0.289 documentam: **quadro de 288 × 262 a 59,299 Hz**, **CPU parada enquanto a imagem é desenhada**, interrupções no pixel 276, só quando habilitadas na porta FC e reconhecidas pela FD; matriz em OU e com o **atraso elétrico**; portas FE e FF completas (endereço do mapa e do PCG, 8 bancos de padrões, tela desligada); portas sem dono lidas como `0xFF` | desenvolver e testar ROMs novas: o que trava ou fica lento no console trava ou fica lento aqui também |

**Validação do modelo de hardware:**
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
  ([as medições do Par](accuracy.pt-BR.md#validação-contra-o-aparelho-as-medições-do-par));
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

**Os dois modelos são independentes:** a temporização do aparelho só vale no modelo de
hardware. No modelo MiSTer, a CPU nunca para e o quadro é de 59,538 Hz, e a comparação byte
a byte com o core continua valendo: a regressão de imagem e RAM (quadros 300 e 3000) confere
isso a cada mudança.

**A taxa de quadros acompanha o modelo:** 59,538 Hz no modelo MiSTer e 59,299 Hz no de
hardware. O ritmo da janela, o som, o contador de velocidade, as gravações em MP4 e, no
RetroArch, a taxa informada ao frontend (com `SET_SYSTEM_AV_INFO` ao trocar de modelo)
seguem o modelo escolhido.
