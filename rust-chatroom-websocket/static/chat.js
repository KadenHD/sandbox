let ws = null;
let connected = false;
let isClosing = false;

let usernameInput = document.getElementById("username");
let roomInput = document.getElementById("room");
let connectBtn = document.getElementById("connectBtn");
let messages = document.getElementById("messages");
let msgInput = document.getElementById("msgInput");
let sendBtn = document.getElementById("sendBtn");
let inputArea = document.getElementById("inputArea");

function scrollToBottom() {
  messages.scrollTop = messages.scrollHeight;
}

function setConnectedUI(state) {
  connected = state;
  connectBtn.textContent = state ? "Disconnect" : "Connect";
  inputArea.style.display = state ? "block" : "none";
  usernameInput.disabled = state;
  roomInput.disabled = state;
}

connectBtn.addEventListener("click", () => {
  if (!connected) {
    connect();
  } else {
    disconnect();
  }
});

function connect() {
  if (ws) return;

  const room = roomInput.value.trim();
  const username = usernameInput.value.trim();
  if (!room || !username) {
    alert("Enter room and username");
    return;
  }

  messages.innerHTML = "";
  isClosing = false;

  ws = new WebSocket(`ws://localhost:8080/ws/${room}/${username}`);

  ws.onopen = () => {
    setConnectedUI(true);
    addSystemMessage(`Connected to room "${room}" as "${username}"`);
  };

  ws.onmessage = (event) => {
    const msg = JSON.parse(event.data);

    if (msg.sender.toLowerCase() === "system") {
      addSystemMessage(msg.content);
    } else if (msg.sender === username) {
      addUserMessage(msg.sender, msg.content, true);
    } else {
      addUserMessage(msg.sender, msg.content, false);
    }
  };

  ws.onclose = () => {
    addSystemMessage("Disconnected from server");

    ws = null;
    setConnectedUI(false);
  };

  ws.onerror = () => {
    addSystemMessage("Connection error");
  };
}

function disconnect() {
  if (!ws) {
    setConnectedUI(false);
    return;
  }

  try {
    ws.close();
  } catch (e) {
    console.warn("Socket already closed");
  }

  ws = null;
  setConnectedUI(false);
}

sendBtn.addEventListener("click", sendMessage);

msgInput.addEventListener("keydown", (e) => {
  if (e.key === "Enter") {
    sendMessage();
  }
});

function sendMessage() {
  const text = msgInput.value.trim();
  if (!text || !ws || ws.readyState !== WebSocket.OPEN) return;

  ws.send(text);
  msgInput.value = "";
}

function addSystemMessage(text) {
  const li = document.createElement("li");
  li.textContent = text;
  li.className = "system";
  messages.appendChild(li);
  scrollToBottom();
}

function addUserMessage(sender, text, self = false) {
  const li = document.createElement("li");
  li.textContent = `${sender}: ${text}`;
  li.className = self ? "self" : "user";
  messages.appendChild(li);
  scrollToBottom();
}
