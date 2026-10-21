# Precisión: lo que falta mejorar

[English](accuracy.md) · [Português](accuracy.pt-BR.md) · **Español**

Lo que ya está validado y lo que todavía falta, verificado en el código y en las fuentes
([N1]–[N9] del README, el documento de Par y el hilo del foro de NESdev).

## Situación

| # | Ítem | Situación |
|---|---|---|
| 1 | Restos de pila en 4 juegos | **resuelto: no era un error de emulación.** Interrupciones y secuencia de instrucciones idénticas al core; la diferencia venía de comparar la RAM en medio de una instrucción que atraviesa el cambio de cuadro. Con la captura alineada al final de la instrucción: 0 diferencias en 1.100 cuadros en Dig Dug, Uchotos y Yewdow; 2 en Ruptus, de instrucciones que empiezan justo en el cambio. De paso, el modelo MiSTer dejó de reflejar la RAM (el core tiene memoria plana) |
| 2 | Robo de bus por ciclo de máquina | **no hace falta cambiarlo**: la aproximación actual (holgura de 4 ciclos) coincide con 11 mediciones del aparato; solo las holguras de 4 y 5 pasan la prueba de división, como describió lidnariq |
| 3 | Detalles del video durante la imagen | pendiente (refinamiento) |
| 4 | Números de Par | **hecho**: 8 escenarios de ciclos y 3 divisiones de pantalla iguales al aparato; dos pruebas automáticas (`par_cycle_counts_match_hardware`, `par_split_lines_match_hardware`) |
| 5 | Tutankham × MAME | **resuelto**: MAME entra al juego solo con Start; el título del cuadro 3000 era la vuelta después de un *game over*. MAME queda ~190–200 ciclos por cuadro por debajo del aparato en la prueba de Par, y por eso las partidas divergen |
| 6 | Matriz del joystick | parcial: soltar emulado (136 ciclos); subida instantánea; la variación entre consolas no se puede emular |
| 7 | IM 0 e IM 2 | **hecho**: bus supuesto en 0xFF (IM 0 = `RST 38h`, IM 2 con el vector en I × 256 + 0xFF); prueba `interrupt_modes_0_and_2` |
| 8 | Rango 0xC000–0xFFFF | **hecho en el modelo de hardware** (escrituras perdidas, lecturas 0xFF); el modelo MiSTer mantiene el comportamiento del core, que guarda lo que se escribe (Battlot escribe allí); prueba `open_bus_above_c000_on_hardware` |
| 9 | CPU (ZEXALL) | **hecho**: ZEXALL 67/67; `z80test` 137/152, con las mismas 15 fallas del `iz80` original (comportamientos no documentados). Corregirlas exige verificar antes el TV80 |
| 10 | Filtros paso alto | **hecho**: primer orden por canal (50, 72, 103 Hz); prueba `tone_frequency_and_highpass` |
| 11 | Fase inicial de los canales detenidos (XOR) | pendiente |
| 12 | Colores de TV | pendiente: hacen falta fotos o capturas para calibrar |

Abajo, el resultado de cada ítem resuelto y el detalle de los que siguen abiertos.

## Cómo se validó cada parte

Cada parte de la emulación, con la prueba. El modelo MiSTer se valida contra la simulación
del core; el modelo de hardware, contra mediciones hechas en aparatos reales por otras
personas.

### Modelo MiSTer (la referencia validada)

| Mejora | Prueba |
|---|---|
| Ciclos del `iz80` corregidos (prefijos DD/FD, `JR cc`, `JP cc`), registro R, `LD A,R`/`LD A,I` | rastreo instrucción por instrucción contra el TV80; RNG de Excite Mahjong igual |
| Interrupción vista 2 ciclos después; primera instrucción tras el reset 2 ciclos más corta | RAM idéntica más allá de ~150 cuadros |
| Video dibujado píxel a píxel, con las escrituras fechadas dentro de la instrucción | 0 píxeles diferentes en los 35 programas del relevamiento original (13 comerciales + 22 de Inufuto) |
| `HALT` que sale con la interrupción (el `iz80` original se colgaba) | prueba `halt_resumes_on_interrupt` |
| Memoria plana de 64 KB, sin espejos, y el rango 0xC000–0xFFFF guardando lo que se escribe, como en el core | regresión 152/152 idéntica (Battlot escribe por encima de 0xDC00) |
| Los "restos de pila" explicados: artefacto de comparar la RAM en medio de una instrucción | interrupciones e instrucciones idénticas; con la captura alineada, 0 diferencias en 1.100 cuadros (Dig Dug, Uchotos, Yewdow) y 2 en Ruptus, de instrucciones que empiezan justo en el cambio |

