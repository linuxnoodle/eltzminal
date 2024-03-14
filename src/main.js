const { listen } = window.__TAURI__.event;
const { invoke } = window.__TAURI__.tauri;

let text_div;
// Send command to rust pty
async function send_command(e) {
  console.log("Sending command", e);
  await invoke("send_command", { command: e });
}

// Client input handling
window.addEventListener("DOMContentLoaded", () => {
  text_div = document.getElementById("text");
  document.onkeydown = (e) => {
    if (e.key.length === 1) {
        text_div.textContent += e.key;
    } else {
      switch (e.key) {
        case "Backspace":
          text_div.textContent = text_div.textContent.slice(0, -1);
          break;
        case "Enter":
          send_command(text_div.textContent);
          text_div.textContent = "";
          break;
        }
    }
  };
});

// Listener for terminal output
listen('output', (event) => {
  console.log("Received output", event.payload);
  text_div.textContent += event.payload;
})
