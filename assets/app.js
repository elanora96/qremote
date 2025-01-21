class AppState {
  ws = createWebSocket(this);

  refreshSocket() {
    this.ws = createWebSocket(this);
  }
}

/**
 * @param {AppState} state
 */
function createWebSocket(state) {
  const ws = new WebSocket(window.location.href);

  Object.assign(ws, {
    onopen: () => {
      console.log("WebSocket connection established");
    },

    onmessage: (event) => {
      console.log(`Message from server: ${event.data}`);
    },

    onclose: () => {
      console.warn("WebSocket connection closed. Attempting to reconnect...");

      setTimeout(() => state.refreshSocket(), 1000);
    },

    onerror: (error) => {
      console.error(`WebSocket Error: ${error}`);
      ws.close();
    },
  });

  return ws;
}

(() => {
  const state = new AppState();
  /**
   * @type {NodeListOf<HTMLButtonElement>}
   */
  const buttons = document.querySelectorAll("button[data-value]");

  buttons.forEach((button) => {
    button.addEventListener("click", (_) => {
      const pressedKey = button.dataset.value;
      state.ws.send(pressedKey);
    });
  });
})();
