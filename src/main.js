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
window.addEventListener("DOMContentLoaded", async () => {
  let wow = await invoke("on_window_open");
  console.dir(wow);
  document.body.style.backgroundColor = wow.background_color;
  document.body.style.fontSize = wow.font_size;
  document.body.style.fontFamily = wow.font;
  document.body.style.color = wow.font_color;

  input_div = document.getElementById("input");
  text_div = document.getElementById("text");
});

document.addEventListener("keydown", async (e) => {
  switch (e.key) {
    case "Backspace":
      input_div.textContent = input_div.textContent.slice(0, -1);
      return;
    case "Enter":
      send_command(input_div.textContent);
      input_div.textContent = "";
      return;
    case "f":
      if (e.ctrlKey) {
        e.preventDefault();
        let results = await invoke("fuzzy_find_history", { query: input_div.textContent });
        console.dir(results);
        if (results.length > 0){
          input_div.textContent = results[0];
        } else {
          console.log("No results found");
          input_div.textContent = "";
        }
        return;
      }
      break;
    case "c":
      if (e.ctrlKey) {
        // send SIGINT
        invoke("send_command", { command: "\x03" });
        input_div.textContent = "";
      }
      e.preventDefault();
      break;
    case "d":
      if (e.ctrlKey) {
        // send EOF
        invoke("send_command", { command: "\x04" });
        input_div.textContent = "";
      }
      break;
    default:
      break;
  }
  if (e.key.length === 1) {
    input_div.textContent += e.key;
    console.log(input_div.textContent)
  }
});

// Listener for terminal output
listen('output', (event) => {
  let content = event.payload;
  text_div.innerHTML += content;
  window.scrollTo(0, document.body.scrollHeight);
})
