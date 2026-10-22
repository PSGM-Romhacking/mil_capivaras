# Compatibilidade

[English](compatibility.md) · **Português** · [Español](compatibility.es.md)

As referências entre colchetes ([N2], [M1]…) estão em [Referências](references.pt-BR.md).

**Como foi testado:** cada jogo rodou no Mil Capivaras e na referência do MiSTer com a
mesma entrada (Start nos quadros 120–129; Fire 1 + direita alternando a cada 20 quadros a
partir do 200). A imagem foi comparada nos quadros 2, 50, 300, 1000 e 3000, e a RAM de 64
KB nos quadros 1000 e 3000. **No quadro 3000, imagem e RAM são idênticas nos 39 programas**; a
coluna de RAM abaixo mostra o quadro 1000, o único com alguma diferença.

**Status:** "Perfeito" = imagem idêntica à referência em todos os quadros testados e
RAM idêntica, ou diferente só em posições **já descartadas da pilha** (abaixo do topo
dela; ver [Precisão e validação](accuracy.pt-BR.md)).

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

## Origem dos homebrews

A maioria é do **Inufuto**, desenvolvedor japonês que publica jogos novos para dezenas de
máquinas de 8 bits, com ROMs gratuitas no site do Inufuto [I1]. Os mais novos têm
o código no GitHub, em <https://github.com/inufuto/Cate_examples>, escritos em **Cate**,
a linguagem e compilador do próprio Inufuto (MIT, <https://github.com/inufuto/Cate>); os outros têm o
código num zip no site. Não há página no itch.io.

| Jogo | Código-fonte |
|---|---|
| Aerial | não publicado para PV-1000 (versões de outras máquinas na [página](https://inufuto.web.fc2.com/8bit/aerial/)) |
| AntiAir | [zip](https://inufuto.web.fc2.com/8bit/pv1000/antiair_src.zip) |
| Ascend | [zip](https://inufuto.web.fc2.com/8bit/pv1000/ascend_src.zip) |
| Awass | só para outras máquinas, no [Cate_examples](https://github.com/inufuto/Cate_examples) (a pasta de PV-1000 que o site indica não existe) |
| Battlot | não publicado para PV-1000 (versões de outras máquinas na [página](https://inufuto.web.fc2.com/8bit/battlot/)) |
| Bootskell | não publicado para PV-1000 (versões de outras máquinas na [página](https://inufuto.web.fc2.com/8bit/bootskell/)) |
| Cacorm | [zip](https://inufuto.web.fc2.com/8bit/pv1000/cacorm_src.zip) |
| Cavit | não publicado para PV-1000 (versões de outras máquinas na [página](https://inufuto.web.fc2.com/8bit/cavit/)) |
| Cracky | [zip](https://inufuto.web.fc2.com/8bit/pv1000/cracky_src.zip) |
| Demônios | [GitHub](https://github.com/PSGM-Romhacking/pv-1000_demonios) |
| Guntus | [zip](https://inufuto.web.fc2.com/8bit/pv1000/guntus_src.zip) |
| Hopman | [zip](https://inufuto.web.fc2.com/8bit/pv1000/hopman_src.zip) |
| Lift | não publicado para PV-1000 (versões de outras máquinas na [página](https://inufuto.web.fc2.com/8bit/lift/)) |
| Mazy | não publicado para PV-1000 (versões de outras máquinas na [página](https://inufuto.web.fc2.com/8bit/mazy/)) |
| Mazy2 | [GitHub](https://github.com/inufuto/Cate_examples/tree/main/pv1000/mazy2) |
| Mieyen | [GitHub](https://github.com/inufuto/Cate_examples/tree/main/pv1000/mieyen) |
| Neuras | não publicado para PV-1000 (versões de outras máquinas na [página](https://inufuto.web.fc2.com/8bit/neuras/)) |
| Osotos | [zip](https://inufuto.web.fc2.com/8bit/pv1000/osotos_src.zip) |
| PV-1000 Control Test | [GitHub](https://github.com/PSGM-Romhacking/pv-1000_control_test_rom) |
| PVong | [GitHub](https://github.com/joffb/pvong) |
| Ruptus | não publicado para PV-1000 (versões de outras máquinas na [página](https://inufuto.web.fc2.com/8bit/ruptus/)) |
| Simon | não publicado (o anexo só tem a ROM) |
| Svellas | só para outras máquinas, no [Cate_examples](https://github.com/inufuto/Cate_examples) (a pasta de PV-1000 que o site indica não existe) |
| SwordWork | [GitHub](https://github.com/inufuto/Cate_examples/tree/main/pv1000/sword) |
| Uchotos | [GitHub](https://github.com/inufuto/Cate_examples/tree/main/pv1000/uchotos) |
| Yewdow | [zip](https://inufuto.web.fc2.com/8bit/pv1000/yewdow_src.zip) |

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
