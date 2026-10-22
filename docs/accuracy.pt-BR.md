# Precisão e validação

[English](accuracy.md) · **Português** · [Español](accuracy.es.md)

Como o Mil Capivaras é comparado com o core do MiSTer e com o aparelho, o resultado, e o que ainda falta.

As referências entre colchetes ([N2], [M1]…) estão em [Referências](references.pt-BR.md).

## Como a comparação é feita

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

## Metodologia (detalhes técnicos)

1. **Referência:** o Verilog do core MiSTer simulado ciclo a ciclo com o Verilator
   (em [`reference/`](../reference/README.pt-BR.md), com o script de comparação), com a CPU TV80, que é o T80 do MiSTer em Verilog.
2. **Mesma entrada:** com `--press-start`, os dois apertam Start nos quadros 120–129 e
   alternam Fire 1 + direita a cada 20 quadros a partir do 200.
3. **Mesma saída:** imagem 224×224 e RAM de 64 KB no quadro N.
4. **Comparação:** pixels e bytes diferentes.
5. **Bisseção:** havendo diferença, acha o primeiro quadro, e daí o rastreio instrução a
   instrução (PC, ciclo e R a cada busca de opcode) acha a primeira instrução divergente.
6. **Ferramentas de diagnóstico.** No núcleo: `Pv1000::log_interrupts` registra cada
   interrupção aceita (linha, posição, PC) e `Pv1000::trace`, cada instrução iniciada (linha,
   posição, PC, A, R). Na simulação do core, as mesmas medidas mais a RAM a cada quadro e um
   "vigia" de endereço; essas ferramentas da simulação não foram guardadas, e `reference/`
   traz só a comparação. Dois cuidados
   aprendidos ao comparar:
   - o TV80 grava o resultado no acumulador já durante a instrução seguinte, então o A lido
     no início dela está atrasado; compare a sequência de PCs, não o A instrução a
     instrução;
   - instruções com prefixo (DD, ED, FD, CB) fazem duas buscas de opcode no TV80 e uma no
     emulador; a interrupção aparece como uma busca a mais no TV80.
7. **Mesma foto.** O emulador roda instruções inteiras; o core escreve no ciclo de máquina.
   Para comparar a RAM no fim de um quadro, a simulação usada na investigação terminava a
   instrução em andamento antes de gravar. A de `reference/` grava na virada do quadro, então
   mostra os restos de pilha descritos abaixo.

## Resultado

