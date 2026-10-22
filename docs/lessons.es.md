# Lecciones aprendidas

[English](lessons.md) · [Português](lessons.pt-BR.md) · **Español**

Las referencias entre corchetes ([N2], [M1]…) están en [Referencias](references.es.md).

## Enfoque
- **Un core de MiSTer no es un programa, es hardware descrito en Verilog.** Para ejecutarlo en el
  PC, o se simula el HDL (Verilator) o se reescribe como emulador usando el HDL como
  especificación.
- **Valió la pena simular un core para tener una referencia exacta:** MiSTer ejecutaba los 13
  juegos y permitió comparar byte a byte. La marca de "no soportados" de MAME, que pesó
  en la elección, resultó después estar desactualizada (en MAME 0.289 funcionan los 13).
- **Los modos sin ventana (`--shot`, `--press-start`, `--dump-ram`, `--bench`)** fueron
  esenciales para medir sin jugar manualmente.
- **Antes de cazar un bug, revisar la referencia.** Tres de las divergencias eran de la propia
  referencia (R desactivado, fin de cuadro en el VSync, fase de los relojes).

## Precisión
- **Pocos ciclos importan.** Excite Mahjong cuenta vueltas de un bucle hasta la siguiente
  interrupción para sortear fichas; `POP IX` con 10 ciclos en lugar de 14 bastó para cambiar
  el juego.
- **Un crate de CPU que pasa ZEXALL puede estar equivocado en los ciclos:** ZEXALL verifica
  resultados y flags, no tiempos.
- **El registro R se usa como generador aleatorio** (Super Cobra, Space Panic, Excite
  Mahjong). Una CPU que no lo incrementa cambia el juego.
- **Las escrituras en mitad de la línea** aparecen ya en el cuadro 2 en cuatro juegos: el chip lee la RAM
  píxel a píxel.
- **Medir en lugar de suponer:** el momento de la escritura de `LDIR` se encontró variando el
  parámetro hasta que la diferencia llegó a cero, y se verificó con la temporización documentada del Z80.

## Verilator
- **Verilator no lee VHDL:** el T80 se sustituyó por TV80, el mismo T80 en Verilog.
- **Una asignación bloqueante (`=`) leída en otro bloque `always` genera una carrera en la
  simulación.** Cambiarla por `<=` reproduce el hardware sintetizado.
- **El costo es el número de `eval()`:** eliminar el reloj de 71 MHz y evaluar solo en los
  flancos de los relojes usados llevó del 30% al 200% del tiempo real.
- **Mover la carpeta rompe el `obj_dir`:** los `.d` guardan rutas absolutas.

## Interfaz (Linux)
- **En Wayland, KWin ignora las peticiones de tamaño** de una ventana maximizada o *tiled*.
  Solución alternativa: fijar mínimo = máximo durante ~20 cuadros, y nunca al abrir (KWin
  desactivaría el maximizado).
- **No usar el vsync para marcar el ritmo de la emulación:** en un monitor de 144 Hz el juego iría
  2,4× más rápido.
- **`/dev/input/jsX` no es solo para mandos:** los reasignadores de ratón crean dispositivos
  virtuales. Hay que filtrar por los códigos de los botones.
- **Los gatillos analógicos reposan en −32767:** "pulsado" se mide a partir del valor de
  reposo.
- **Redibujar en cada cuadro estando detenido gasta CPU:** la biblioteca bajó del 25% al 6% de un
  núcleo redibujando solo cuando algo cambia. Enviar el título de la ventana en cada cuadro
  también fuerza el redibujado.

## Grabación
- **Tasa como fracción:** al pasar 59,5382, `ffmpeg` redondeó a 59,94 fps;
  `71590909/1202436` da la tasa exacta.
- **GIF:** los retardos menores de 2 cs se convierten en 10 cs en los navegadores; por eso ~30 fps.

## Modelo de hardware
- **Cotejar las cuentas escritas con el código.** El README decía 59,527 Hz para el cuadro
  de MiSTer desde el principio; la fracción en el código siempre fue la correcta
  (71.590.909 / 1.202.436 = 59,538 Hz). El error solo apareció cuando una prueba automática
  comparó los dos números.
- **Las ROMs de prueba con fotos de la consola valen más que las tablas.** La falta de la
  barra de `irqtiming` mostró, sin ambigüedad, que el problema era el largo del apagado
  horizontal, algo que ninguna descripción de puerto revelaría.
- **Un cambio grande en un modelo aparte preserva la validación.** La temporización de la
  consola cambió la velocidad de los juegos; como vive solo en el modelo de hardware, el
  modelo MiSTer sigue coincidiendo byte a byte con el core, y la regresión lo prueba en cada
  cambio.
- **El foro tiene lo que la wiki no tiene.** Detalles como los 5 ciclos de más antes del
  BUSACK, la latencia de 55 ciclos de la interrupción y la peculiaridad del puerto FE solo
  estaban en las conversaciones del hilo [N7].
- **Las mediciones de terceros también validan.** Sin tener la consola, los programas de
  prueba de Par, con los números que anotó en dos consolas, permitieron verificar el modelo
  de hardware ciclo a ciclo. Y mostraron que la aproximación del robo de bus ya bastaba: el
  punto que parecía el mayor de la lista de precisión no necesitaba cambios.
- **Comparar en el mismo instante lógico.** Desde la validación inicial, 1 o 2 bytes distintos se
  atribuyeron a una interrupción fuera de lugar. El rastreo mostró interrupciones e
  instrucciones idénticas: la "foto" de la RAM cortaba una instrucción a la mitad de un lado
  y no del otro.
- **Una hipótesis que parece correcta necesita prueba.** El "Tutankham detenido en el
  título" de MAME era un *game over*; con un guion de entrada más simple, la duda se
  resolvió.
- **Simular el circuito, no el resultado.** El retardo de la matriz se modeló como el
  condensador que es (cada entrada recuerda cuándo dejó de ser activada), y no como una
  regla por juego; la prueba `joystick` pasó sin ajustes.
