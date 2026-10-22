# Precisión y validación

[English](accuracy.md) · [Português](accuracy.pt-BR.md) · **Español**

Cómo se compara Mil Capivaras con el core de MiSTer y con la consola, el resultado y lo que aún falta.

Las referencias entre corchetes ([N2], [M1]…) están en [Referencias](references.es.md).

## Cómo se hace la comparación

**1. Las dos "consolas"**
- **La referencia:** el core de MiSTer es la descripción del circuito del PV-1000 (en Verilog).
  Un programa (Verilator) simula ese circuito en el PC, cable por cable, reloj por reloj. Es
  lento, pero es el propio hardware de MiSTer funcionando.
- **Mil Capivaras:** el emulador, escrito desde cero en Rust, rápido.

Si Mil Capivaras es correcto, los dos tienen que hacer **exactamente lo mismo**.

**2. El mismo juego, la misma "mano"**

Los dos ejecutan la misma ROM, sin nadie jugando. Un "jugador automático" pulsa los
botones en los mismos momentos en los dos:
- pulsa **Start** del cuadro 120 al 129 (el juego sale de la pantalla de título);
- a partir del cuadro 200, alterna disparo + derecha cada 20 cuadros.

Es como dos músicos tocando la misma partitura: si los dos son fieles, el sonido sale igual.

**3. Pausa y foto en el mismo instante**

Después de N cuadros (el PV-1000 dibuja ~60 por segundo, así que el cuadro 3000 son unos 50
segundos de juego), los dos se detienen y guardan:
- **la imagen de la pantalla:** 224 × 224 puntos;
- **la memoria RAM entera:** todo lo que el juego está "pensando" (puntos, vidas, posición
  de los enemigos…).

**4. Comparar, punto por punto**

Un script compara los archivos:
- **imagen:** cuenta cuántos puntos tienen un color distinto. 0 = pantallas idénticas;
- **RAM:** cuenta cuántos bytes tienen un valor distinto. 0 = el juego está en el mismo estado en los
  dos.

La RAM es la prueba más fuerte: dos juegos pueden mostrar la misma pantalla y aun así estar
"pensando" distinto por dentro.

**5. Repetir en varios momentos y juegos**

Esto se hizo en los cuadros 2, 50, 300, 1000 y 3000, para los 39 programas. Si una diferencia
apareciera en un momento y desapareciera después, o solo apareciera con el juego funcionando desde hace mucho
tiempo, la comparación la detectaría.

**6. Cuando sale distinto: encontrar dónde empezó**

Se prueba la mitad del camino (cuadro 1500, luego 750…) hasta encontrar el primer cuadro en
el que los dos se separan. Desde ahí, un rastreo compara los dos **instrucción por instrucción**,
hasta el punto exacto en el que la CPU de cada uno hizo cosas distintas. Así se descubrió,
por ejemplo, que la biblioteca del Z80 contaba mal los ciclos.

**Resultado:** en el cuadro 3000, los 39 programas tienen pantalla y memoria **idénticas** en los dos. Es
la prueba de que Mil Capivaras se comporta como el hardware de MiSTer.

## Metodología (detalles técnicos)

1. **Referencia:** el Verilog del core de MiSTer simulado ciclo a ciclo con Verilator
   (en [`reference/`](../reference/README.es.md), con el script de comparación), con la CPU TV80, que es el T80 de MiSTer en Verilog.
2. **Misma entrada:** con `--press-start`, los dos pulsan Start en los cuadros 120–129 y
   alternan Fire 1 + derecha cada 20 cuadros a partir del 200.
3. **Misma salida:** imagen de 224×224 y RAM de 64 KB en el cuadro N.
4. **Comparación:** píxeles y bytes distintos.
5. **Bisección:** si hay diferencia, se encuentra el primer cuadro, y desde ahí el rastreo instrucción a
   instrucción (PC, ciclo y R en cada búsqueda de opcode) encuentra la primera instrucción divergente.
