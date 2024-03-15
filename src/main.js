const { listen } = window.__TAURI__.event;
const { invoke } = window.__TAURI__.tauri;

let input_div, text_div;
// Send command to rust pty
async function send_command(e) {
  console.log("Sending command", e);
  await invoke("send_command", { command: e }).catch((e) => {
    console.error(e);
  });
}

// Client input handling
window.addEventListener("DOMContentLoaded", () => {
  invoke("on_window_open");
  input_div = document.getElementById("input");
  text_div = document.getElementById("text");
});

document.addEventListener("keydown", (e) => {
  if (e.key.length === 1) {
    input_div.textContent += e.key;
    console.log(input_div.textContent)
  } else {
    switch (e.key) {
      case "Backspace":
        input_div.textContent = input_div.textContent.slice(0, -1);
        break;
      case "Enter":
        send_command(input_div.textContent);
        input_div.textContent = "";
        break;
    }
  }
});

// Listener for terminal output
listen('output', (event) => {
  let content = event.payload;
  text_div.innerHTML += content;
  window.scrollTo(0, document.body.scrollHeight);
})