- **39 programas** (13 comerciais e 26 homebrews), imagem nos quadros 2, 50, 300, 1000 e
  3000: **0 pixels diferentes em todos**. RAM no quadro 3000: **idêntica nos 39**. No
  quadro 1000, idêntica em 34; nos outros 5, 1 ou 2 bytes de pilha diferem por um artefato da
  comparação, não da emulação (ver "Restos de pilha" abaixo). Detalhes na
  [Compatibilidade](compatibility.pt-BR.md).
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
- **"Restos de pilha" (não é erro de emulação):** no Dig Dug, Ruptus, Uchotos e Yewdow, 1
  ou 2 bytes diferem em alguns quadros. A causa:
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
  - o **Simon** tem a mesma sobra: 1 byte em 0xBFE6, o mesmo endereço
    do Uchotos.

  As ferramentas usadas (registro de interrupções e rastreio de instruções no núcleo,
  `log_interrupts`/`trace`, e as mesmas opções na simulação do core) estão descritas na
  [Metodologia](#metodologia-detalhes-técnicos).
- **Flags não documentados:** os bits 3 e 5 do registrador F depois de `BIT n,(HL)`. O
  `iz80` os tira do endereço, o TV80 zera e o T80 do MiSTer usa o registrador interno
  MEMPTR. Também só aparece como resto na pilha (Amidar, no quadro 1500).

## Classificação

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
  FF, borda e proporção do pixel ([Como as implementações diferem](hardware.pt-BR.md#como-as-implementações-diferem)). Nesses
  pontos, o modelo de hardware segue o aparelho, conferido com as medições do Par (abaixo),
  mas sem uma referência para comparar byte a byte;
- o som é a exceção: segue a documentação do hardware, não o core;
- a referência usa o TV80, não o T80 do MiSTer, com a E/S no relógio da CPU em vez do
  `clk_sys`.

## Validação contra o aparelho: as medições do Par

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
partidas automáticas divergem com o tempo entre os dois. Isso explica também o
Tutankham: com o Start sozinho, o MAME também entra no jogo; o título que aparecia
no quadro 3000 era a volta ao título depois de um *game over* que lá chegou antes.

## Situação

| # | Item | Situação |
|---|---|---|
| 1 | Restos de pilha em 4 jogos | **resolvido: não era erro de emulação.** Interrupções e sequência de instruções idênticas ao core; a diferença vinha de a RAM ser comparada no meio de uma instrução que atravessa a virada do quadro. Com a foto alinhada ao fim da instrução: 0 diferenças em 1.100 quadros no Dig Dug, Uchotos e Yewdow; 2 no Ruptus, de instruções que começam exatamente na virada. De quebra, o modelo MiSTer deixou de espelhar a RAM (o core tem memória plana) |
| 2 | Roubo de barramento por ciclo de máquina | **não precisa mudar**: a aproximação atual (folga de 4 ciclos) bate com 11 medições do aparelho; só as folgas de 4 e 5 passam no teste de divisão, como o lidnariq descreveu |
| 3 | Detalhes do vídeo durante a imagem | pendente (refinamento) |
| 4 | Números do Par | **feito**: 8 cenários de ciclos e 3 divisões de tela iguais ao aparelho; dois testes automáticos (`par_cycle_counts_match_hardware`, `par_split_lines_match_hardware`) |
| 5 | Tutankham × MAME | **resolvido**: o MAME entra no jogo com o Start sozinho; o título do quadro 3000 era a volta depois de um *game over*. O MAME fica ~190–200 ciclos por quadro abaixo do aparelho no teste do Par, e por isso as partidas divergem |
| 6 | Matriz do joystick | parcial: soltar emulado (136 ciclos); subida instantânea; a variação entre consoles não é emulável |
| 7 | IM 0 e IM 2 | **feito**: barramento suposto em 0xFF (IM 0 = `RST 38h`, IM 2 com vetor em I × 256 + 0xFF); teste `interrupt_modes_0_and_2` |
| 8 | Faixa 0xC000–0xFFFF | **feito no modelo de hardware** (escritas perdidas, leituras 0xFF); o modelo MiSTer mantém o comportamento do core, que guarda o que é escrito (o Battlot escreve ali); teste `open_bus_above_c000_on_hardware` |
| 9 | CPU (ZEXALL) | **feito**: ZEXALL 67/67; `z80test` 137/152, com as mesmas 15 falhas do `iz80` original (comportamentos não documentados). Corrigi-las exige conferir antes o TV80 |
| 10 | Filtros passa-alta | **feito**: primeira ordem por canal (50, 72, 103 Hz); teste `tone_frequency_and_highpass` |
| 11 | Fase inicial dos canais parados (XOR) | pendente |
| 12 | Cores de TV | pendente: precisa de fotos ou capturas para calibrar |

Abaixo, o resultado de cada item resolvido e o detalhe dos que estão em aberto.

## Como cada parte foi validada

Cada parte da emulação, com a prova. O modelo MiSTer é validado contra a simulação do core; o modelo de hardware, contra
medições feitas em aparelhos reais por outras pessoas.

### Modelo MiSTer (a referência validada)

| Melhoria | Prova |
|---|---|
| Ciclos do `iz80` corrigidos (prefixos DD/FD, `JR cc`, `JP cc`), registrador R, `LD A,R`/`LD A,I` | rastreio instrução a instrução contra o TV80; RNG do Excite Mahjong igual |
| Interrupção vista 2 ciclos depois; primeira instrução após o reset 2 ciclos mais curta | RAM idêntica além de ~150 quadros |
| Vídeo desenhado pixel a pixel, com as escritas datadas dentro da instrução | 0 pixels diferentes nos 35 programas do levantamento original (13 comerciais + 22 do Inufuto) |
| `HALT` que sai com a interrupção (o `iz80` original travava) | teste `halt_resumes_on_interrupt` |
| Memória plana de 64 KB, sem espelhos, e a faixa 0xC000–0xFFFF guardando o que é escrito, como no core | regressão 152/152 idêntica (o Battlot escreve acima de 0xDC00) |
| Os "restos de pilha" explicados: artefato de comparar a RAM no meio de uma instrução | interrupções e instruções idênticas; com a foto alinhada, 0 diferenças em 1.100 quadros (Dig Dug, Uchotos, Yewdow) e 2 no Ruptus, de instruções que começam exatamente na virada |

Os outros 4 homebrews (PVong, Simon, Demônios, PV-1000 Control Test), com o mesmo método,
também são idênticos à referência: imagem 0 px, RAM igual
no quadro 3000. O Simon tem 1 byte de pilha em 0xBFE6 no quadro 1000, como o Uchotos.

### Som (os dois modelos)

| Melhoria | Prova |
|---|---|
| Gerador uma oitava abaixo do MiSTer: 8.739 / (63 − n) Hz | NESdev [N2] e MAME; bipe da ROM de teste a 380 Hz |
| Volumes −6 / −3 / 0 dB por canal e a porta FB (liga o som, modo XOR) | traçado da placa [N2] |
| Filtros passa-alta de primeira ordem por canal (50, 72, 103 Hz) | teste `tone_frequency_and_highpass` |

### Modelo de hardware (o aparelho)

| Melhoria | Prova |
|---|---|
| Interrupções habilitadas pela porta FC e reconhecidas pela FD; 16 por quadro mais a de pré-render | teste `frame_flag_needs_enabling_on_hardware`; ROM de teste trava sem habilitar, como no MAME |
| Matriz em OU | teste `matrix_rows_or_on_hardware` |
| Atraso elétrico da matriz (136 ciclos para soltar) | teste `matrix_release_is_slow_on_hardware`; ROM `joystick` do lidnariq com seis colunas na linha de 21 ciclos, como ele mediu |
| Portas FE e FF completas (mapa e PCG móveis, 8 bancos, tela desligada) | ROMs `scanwfe` e `wff-v2` do lidnariq batem com as fotos |
| Temporização do aparelho: 288 × 262 a 59,299 Hz, roubo de barramento nas linhas de imagem, interrupção no pixel 276, imagem nos pixels 52–275 | **11 medições do Par em dois PV-1000 iguais** (8 cenários de ciclos, 3 divisões de tela); `irqtiming` e `toggle` do lidnariq batem com as fotos; teste `bus_stealing_on_hardware` |
| Folga do roubo de barramento (4 ciclos) | só 4 e 5 ciclos passam no teste de divisão, como o lidnariq descreveu |
| Imagem TV: borda de 26 linhas e pixel 48:35 | NESdev [N4] |
| Espelhos da RAM em 0x8000–0xBFFF e barramento aberto em 0xC000–0xFFFF | NESdev [N1]; teste `open_bus_above_c000_on_hardware` |

### CPU (os dois modelos)

| Melhoria | Prova |
|---|---|
| IM 0 (como `RST 38h`) e IM 2 (vetor em I × 256 + 0xFF), supondo o barramento em 0xFF | teste `interrupt_modes_0_and_2` (suposição não medida) |
| Conferência com as baterias padrão | ZEXALL 67/67; `z80test` 137/152, as mesmas 15 falhas do `iz80` original |

### Ferramentas que ficaram

- No núcleo: `Pv1000::log_interrupts` e `Pv1000::trace` (desligados por padrão).
- Testes que usam os arquivos de `_referencias/` (fora do git) e são pulados sem eles:
  `par_cycle_counts_match_hardware`, `par_split_lines_match_hardware`.

## Cada item em detalhe

### Modelo MiSTer

#### 1. Restos de pilha em 4 jogos (feito)

Não era erro de emulação. Interrupções e sequência de instruções são idênticas ao core; a
diferença vinha de a RAM ser comparada no meio de uma instrução que atravessa a virada do
quadro. Com a foto alinhada ao fim da instrução: 0 diferenças em 1.100 quadros no Dig Dug,
Uchotos e Yewdow, e 2 no Ruptus, de instruções que começam exatamente na virada.

### Modelo de hardware

#### 2. Roubo de barramento por ciclo de máquina (não precisa mudar)

O emulador aproxima: a instrução que já começou passa inteira se terminar até 4 ciclos
depois do BUSREQ; senão, é cortada no início do BUSREQ. Isso bate com as 11 medições do
aparelho (item 4). Só as folgas de 4 e 5 ciclos passam no teste de divisão, como o lidnariq
descreveu.

#### 3. Detalhes do vídeo durante a imagem

- **Busca antecipada:** o chip busca o tile e as cores 8 pixels antes de desenhá-los. O
  emulador lê no próprio pixel. Só faz diferença se algo mudar no meio da linha de imagem,
  o que quase nunca acontece, porque a CPU está parada.
- **Ligar a imagem no meio de uma linha** desenha de 8 a 16 pixels de lixo no aparelho; o
  emulador não desenha.
- **Trocas de vídeo perto do fim da janela livre:** o Par viu uma linha inteira que pisca
  entre as duas configurações. O emulador é determinístico e escolhe sempre uma delas.

#### 4. Conferir os números medidos pelo Par (feito)

Os programas do Par [N9] foram rodados no modelo de hardware: os 8 cenários de ciclos e as
3 divisões de tela dão os valores que o Par viu no aparelho. Dois testes conferem
(`par_cycle_counts_match_hardware`, `par_split_lines_match_hardware`), pulados sem os
arquivos de `_referencias/`.

#### 5. Tutankham contra o MAME (feito)

O MAME também entra no jogo com o Start sozinho; o título do quadro 3000 era a volta depois
de um *game over*, que lá chega antes. O MAME fica ~190–200 ciclos por quadro abaixo do
aparelho no teste do Par, e por isso as partidas divergem.

#### 6. Matriz do joystick

- A **subida** (0 a 41 ciclos para o botão aparecer) é tratada como instantânea.
- O Par mediu que o tempo necessário **varia entre consoles, controles, TVs ligadas e
  temperatura**. Isso não tem como ser emulado com um valor só; o emulador usa o meio da
  faixa medida (136 ciclos para soltar).

### Os dois modelos

#### 7. Modos de interrupção IM 0 e IM 2 (feito)

No aparelho, o chip não coloca nada no barramento durante o reconhecimento. O emulador
supõe o barramento em 0xFF (resistores de pull-up): o IM 0 executa `RST 38h`, igual ao
IM 1, e o IM 2 usa o vetor em `(I × 256) + 0xFF`. É uma suposição, **não confirmada** em
nenhuma fonte; nenhum dos 35 programas do levantamento original usa esses modos. Teste
`interrupt_modes_0_and_2`.

#### 8. A faixa 0xC000–0xFFFF (feito)

No modelo de hardware, as escritas se perdem e as leituras dão 0xFF, como no aparelho, onde
não há nada ligado ali ([N1]). O modelo MiSTer mantém o comportamento do core, que guarda o
que é escrito (o Battlot escreve acima de 0xDC00). Teste `open_bus_above_c000_on_hardware`.

#### 9. A CPU como um todo (feito)

O ZEXALL passa 67/67. O `z80test` passa 137/152, com as mesmas 15 falhas do `iz80`
original (comportamentos não documentados). Corrigi-las exige conferir antes o TV80.

### Som

#### 10. Filtros passa-alta (feito)

O aparelho tem um passa-alta em cada canal (50, 72 e 103 Hz). Em uso normal eles são
inaudíveis, mas **com o modo XOR ficam audíveis** ([N2]). Emulados como filtro de primeira
ordem por canal; teste `tone_frequency_and_highpass`.

#### 11. Fase inicial dos canais parados

Com o modo XOR, um canal parado fica em nível alto ou baixo ao acaso, o que muda o volume
de outro canal ([N7], lidnariq e Par). O emulador começa sempre na mesma fase. Emular isso
tornaria o som não determinístico; o melhor é documentar e, no máximo, sortear com uma
semente fixa.

### Imagem

#### 12. Cores do aparelho

No aparelho as cores saem **lavadas**, menos saturadas que o RGB puro (vermelho e amarelo
principalmente), porque a cor de TV é montada na placa ([N7]). O emulador usa o RGB puro.
Uma opção de "cores de TV" precisaria de fotos ou capturas de referência para calibrar.

## Ordem sugerida

Os itens 1, 2, 4, 5 e 7–10 estão resolvidos e o item 6 é parcial (ver a situação). O que falta:

| # | Item | Esforço | Ganho |
|---|---|---|---|
| 1 | Cores de TV (12), fase do XOR (11), detalhes do vídeo (3) | variável | refinamento |
| 2 | Matriz do joystick (6), parcial: subida instantânea; a variação entre consoles não é emulável | variável | refinamento |
