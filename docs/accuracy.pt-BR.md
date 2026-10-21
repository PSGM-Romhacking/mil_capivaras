# Precisão: o que falta melhorar

[English](accuracy.md) · **Português** · [Español](accuracy.es.md)

Levantamento de 2026-10-18, conferido no código e nas fontes ([N1]–[N9] do README, o
documento do Par e o tópico do fórum do NESdev). Notas no início do levantamento: **modelo
MiSTer 9,5**, **modelo de hardware 8**.

## Situação (atualizada em 2026-10-18)

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

## Histórico das melhorias de precisão

Tudo o que mudou na precisão em 2026-10-17 e 2026-10-18, em ordem, com a prova de cada
item. O modelo MiSTer é validado contra a simulação do core; o modelo de hardware, contra
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
| Taxa escrita corrigida: 59,538 Hz (o README dizia 59,527) | teste `bus_stealing_on_hardware` compara as duas taxas |

Os outros 4 homebrews (PVong, Simon, Demônios, PV-1000 Control Test) foram comparados
depois com o mesmo método e também ficaram idênticos à referência: imagem 0 px, RAM igual
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

### Comparação com o MAME 0.289

No teste de ciclos do Par (só INT1, imagem ligada), o aparelho dá $57AD / $5702 (versões
ADD / INC), o Mil Capivaras dá **$57AD / $5702** e o MAME, $56F2 / $563A: o MAME fica
~190–200 ciclos por quadro abaixo do aparelho.

### Ferramentas que ficaram

- No núcleo: `Pv1000::log_interrupts` e `Pv1000::trace` (desligados por padrão).
- Testes que usam os arquivos de `_referencias/` (fora do git) e são pulados sem eles:
  `par_cycle_counts_match_hardware`, `par_split_lines_match_hardware`.

## Modelo MiSTer (9,5)

### 1. Restos de pilha em 4 jogos (feito)

Não era erro de emulação. Interrupções e sequência de instruções são idênticas ao core; a
diferença vinha de a RAM ser comparada no meio de uma instrução que atravessa a virada do
quadro. Com a foto alinhada ao fim da instrução: 0 diferenças em 1.100 quadros no Dig Dug,
Uchotos e Yewdow, e 2 no Ruptus, de instruções que começam exatamente na virada.

## Modelo de hardware (8)

### 2. Roubo de barramento por ciclo de máquina (não precisa mudar)

O emulador aproxima: a instrução que já começou passa inteira se terminar até 4 ciclos
depois do BUSREQ; senão, é cortada no início do BUSREQ. Isso bate com as 11 medições do
aparelho (item 4). Só as folgas de 4 e 5 ciclos passam no teste de divisão, como o lidnariq
descreveu.

### 3. Detalhes do vídeo durante a imagem

- **Busca antecipada:** o chip busca o tile e as cores 8 pixels antes de desenhá-los. O
  emulador lê no próprio pixel. Só faz diferença se algo mudar no meio da linha de imagem,
  o que quase nunca acontece, porque a CPU está parada.
- **Ligar a imagem no meio de uma linha** desenha de 8 a 16 pixels de lixo no aparelho; o
  emulador não desenha.
- **Trocas de vídeo perto do fim da janela livre:** o Par viu uma linha inteira que pisca
  entre as duas configurações. O emulador é determinístico e escolhe sempre uma delas.

### 4. Conferir os números medidos pelo Par (feito)

Os programas do Par [N9] foram rodados no modelo de hardware: os 8 cenários de ciclos e as
3 divisões de tela dão os valores que o Par viu no aparelho. Dois testes conferem
(`par_cycle_counts_match_hardware`, `par_split_lines_match_hardware`), pulados sem os
arquivos de `_referencias/`.

### 5. Tutankham contra o MAME (feito)

O MAME também entra no jogo com o Start sozinho; o título do quadro 3000 era a volta depois
de um *game over*, que lá chega antes. O MAME fica ~190–200 ciclos por quadro abaixo do
aparelho no teste do Par, e por isso as partidas divergem.

### 6. Matriz do joystick

- A **subida** (0 a 41 ciclos para o botão aparecer) é tratada como instantânea.
- O Par mediu que o tempo necessário **varia entre consoles, controles, TVs ligadas e
  temperatura**. Isso não tem como ser emulado com um valor só; o emulador usa o meio da
  faixa medida (136 ciclos para soltar).

## Os dois modelos

### 7. Modos de interrupção IM 0 e IM 2 (feito)

No aparelho, o chip não coloca nada no barramento durante o reconhecimento. O emulador
supõe o barramento em 0xFF (resistores de pull-up): o IM 0 executa `RST 38h`, igual ao
IM 1, e o IM 2 usa o vetor em `(I × 256) + 0xFF`. É uma suposição, **não confirmada** em
nenhuma fonte; nenhum dos 35 programas do levantamento original usa esses modos. Teste
`interrupt_modes_0_and_2`.

### 8. A faixa 0xC000–0xFFFF (feito)

No modelo de hardware, as escritas se perdem e as leituras dão 0xFF, como no aparelho, onde
não há nada ligado ali ([N1]). O modelo MiSTer mantém o comportamento do core, que guarda o
que é escrito (o Battlot escreve acima de 0xDC00). Teste `open_bus_above_c000_on_hardware`.

### 9. A CPU como um todo (feito)

O ZEXALL passa 67/67. O `z80test` passa 137/152, com as mesmas 15 falhas do `iz80`
original (comportamentos não documentados). Corrigi-las exige conferir antes o TV80.

## Som

### 10. Filtros passa-alta (feito)

O aparelho tem um passa-alta em cada canal (50, 72 e 103 Hz). Em uso normal eles são
inaudíveis, mas **com o modo XOR ficam audíveis** ([N2]). Emulados como filtro de primeira
ordem por canal; teste `tone_frequency_and_highpass`.

### 11. Fase inicial dos canais parados

Com o modo XOR, um canal parado fica em nível alto ou baixo ao acaso, o que muda o volume
de outro canal ([N7], lidnariq e Par). O emulador começa sempre na mesma fase. Emular isso
tornaria o som não determinístico; o melhor é documentar e, no máximo, sortear com uma
semente fixa.

## Imagem

### 12. Cores do aparelho

No aparelho as cores saem **lavadas**, menos saturadas que o RGB puro (vermelho e amarelo
principalmente), porque a cor de TV é montada na placa ([N7]). O emulador usa o RGB puro.
Uma opção de "cores de TV" precisaria de fotos ou capturas de referência para calibrar.

## Ordem sugerida

Os itens 1, 2, 4, 5 e 7–10 estão resolvidos e o item 6 é parcial (ver a situação). O que falta:

| # | Item | Esforço | Ganho |
|---|---|---|---|
| 1 | Cores de TV (12), fase do XOR (11), detalhes do vídeo (3) | variável | refinamento |
| 2 | Matriz do joystick (6), parcial: subida instantânea; a variação entre consoles não é emulável | variável | refinamento |
