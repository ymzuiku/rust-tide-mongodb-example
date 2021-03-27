function query({ url = "", method, body }) {
  const list = url.split("/");
  let id = list[list.length - 1];
  id = id.split("?")[0];

  fetch(url, {
    method: method || "POST",
    body: body ? JSON.stringify(body) : void 0,
  })
    .then((v) => v.json())
    .then((res) => {
      const ele = document.getElementById(id);
      ele.textContent = JSON.stringify(res);
    })
    .catch((err) => {
      const ele = document.getElementById(id);
      ele.textContent = err.toString();
      ele.style.border = "1px solid #333";
    });
}
