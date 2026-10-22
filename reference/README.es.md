# Referencia: el core de MiSTer simulado con Verilator

[English](README.md) · [Português](README.pt-BR.md) · **Español**

La herramienta que valida Mil Capivaras. Simula ciclo a ciclo, con Verilator, el Verilog
del [core de MiSTer para el PV-1000](https://github.com/MiSTer-devel/Casio_PV-1000_MiSTer),
ejecuta una ROM con la misma entrada que el emulador y graba la imagen y la RAM, para
comparar los dos byte a byte. El método y los resultados están en
[Precisión y validación](../docs/accuracy.es.md).

No forma parte del emulador: nada de aquí se compila con `cargo build`.

## Uso

Requisitos: Verilator 5 y un compilador C++17; `7z`, para que `compare.sh` lea ROMs en
`.zip`/`.7z`.

```bash
make -C reference                         # genera reference/pv1000-ref (unos segundos)
cargo build --release                     # genera el emulador
reference/compare.sh target/release/mil_capivaras roms/*.bin
```

`compare.sh` ejecuta cada ROM en los dos, con `--press-start` (Start en los cuadros
120–129 y, a partir del 200, Fire 1 + derecha alternando cada 20 cuadros), y muestra los
píxeles y los bytes de RAM distintos en cada cuadro (0 = idénticos). Los cuadros son 300 y
3000 por defecto; otra lista va en `FRAMES`, por ejemplo `FRAMES="2 50 300 1000 3000"`. La
referencia tarda unos 30 s en 3000 cuadros (el emulador, menos de 1 s): con los cuadros por
defecto, unos 35 s por juego.

Solo la referencia:

```bash
reference/pv1000-ref JUEGO.bin --shot 3000 salida.ppm --press-start --dump-ram ram.bin
```

El emulador acepta las mismas opciones (`--shot`, `--press-start`, `--dump-ram`) y, por
defecto, ejecuta el modelo MiSTer, que es el que se compara aquí.

## Qué esperar

En el cuadro 3000, imagen y RAM idénticas en todos los juegos de la
[lista de compatibilidad](../docs/compatibility.es.md) (los homebrews en las versiones
indicadas allí). En otros cuadros, algunos juegos difieren en 1 o 2 bytes de la pila
(Amidar en el cuadro 300, por ejemplo): un efecto de grabar la RAM en medio de una
instrucción que cruza el final del cuadro, explicado en
[Precisión y validación](../docs/accuracy.es.md). Las ROMs no se incluyen.

## Archivos

| Archivo | Qué es |
|---|---|
| `rtl/CasioPV1000.v` | el core de MiSTer (Flandango), con los ajustes para la simulación listados en `rtl/CasioPV1000.sim.patch` |
| `rtl/sim_top.v` | el módulo superior, en lugar del framework de MiSTer: relojes, RAM de 64 KB, carga del cartucho |
| `rtl/cpu_z80.v` | la interfaz de la CPU del core, con TV80 en lugar de T80 (VHDL, que Verilator no lee) |
| `rtl/tv80/` | TV80, el Z80 en Verilog de Guy Hutchison |
| `sim.h` | la simulación: relojes, captura de la imagen, un cuadro |
| `ref_main.cpp` | la línea de comandos (`pv1000-ref`) |
| `compare.sh` | ejecuta la referencia y el emulador y los compara |

La simulación prescinde del `clk_sys` de 71,59 MHz del core para ir lo bastante rápido: E/S
e interrupciones van en el flanco de bajada de `clk_cpu`, el sonido en `clk_vdp`, y la
imagen la dibuja `sim.h` a partir de la RAM, con la misma decodificación que la máquina de
estados de vídeo del core. Tres detalles de la simulación se corrigieron antes de las
comparaciones: TV80 se compila con `TV80_REFRESH` (sin eso, el registro R siempre es 0), el
cuadro termina cuando el barrido vuelve a la línea 0 (y no en el VSync), y el reset se
suelta en una fase fija entre los relojes de la CPU y del vídeo.

## Licencias

| Parte | Licencia |
|---|---|
| Core de MiSTer (`rtl/CasioPV1000.v`) | GPL-2.0 (`rtl/LICENSE`) |
| TV80 (`rtl/tv80/`) | MIT (`rtl/tv80/LICENSE`) |
| El resto de esta carpeta | GPL-2.0 o posterior |

El resto de la carpeta es GPL-2.0 o posterior, y no GPL-3.0 como el emulador, porque se
compila en un solo programa con el core de MiSTer, que es GPL-2.0. La referencia es un
programa aparte: no está enlazada con el emulador.