Los otros 4 homebrews (PVong, Simon, Demônios, PV-1000 Control Test), con el mismo método,
también son idénticos a la referencia: imagen 0 px, RAM igual en el cuadro 3000. Simon tiene 1 byte de pila en 0xBFE6 en el cuadro 1000, como Uchotos.

### Sonido (los dos modelos)

| Mejora | Prueba |
|---|---|
| Generador una octava por debajo de MiSTer: 8.739 / (63 − n) Hz | NESdev [N2] y MAME; pitido de la ROM de prueba a 380 Hz |
| Volúmenes −6 / −3 / 0 dB por canal y el puerto FB (enciende el sonido, modo XOR) | trazado de la placa [N2] |
| Filtros paso alto de primer orden por canal (50, 72, 103 Hz) | prueba `tone_frequency_and_highpass` |

### Modelo de hardware (el aparato)

| Mejora | Prueba |
|---|---|
| Interrupciones habilitadas por el puerto FC y reconocidas por FD; 16 por cuadro más la de prerrenderizado | prueba `frame_flag_needs_enabling_on_hardware`; la ROM de prueba se cuelga sin habilitarlas, como en MAME |
| Matriz en OR | prueba `matrix_rows_or_on_hardware` |
| Retardo eléctrico de la matriz (136 ciclos para soltar) | prueba `matrix_release_is_slow_on_hardware`; ROM `joystick` de lidnariq con seis columnas en la fila de 21 ciclos, como él midió |
| Puertos FE y FF completos (mapa y PCG móviles, 8 bancos, pantalla apagada) | las ROMs `scanwfe` y `wff-v2` de lidnariq coinciden con las fotos |
| Temporización del aparato: 288 × 262 a 59,299 Hz, robo de bus en las líneas de imagen, interrupción en el píxel 276, imagen en los píxeles 52–275 | **11 mediciones de Par en dos PV-1000, todas iguales** (8 escenarios de ciclos, 3 divisiones de pantalla); `irqtiming` y `toggle` de lidnariq coinciden con las fotos; prueba `bus_stealing_on_hardware` |
| Holgura del robo de bus (4 ciclos) | solo 4 y 5 ciclos pasan la prueba de división, como describió lidnariq |
| Imagen de TV: borde de 26 líneas y píxel 48:35 | NESdev [N4] |
| Espejos de la RAM en 0x8000–0xBFFF y bus abierto en 0xC000–0xFFFF | NESdev [N1]; prueba `open_bus_above_c000_on_hardware` |

### CPU (los dos modelos)

| Mejora | Prueba |
|---|---|
| IM 0 (como `RST 38h`) e IM 2 (vector en I × 256 + 0xFF), suponiendo el bus en 0xFF | prueba `interrupt_modes_0_and_2` (suposición no medida) |
| Verificación con las baterías estándar | ZEXALL 67/67; `z80test` 137/152, las mismas 15 fallas del `iz80` original |

### Comparación con MAME 0.289

En la prueba de ciclos de Par (solo INT1, imagen encendida), el aparato da $57AD / $5702
(versiones ADD / INC), Mil Capivaras da **$57AD / $5702** y MAME, $56F2 / $563A: MAME queda
~190–200 ciclos por cuadro por debajo del aparato.

### Herramientas que quedaron

- En el núcleo: `Pv1000::log_interrupts` y `Pv1000::trace` (desactivados por defecto).
- Pruebas que usan los archivos de `_referencias/` (fuera de git) y se omiten sin ellos:
  `par_cycle_counts_match_hardware`, `par_split_lines_match_hardware`.

## Modelo MiSTer

### 1. Restos de pila en 4 juegos (hecho)

No era un error de emulación. Las interrupciones y la secuencia de instrucciones son
idénticas al core; la diferencia venía de comparar la RAM en medio de una instrucción que
atraviesa el cambio de cuadro. Con la captura alineada al final de la instrucción:
0 diferencias en 1.100 cuadros en Dig Dug, Uchotos y Yewdow, y 2 en Ruptus, de
instrucciones que empiezan justo en el cambio.

## Modelo de hardware

### 2. Robo de bus por ciclo de máquina (no hace falta cambiarlo)

El emulador aproxima: una instrucción que ya empezó termina entera si acaba hasta 4 ciclos
después del BUSREQ; si no, se corta al inicio del BUSREQ. Eso coincide con las 11
mediciones del aparato (ítem 4). Solo las holguras de 4 y 5 ciclos pasan la prueba de
división, como describió lidnariq.