6. **Herramientas de diagnóstico.** En el núcleo: `Pv1000::log_interrupts` registra cada
   interrupción aceptada (línea, posición, PC) y `Pv1000::trace`, cada instrucción iniciada
   (línea, posición, PC, A, R). En la simulación del core, las mismas medidas más la RAM en cada
   cuadro y un "vigía" de dirección; esas herramientas de la simulación no se guardaron, y
   `reference/` trae solo la comparación.
   Dos cuidados aprendidos al comparar:
   - el TV80 graba el resultado en el acumulador ya durante la instrucción siguiente, así
     que el A leído al inicio de ella está atrasado; hay que comparar la secuencia de PCs,
     no el A instrucción a instrucción;
   - las instrucciones con prefijo (DD, ED, FD, CB) hacen dos búsquedas de opcode en el TV80
     y una en el emulador; la interrupción aparece como una búsqueda de más en el TV80.
7. **Misma foto.** El emulador ejecuta instrucciones enteras; el core escribe en el ciclo de
   máquina. Para comparar la RAM al final de un cuadro, la simulación usada en la
   investigación terminaba la instrucción en curso antes de grabar. La de `reference/` graba
   en el cambio de cuadro, así que muestra los restos de pila descritos abajo.

## Resultado

- **39 programas** (13 comerciales y 26 homebrews), imagen en los cuadros 2, 50, 300, 1000 y
  3000: **0 píxeles distintos en todos**. RAM en el cuadro 3000: **idéntica en los 39**. En el
  cuadro 1000, idéntica en 34; en los otros 5, 1 o 2 bytes de pila difieren por un artefacto de la
  comparación, no de la emulación (ver "Restos de pila" abajo). Detalles en la
  [Compatibilidad](compatibility.es.md).
- El PC, el ciclo y el R coinciden **en cada instrucción** en las primeras 40.000 líneas (~152
  cuadros) de los 16 juegos rastreados.

**Qué hubo que corregir para llegar ahí:**

| Problema | Dónde | Efecto antes de la corrección |
|---|---|---|
| Ciclos erróneos en `iz80` (DD/FD, `JR cc`, `JP cc`) | CPU | Excite Mahjong un 2,5% distinto; el RNG sorteaba otras fichas |
| R nunca incrementado; `LD A,R` sin S/Z | CPU | Super Cobra, Space Panic y Excite Mahjong usan el R como aleatorio |
| Interrupción vista en el acto, no 2 ciclos después | temporización | RAM divergiendo después de ~150 cuadros |
| Primera instrucción tras el reset 2 ciclos más larga | temporización | fase errónea para siempre (los bucles `JR $` la mantienen) |
| Línea dibujada de una vez | video | hasta 80 píxeles erróneos por cuadro en pantallas que cambian durante la imagen |
| Última columna dibujada | video | 1 columna distinta |

**La propia referencia tenía defectos**, corregidos en ella antes de las comparaciones:
- TV80 se compilaba sin `TV80_REFRESH` (R siempre 0) y calculaba S y Z de `LD A,R` a
  partir del I;
- el cuadro terminaba en el VSync (línea 259), y no en la vuelta a la línea 0, lo que desfasaba
  mandos y capturas en ~3 líneas;
- la fase entre el reloj de la CPU y el del video dependía del tamaño de la ROM.

