const chat = document.getElementById("chat");
const msgs = document.getElementById("msgs");
const presence = document.getElementById("presence-indicator");
let allChat = [];

// listen for events on the form
chat.addEventListener("submit", function (e) {
  e.preventDefault();
  postNewMsg(chat.elements.user.value, chat.elements.text.value);
  chat.elements.text.value = "";
});

async function postNewMsg(user, text) {
  const data = {
    user,
    text,
  };

  ws.send(JSON.stringify(data));
}

const ws = new WebSocket("ws://localhost:9001", ["json"]); // or ["json", "xml"]. Server can pick only one

ws.addEventListener("open", () => {
  console.log("connected!");
  presence.innerHTML = "🟢";
});

ws.addEventListener("close", () => {
  presence.innerText = "🔴";
});

ws.addEventListener("message", (event) => {
  console.log(event.data);
  const data = JSON.parse(event.data);
  render(data);
});

function render({ user, text }) {
  var message = document.createElement("div");
  message.innerHTML = template(user, text);
  msgs.append(message);
}

const template = (user, msg) =>
  `<li class="collection-item"><span class="badge">${user}</span>${msg}</li>`;