### 3. Detalles del video durante la imagen

- **Búsqueda anticipada:** el chip busca el tile y los colores 8 píxeles antes de
  dibujarlos. El emulador los lee en el propio píxel. Solo hace diferencia si algo cambia en
  medio de la línea de imagen, lo que casi nunca ocurre, porque la CPU está detenida.
- **Encender la imagen en medio de una línea** dibuja de 8 a 16 píxeles de basura en el
  aparato; el emulador no los dibuja.
- **Cambios de video cerca del final de la ventana libre:** Par vio una línea entera que
  parpadea entre las dos configuraciones. El emulador es determinista y siempre elige una de
  ellas.

### 4. Verificar los números medidos por Par (hecho)

Los programas de Par [N9] se ejecutaron en el modelo de hardware: los 8 escenarios de
ciclos y las 3 divisiones de pantalla dan los valores que Par vio en el aparato. Dos pruebas
lo comprueban (`par_cycle_counts_match_hardware`, `par_split_lines_match_hardware`), que se
saltan sin los archivos de `_referencias/`.

### 5. Tutankham contra MAME (hecho)

MAME también entra en el juego solo con Start; la pantalla de título del cuadro 3000 era la
vuelta después de un *game over*, que allí llega antes. MAME queda ~190–200 ciclos por
cuadro por debajo del aparato en la prueba de Par, y por eso las partidas divergen.

### 6. Matriz del joystick

- La **subida** (0 a 41 ciclos para que aparezca el botón) se trata como instantánea.
- Par midió que el tiempo necesario **varía entre consolas, mandos, televisores encendidos y
  temperatura**. Eso no se puede emular con un solo valor; el emulador usa el medio del
  rango medido (136 ciclos para soltar).

## Los dos modelos

### 7. Modos de interrupción IM 0 e IM 2 (hecho)

En el aparato, el chip no pone nada en el bus durante el reconocimiento. El emulador supone
el bus en 0xFF (resistencias de pull-up): IM 0 ejecuta `RST 38h`, igual que IM 1, e IM 2
usa el vector en `(I × 256) + 0xFF`. Es una suposición, **no confirmada** por ninguna
fuente; ninguno de los 35 programas del relevamiento original usa estos modos. Prueba
`interrupt_modes_0_and_2`.

### 8. El rango 0xC000–0xFFFF (hecho)

En el modelo de hardware, las escrituras se pierden y las lecturas dan 0xFF, como en el
aparato, donde no hay nada conectado ([N1]). El modelo MiSTer mantiene el comportamiento del
core, que guarda lo que se escribe (Battlot escribe por encima de 0xDC00). Prueba
`open_bus_above_c000_on_hardware`.

### 9. La CPU en su conjunto (hecho)

ZEXALL pasa 67/67. `z80test` pasa 137/152, con los mismos 15 fallos del `iz80` original
(comportamientos no documentados). Corregirlos exige verificar antes TV80.

## Sonido

### 10. Filtros paso alto (hecho)

El aparato tiene un paso alto en cada canal (50, 72 y 103 Hz). En uso normal son
inaudibles, pero **con el modo XOR se vuelven audibles** ([N2]). Emulados como filtro de
primer orden por canal; prueba `tone_frequency_and_highpass`.

### 11. Fase inicial de los canales detenidos

Con el modo XOR, un canal detenido queda en nivel alto o bajo al azar, lo que cambia el
volumen de otro canal ([N7], lidnariq y Par). El emulador siempre empieza en la misma fase.
Emular eso volvería el sonido no determinista; lo mejor es documentarlo y, como mucho,
sortearlo con una semilla fija.

## Imagen

### 12. Colores del aparato

En el aparato los colores salen **lavados**, menos saturados que el RGB puro (sobre todo el
rojo y el amarillo), porque el color de TV se arma en la placa ([N7]). El emulador usa el
RGB puro. Una opción de "colores de TV" necesitaría fotos o capturas de referencia para
calibrar.

## Orden sugerido

Los ítems 1, 2, 4, 5 y 7–10 están resueltos y el ítem 6 es parcial (ver la situación). Lo que falta:

| # | Ítem | Esfuerzo | Ganancia |
|---|---|---|---|
| 1 | Colores de TV (12), fase del XOR (11), detalles del video (3) | variable | refinamiento |
| 2 | Matriz del joystick (6), parcial: subida instantánea; la variación entre consolas no se puede emular | variable | refinamiento |
