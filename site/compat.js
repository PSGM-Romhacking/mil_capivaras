(function () {
  var q = document.getElementById("compat-q");
  var rows = document.querySelectorAll("#compat-body tr[data-q]");
  var empty = document.getElementById("compat-empty");
  if (!q) return;
  q.addEventListener("input", function () {
    var t = q.value.trim().toLowerCase(), shown = 0;
    rows.forEach(function (r) {
      var ok = !t || r.dataset.q.indexOf(t) !== -1;
      r.hidden = !ok;
      if (ok) shown++;
    });
    empty.hidden = shown !== 0;
  });
})();