**Qué sigue siendo distinto:**
- **"Restos de pila" (no es un error de emulación):** en Dig Dug, Ruptus, Uchotos y
  Yewdow, 1 o 2 bytes difieren en algunos cuadros. La causa:
  - las **interrupciones** se aceptan en el mismo cuadro, la misma línea y la misma
    instrucción en los dos, en los 4 juegos (de 11.916 a 14.209 interrupciones por juego en
    1000–1100 cuadros);
  - la **secuencia de instrucciones** es idéntica (comparada búsqueda a búsqueda en el
    cuadro donde aparecía la primera diferencia);
  - la diferencia viene **del instante de la comparación**: la RAM se graba en el cambio de
    cuadro, y una instrucción a medias (por ejemplo, `LD (IY+0),A` empezando al final de la
    línea 262) ya escribió en el emulador, que ejecuta instrucciones enteras, pero todavía no
    en el core, que escribe en su ciclo de máquina;
  - **prueba:** grabando la RAM del core después de terminar la instrucción en curso, Dig
    Dug, Uchotos y Yewdow dan **cero diferencias en 1100 cuadros**. En Ruptus quedan 2 en
    1100 cuadros, ambas de una instrucción que empieza justo en el cambio (posición 0 o
    380), que cada lado pone en un cuadro distinto por ±1 píxel de diferencia en la medida
    del tiempo.
  - **Simon** tiene la misma sobra: 1 byte en 0xBFE6, la misma
    dirección que en Uchotos.

  Las herramientas usadas (registro de interrupciones y rastreo de instrucciones en el
  núcleo, `log_interrupts`/`trace`, y las mismas opciones en la simulación del core) se
  describen en [Metodología](#metodología-detalles-técnicos).
- **Flags no documentados:** los bits 3 y 5 del registro F después de `BIT n,(HL)`. El
  `iz80` los toma de la dirección, TV80 los pone a cero y el T80 de MiSTer usa el registro interno
  MEMPTR. También aparece solo como resto en la pila (Amidar, en el cuadro 1500).

## Clasificación

En la escala de la **Emulation General Wiki** [G1]:

| Nivel | Definición (resumida) | Mil Capivaras |
|---|---|---|
| Baja | muchos fallos de imagen y sonido; la velocidad por encima de todo | — |
| Media | buena compatibilidad con los juegos comerciales, sin cubrir todos los matices | — |
| **Alta** | reproduce los componentes de cerca, con más costo de procesamiento; pocos fallos, casos límite tratados; no necesita ser ciclo a ciclo | **sí** |
| Ciclo a ciclo | cada componente avanza según su comportamiento en cada ciclo, sincronizado al nivel de un ciclo de reloj | no, por construcción |
| Subciclo | considera los dos flancos de cada ciclo de reloj | no |

**Por qué "alta" y no "ciclo a ciclo":**
- la CPU ejecuta **instrucciones enteras** (con el número exacto de ciclos de cada una), no
  ciclo a ciclo;
- el momento de cada escritura dentro de la instrucción viene de una **regla**, no del bus
  simulado;
- el video no simula la máquina de estados de ~11 ciclos de `clk_sys` por píxel, solo su
  resultado.

**Qué justifica el tope del nivel "alta":** el resultado es **idéntico** al de una
simulación ciclo a ciclo del hardware en todos los juegos y cuadros probados. En la práctica,
Mil Capivaras es **exacto al ciclo en los límites de instrucción** (el PC, el ciclo y el R coinciden en
cada búsqueda de opcode).

**Salvedades honestas:**
- la referencia es el **core de MiSTer**, no un aparato real. "Idéntico" quiere decir
  idéntico al core, y el core difiere del aparato en temporización, interrupciones, puertos FE y
  FF, borde y proporción del píxel ([Cómo difieren las implementaciones](hardware.es.md#cómo-difieren-las-implementaciones)). En esos
  puntos, el modelo de hardware sigue al aparato, verificado con las mediciones de Par
  (abajo), pero sin una referencia para comparar byte a byte;
- el sonido es la excepción: sigue la documentación del hardware, no el core;
- la referencia usa TV80, no el T80 de MiSTer, con la E/S en el reloj de la CPU en lugar de
  `clk_sys`.

## Validación contra la consola: las mediciones de Par

Par (team-europe) midió el PV-1000 en **dos consolas** con programas de prueba propios,
publicados con el código fuente [N9]. Como muestran números en pantalla, se pueden ejecutar
en el emulador y comparar con lo que él anotó, **sin tener la consola**. Los programas están
en `_referencias/par/` (fuera de git), y dos pruebas automáticas hacen la comparación
(`par_cycle_counts_match_hardware` y `par_split_lines_match_hardware`, omitidas si falta la
carpeta).

**1. Ciclos entre interrupciones** (*Cycle Count Before Interrupt*). El programa cuenta
cuántos ciclos consigue ejecutar su bucle principal entre dos interrupciones. Hay dos
versiones: una cuenta en pasos de 11 ciclos (`ADD HL,DE`) y otra en pasos de 4 y 6
(`INC E`/`INC DE`). Valores en hexadecimal, como en pantalla:

| Versión | Interrupciones | Imagen | Par (PV-1000 real) | Mil Capivaras (hardware) |
|---|---|---|---|---|
| ADD | solo INT1 | encendida | $57AD / $57B8 | **$57AD** |
| ADD | solo INT1 | apagada | $EBBF / $EBCA | **$EBBF** |
| INC | solo INT1 | encendida | $56FE / $5702 / $5706 | **$5702** |
| INC | solo INT1 | apagada | $EBBE / $EBC2 | **$EBBE** |
| ADD | INT1 + INT16 | encendida | $21B5; demás $038B / $0396 | **$21B5; $038B / $0396** |
| ADD | INT1 + INT16 | apagada | $B5BC / $B5C7; demás $038B / $0396 | **$B5BC; $038B / $0396** |
| INC | INT1 + INT16 | encendida | $2100; demás $038A / $038E | **$2100; $038A / $038E** |
| INC | INT1 + INT16 | apagada | $B5BE / $B5C2; demás $038A / $038E | **$B5BE; $038A / $038E** |

**Todos los valores están entre los que Par vio en la consola.** Las versiones ADD e INC dan
números distintos con la imagen encendida porque el robo de bus corta instrucciones de
tamaños distintos en puntos distintos, y el emulador lo reproduce.

**2. Dónde cae una división de pantalla** (*Screen Split Tool*). Tres rutinas de prueba
cambian la configuración del video después de ~3520 ciclos, cada una esperando con
instrucciones distintas. En la consola, la versión con `NOP` dividió la pantalla **3 líneas
más abajo** que la versión con bucle en BC, y la versión con `DJNZ`, **2 líneas más abajo**.
En Mil Capivaras: líneas 66, 63 y 65, es decir, **3 y 2 líneas más abajo**, exactamente.

**3. El margen del robo de bus.** El emulador deja terminar entera la instrucción que
termina hasta 4 ciclos después del pedido de bus. Variando ese valor, solo 4 y 5 ciclos
pasan la prueba de división (2, 3 y 6 fallan), lo que confirma lo que lidnariq escribió en
el foro ("hasta 5 ciclos más antes del BUSACK") [N7].

**4. Comparación con MAME 0.289**, en la misma prueba de ciclos (solo INT1, imagen
encendida):

| Versión | PV-1000 real | Mil Capivaras | MAME 0.289 |
|---|---|---|---|
| ADD | $57AD (22.445) | **$57AD (22.445)** | $56F2 (22.258) |
| INC | $5702 (22.274) | **$5702 (22.274)** | $563A (22.074) |

MAME queda ~190–200 ciclos por cuadro por debajo de la consola. Eso explica por qué las
partidas automáticas se separan con el tiempo entre los dos. Eso explica también
Tutankham: con Start solo, MAME también entra al juego; el título que aparecía en el
cuadro 3000 era la vuelta al título después de un *game over* que allí llegó antes.

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

### Herramientas que quedaron

- En el núcleo: `Pv1000::log_interrupts` y `Pv1000::trace` (desactivados por defecto).
- Pruebas que usan los archivos de `_referencias/` (fuera de git) y se omiten sin ellos:
  `par_cycle_counts_match_hardware`, `par_split_lines_match_hardware`.

## Cada ítem en detalle

### Modelo MiSTer

#### 1. Restos de pila en 4 juegos (hecho)

No era un error de emulación. Las interrupciones y la secuencia de instrucciones son
idénticas al core; la diferencia venía de comparar la RAM en medio de una instrucción que
atraviesa el cambio de cuadro. Con la captura alineada al final de la instrucción:
0 diferencias en 1.100 cuadros en Dig Dug, Uchotos y Yewdow, y 2 en Ruptus, de
instrucciones que empiezan justo en el cambio.

### Modelo de hardware

#### 2. Robo de bus por ciclo de máquina (no hace falta cambiarlo)

El emulador aproxima: una instrucción que ya empezó termina entera si acaba hasta 4 ciclos
después del BUSREQ; si no, se corta al inicio del BUSREQ. Eso coincide con las 11
mediciones del aparato (ítem 4). Solo las holguras de 4 y 5 ciclos pasan la prueba de
división, como describió lidnariq.

#### 3. Detalles del video durante la imagen

- **Búsqueda anticipada:** el chip busca el tile y los colores 8 píxeles antes de
  dibujarlos. El emulador los lee en el propio píxel. Solo hace diferencia si algo cambia en
  medio de la línea de imagen, lo que casi nunca ocurre, porque la CPU está detenida.
- **Encender la imagen en medio de una línea** dibuja de 8 a 16 píxeles de basura en el
  aparato; el emulador no los dibuja.
- **Cambios de video cerca del final de la ventana libre:** Par vio una línea entera que
  parpadea entre las dos configuraciones. El emulador es determinista y siempre elige una de
  ellas.

#### 4. Verificar los números medidos por Par (hecho)

Los programas de Par [N9] se ejecutaron en el modelo de hardware: los 8 escenarios de
ciclos y las 3 divisiones de pantalla dan los valores que Par vio en el aparato. Dos pruebas
lo comprueban (`par_cycle_counts_match_hardware`, `par_split_lines_match_hardware`), que se
saltan sin los archivos de `_referencias/`.

#### 5. Tutankham contra MAME (hecho)

MAME también entra en el juego solo con Start; la pantalla de título del cuadro 3000 era la
vuelta después de un *game over*, que allí llega antes. MAME queda ~190–200 ciclos por
cuadro por debajo del aparato en la prueba de Par, y por eso las partidas divergen.

#### 6. Matriz del joystick

- La **subida** (0 a 41 ciclos para que aparezca el botón) se trata como instantánea.
- Par midió que el tiempo necesario **varía entre consolas, mandos, televisores encendidos y
  temperatura**. Eso no se puede emular con un solo valor; el emulador usa el medio del
  rango medido (136 ciclos para soltar).

### Los dos modelos

#### 7. Modos de interrupción IM 0 e IM 2 (hecho)

En el aparato, el chip no pone nada en el bus durante el reconocimiento. El emulador supone
el bus en 0xFF (resistencias de pull-up): IM 0 ejecuta `RST 38h`, igual que IM 1, e IM 2
usa el vector en `(I × 256) + 0xFF`. Es una suposición, **no confirmada** por ninguna
fuente; ninguno de los 35 programas del relevamiento original usa estos modos. Prueba
`interrupt_modes_0_and_2`.

#### 8. El rango 0xC000–0xFFFF (hecho)

En el modelo de hardware, las escrituras se pierden y las lecturas dan 0xFF, como en el
aparato, donde no hay nada conectado ([N1]). El modelo MiSTer mantiene el comportamiento del
core, que guarda lo que se escribe (Battlot escribe por encima de 0xDC00). Prueba
`open_bus_above_c000_on_hardware`.

#### 9. La CPU en su conjunto (hecho)

ZEXALL pasa 67/67. `z80test` pasa 137/152, con los mismos 15 fallos del `iz80` original
(comportamientos no documentados). Corregirlos exige verificar antes TV80.

### Sonido

#### 10. Filtros paso alto (hecho)

El aparato tiene un paso alto en cada canal (50, 72 y 103 Hz). En uso normal son
inaudibles, pero **con el modo XOR se vuelven audibles** ([N2]). Emulados como filtro de
primer orden por canal; prueba `tone_frequency_and_highpass`.

#### 11. Fase inicial de los canales detenidos

Con el modo XOR, un canal detenido queda en nivel alto o bajo al azar, lo que cambia el
volumen de otro canal ([N7], lidnariq y Par). El emulador siempre empieza en la misma fase.
Emular eso volvería el sonido no determinista; lo mejor es documentarlo y, como mucho,
sortearlo con una semilla fija.

### Imagen

#### 12. Colores del aparato

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
