# Banco de jogos

[English](sheets.md) · **Português** · [Español](sheets.es.md)

A ficha (clique direito no jogo › **Ficha do jogo…**, ou **Ajuda › Ficha do jogo…**) e o
nome e a capa do jogo na biblioteca vêm do **banco de jogos**, embutido no programa. Ele
fica separado dos textos do próprio emulador (menus, botões), que são as traduções da
interface em `assets/i18n/`.

| Arquivo | O que tem |
|---|---|
| `assets/games/games.json` | os dados de cada jogo, iguais em todos os idiomas |
| `assets/games/games.en.json` | os textos de cada jogo em inglês (o texto de origem) |
| `assets/games/games.pt.json`, `games.es.json` | os mesmos textos em português e espanhol |

O banco começou a partir da lista de software do MAME para o PV-1000 (CC0,
<https://github.com/mamedev/mame/blob/master/hash/pv1000.xml>) e é mantido aqui: os 13
cartuchos e os homebrews têm o mesmo tipo de entrada.

## `games.json`

```json
{
  "pooyan": {
    "title": "Pooyan",
    "alt_title": "プーヤン",
    "year": "1982",
    "publisher": "Casio",
    "developer": "Konami (arcade, 1982)",
    "serial": "GPA-101",
    "roms": [{ "name": "Pooyan (Japan)", "crc": "bd6edc48", "size": 8192 }]
  }
}
```

| Campo | Observações |
|---|---|
| chave (`pooyan`) | o id do jogo: minúsculas, sem espaços; a mesma chave nos arquivos de textos |
| `title` | obrigatório |
| `alt_title` | título original (japonês) |
| `year`, `publisher`, `developer` | |
| `serial` | código do cartucho (`GPA-101`) |
| `players` | máximo de jogadores ao mesmo tempo: `1` ou `2`. Só o número: a frase ("1 ou 2 jogadores") vem da tradução da interface |
| `roms` | as ROMs do jogo: uma, ou uma por versão (o Demônios tem uma ROM por idioma, e todas são o mesmo jogo) |
| `roms[].name` | o nome No-Intro da ROM (`Pooyan (Japan)`, `Cacorm (World) (Aftermarket) (Unl)`): o nome da capa, da tela de título e da tela de jogo, como no RetroArch. São baixadas do [libretro-thumbnails](https://thumbnails.libretro.com/Casio%20-%20PV-1000/) ou, para os homebrews, do [emulation-thumbnails](https://github.com/PSGM-Romhacking/emulation-thumbnails) |
| `roms[].crc` | CRC32 da ROM: é assim que o jogo é reconhecido, qualquer que seja o nome do arquivo |
| `roms[].size` | tamanho em bytes, quando conhecido. Os homebrews também são reconhecidos sem o preenchimento com zeros até 8 ou 16 KB que os dumps do MAME têm |

Para achar o CRC de uma ROM: `./mil-capivaras-0.9.0-linux-x86_64 --identify PASTA`. As ROMs
fora do banco aparecem como "(fora do banco de jogos)".

## `games.en.json`, `games.pt.json`, `games.es.json`

```json
{
  "pooyan": {
    "description": "A mamãe porca sobe e desce um penhasco num elevador…",
    "notes": "…"
  }
}
```

| Campo | Observações |
|---|---|
| `description` | aparece logo abaixo do cartucho, sem rótulo |
| `notes` | aparece no fim da ficha |

Os dois campos são opcionais. Sem o texto no idioma atual, a ficha mostra o inglês. Um idioma
novo é um `games.<código>.json` novo, com os mesmos jogos; ele entra no programa na versão
seguinte ([translating.pt-BR.md](translating.pt-BR.md)).

## Acrescentando um jogo

1. Ache o CRC com `--identify`.
2. Acrescente uma entrada no `games.json`.
3. Acrescente os textos em `games.en.json`, `games.pt.json` e `games.es.json`.
4. Confira: `cargo test --release` e `./mil-capivaras-0.9.0-linux-x86_64 --info JOGO`.

## Regras (conferidas pelos testes)

- Todos os arquivos de textos têm **os mesmos jogos e os mesmos campos**; só o texto muda.
- Todo jogo dos arquivos de textos existe no `games.json`; dois jogos nunca têm o mesmo CRC.
- Um campo desconhecido (ex.: `descripton`) é erro.
- Só entra conteúdo verificável: desenvolvedora com fonte, descrição do autor ou do jogo.

**Japonês:** os títulos originais usam um recorte embutido da Noto Sans CJK JP
(`assets/fonts/`, OFL): kana, pontuação e só os kanji já usados. Um título com
kanji novo exige refazer o recorte: `python3 tools/fonts/generate.py "新しい漢字"` (o gerador
fica fora do repositório).
