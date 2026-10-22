(function () {
  var T = {
    pt: {
      nav_features: "Recursos", nav_screens: "Capturas", nav_download: "Baixar",
      hero_title: "Um emulador preciso do Casio PV-1000",
      hero_lead: "Jogue os jogos do Casio PV-1000 no seu computador. Um emulador escrito em Rust, portado do core FPGA do MiSTer.",
      btn_download: "Baixar", btn_source: "Código-fonte", btn_manual: "Manual",
      stat_commercial: "jogos comerciais", stat_homebrew: "homebrews", stat_platforms: "plataformas",
      nav_compat: "Compatibilidade",
      compat_title: "Compatibilidade", compat_all: "Ver a lista completa",  compat_sub: "Cada jogo foi executado lado a lado com o core do MiSTer. No quadro 3000, imagem e memória são idênticas nos 39.",
      compat_s1: "jogos perfeitos",
      compat_s2: "comerciais",
      compat_s3: "homebrews",
      compat_search: "Buscar por nome, serial ou CRC",
      compat_c_game: "Título",
      compat_c_serial: "Serial",
      compat_c_type: "Tipo",
      compat_c_year: "Ano",
      compat_c_status: "Status",
      compat_c_links: "Links",
      compat_t_com: "Comercial",
      compat_t_hb: "Homebrew",
      compat_ok: "✓ Perfeito",
      compat_l_page: "Página",
      compat_l_dl: "Baixar",
      compat_none: "Nenhum jogo encontrado.",
      more: "Saiba mais →",
      compat_more: "Como os testes foram feitos e notas de cada jogo →",
      dl_bsd: "O FreeBSD precisa de alguns pacotes extras",
      dl_build: "Compilar a partir do código",
      nav_docs: "Docs",
      feat_title: "Recursos", feat_sub: "Tudo o que se espera de um emulador moderno.",
     
      f2_t: "Save states e rewind", f2_d: "Vários slots, avanço rápido e retrocesso enquanto você segura uma tecla ou botão.",
      f3_t: "Gravação de GIF e MP4", f3_d: "Capture telas, GIFs e vídeos MP4 (o MP4 precisa do ffmpeg).",
      f4_t: "Depurador e cheats", f4_d: "Visores de CPU, memória e tiles, e um gerenciador de cheats.",
      f5_t: "Core libretro", f5_d: "Jogue dentro do RetroArch no Linux, Windows e FreeBSD.",
     
      shots_title: "Telas",
      s1: "Biblioteca no modo sofá", s2: "Jogo com moldura de TV", s4: "Biblioteca com imagens de jogo",
      s5: "Ficha do jogo", s6: "Menu rápido", s7: "Cheats", s8: "Depurador: CPU", s9: "Depurador: memória", s11: "Biblioteca em lista", s10: "Depurador: tiles",
      cl0: "A primeira versão: os 13 cartuchos e 26 homebrews, validados contra o core do MiSTer, para Linux, Windows, FreeBSD e RetroArch.", cl_all: "Changelog completo (em inglês) →",
      cl1: "Modo lista na biblioteca: título, ano, desenvolvedora, editora, jogadores, tempo de jogo e última vez; clique num cabeçalho para ordenar.",
      cl2: "Tempo de jogo e histórico de cada jogo.",
      cl3: "Capas 20% maiores, espalhadas pela largura toda da janela.",
      cl4: "Suas capas próprias também na ficha do jogo e na moldura de capa.",
      cl5: "Relatório de erro: se o programa travar, a próxima abertura mostra onde está o relatório.",
      cl6: "A janela nunca abre maior que a tela, e copiar texto funciona.",
      cl7: "A moldura Console saiu: o desenho não parecia o PV-1000 de verdade.",
      cl8: "Menu rápido mais estreito, com as dicas de botões em duas linhas.",
      dl_title: "Baixar", dl_sub: "Versão 0.9.1.",
      dl_btn: "Todas as versões e checksums",
      foot_lic: "Software livre sob GPL-3.0 ou posterior.",
    },
    es: {
      nav_features: "Funciones", nav_screens: "Capturas", nav_download: "Descargar",
      hero_title: "Un emulador preciso de la Casio PV-1000",
      hero_lead: "Juega a los juegos de la Casio PV-1000 en tu ordenador. Un emulador escrito en Rust, portado del core FPGA de MiSTer.",
      btn_download: "Descargar", btn_source: "Código fuente", btn_manual: "Manual",
      stat_commercial: "juegos comerciales", stat_homebrew: "homebrews", stat_platforms: "plataformas",
      nav_compat: "Compatibilidad",
      compat_title: "Compatibilidad", compat_all: "Ver la lista completa",  compat_sub: "Cada juego se ejecutó lado a lado con el core de MiSTer. En el fotograma 3000, imagen y memoria son idénticas en los 39.",
      compat_s1: "juegos perfectos",
      compat_s2: "comerciales",
      compat_s3: "homebrews",
      compat_search: "Buscar por nombre, serial o CRC",
      compat_c_game: "Título",
      compat_c_serial: "Serial",
      compat_c_type: "Tipo",
      compat_c_year: "Año",
      compat_c_status: "Estado",
      compat_c_links: "Enlaces",
      compat_t_com: "Comercial",
      compat_t_hb: "Homebrew",
      compat_ok: "✓ Perfecto",
      compat_l_page: "Página",
      compat_l_dl: "Descargar",
      compat_none: "No se encontraron juegos.",
      more: "Más información →",
      compat_more: "Cómo se hicieron las pruebas y notas de cada juego →",
      dl_bsd: "FreeBSD necesita algunos paquetes extra",
      dl_build: "Compilar desde el código",
      nav_docs: "Documentación",
      feat_title: "Funciones", feat_sub: "Todo lo que se espera de un emulador moderno.",
     
      f2_t: "Save states y rebobinado", f2_d: "Varias ranuras, avance rápido y rebobinado mientras mantienes una tecla o botón.",
      f3_t: "Grabación de GIF y MP4", f3_d: "Captura pantallas, GIFs y vídeos MP4 (el MP4 necesita ffmpeg).",
      f4_t: "Depurador y trucos", f4_d: "Visores de CPU, memoria y tiles, y un gestor de trucos.",
      f5_t: "Core libretro", f5_d: "Juega dentro de RetroArch en Linux, Windows y FreeBSD.",
     
      shots_title: "Capturas de pantalla",
      s1: "Biblioteca en modo sofá", s2: "Juego con marco de TV", s4: "Biblioteca con imágenes de juego",
      s5: "Ficha del juego", s6: "Menú rápido", s7: "Trucos", s8: "Depurador: CPU", s9: "Depurador: memoria", s11: "Biblioteca en lista", s10: "Depurador: tiles",
      cl0: "La primera versión: los 13 cartuchos y 26 homebrews, validados contra el core de MiSTer, para Linux, Windows, FreeBSD y RetroArch.", cl_all: "Changelog completo (en inglés) →",
      cl1: "Vista de lista en la biblioteca: título, año, desarrolladora, editora, jugadores, tiempo de juego y última partida; haz clic en un encabezado para ordenar.",
      cl2: "Tiempo de juego e historial de cada juego.",
      cl3: "Carátulas un 20% más grandes, repartidas por todo el ancho de la ventana.",
      cl4: "Tus carátulas propias también en la ficha del juego y en el marco de carátula.",
      cl5: "Informe de errores: si el programa falla, el siguiente inicio muestra dónde está el informe.",
      cl6: "La ventana nunca se abre más grande que la pantalla, y copiar texto funciona.",
      cl7: "Se quitó el marco Consola: el dibujo no se parecía al PV-1000 real.",
      cl8: "Menú rápido más estrecho, con las ayudas de botones en dos líneas.",
      dl_title: "Descargar", dl_sub: "Versión 0.9.1.",
      dl_btn: "Todas las versiones y checksums",
      foot_lic: "Software libre bajo GPL-3.0 o posterior.",
    }
  };

  var nodes = document.querySelectorAll("[data-i18n], [data-i18n-html]");
  var orig = {};
  nodes.forEach(function (n) {
    var k = n.getAttribute("data-i18n") || n.getAttribute("data-i18n-html");
    orig[k] = n.hasAttribute("data-i18n-html") ? n.innerHTML : n.textContent;
  });

  var phNodes = document.querySelectorAll("[data-i18n-ph]");
  phNodes.forEach(function (n) { orig["ph:" + n.getAttribute("data-i18n-ph")] = n.placeholder; });

  function apply(lang) {
    phNodes.forEach(function (n) {
      var k = n.getAttribute("data-i18n-ph");
      n.placeholder = lang !== "en" && T[lang] && T[lang][k] ? T[lang][k] : orig["ph:" + k];
    });
    nodes.forEach(function (n) {
      var html = n.hasAttribute("data-i18n-html");
      var k = n.getAttribute(html ? "data-i18n-html" : "data-i18n");
      var v = lang === "en" || !T[lang] || !T[lang][k] ? orig[k] : T[lang][k];
      if (html) n.innerHTML = v; else n.textContent = v;
    });
    document.documentElement.lang = lang === "pt" ? "pt-BR" : lang;
    document.querySelectorAll(".lang button").forEach(function (b) {
      b.setAttribute("aria-pressed", b.dataset.lang === lang ? "true" : "false");
    });
    try { localStorage.setItem("lang", lang); } catch (e) {}
  }

  document.querySelectorAll(".lang button").forEach(function (b) {
    b.addEventListener("click", function () { apply(b.dataset.lang); });
  });

  var lang = "en";
  try { lang = localStorage.getItem("lang") || ""; } catch (e) {}
  if (!lang) {
    var nav = (navigator.language || "en").slice(0, 2);
    lang = T[nav] ? nav : "en";
  }
  apply(lang);
})();
