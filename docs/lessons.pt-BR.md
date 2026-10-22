# Lições aprendidas

[English](lessons.md) · **Português** · [Español](lessons.es.md)

As referências entre colchetes ([N2], [M1]…) estão em [Referências](references.pt-BR.md).

## Abordagem
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

## Precisão
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

## Verilator
- **O Verilator não lê VHDL:** o T80 foi trocado pelo TV80, o mesmo T80 em Verilog.
- **Atribuição bloqueante (`=`) lida em outro bloco `always` gera corrida na
  simulação.** Trocar por `<=` reproduz o hardware sintetizado.
- **O custo é o número de `eval()`:** eliminar o relógio de 71 MHz e avaliar só nas
  bordas dos relógios usados levou de 30% para 200% do tempo real.
- **Mover a pasta quebra o `obj_dir`:** os `.d` guardam caminhos absolutos.

## Interface (Linux)
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

## Gravação
- **Taxa como fração:** passando 59,5382, o `ffmpeg` arredondou para 59,94 fps;
  `71590909/1202436` dá a taxa exata.
- **GIF:** atrasos menores que 2 cs viram 10 cs nos navegadores; por isso ~30 fps.

## Modelo de hardware
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
