/**
 * Our app's state object
 * Contains our WebSocket and a method to refresh it
 */
class AppState {
	/**
	 * @type {WebSocket}
	 */
	ws = createWebSocket(this);

	refreshSocket() {
		this.ws = createWebSocket(this);
	}
}

/**
 * An event to be sent to the server
 */
class ClientEventMessage {
	/**
	 * @param {string} eventType - The event's type
	 * @param {string} [clickedKey] - Key that was clicked (optional)
	 * @param {string[]} [modifiers] - An array of active modifier keys (optional)
	 */
	constructor(eventType, clickedKey, modifiers) {
		this.eventType = eventType;
		this.clickedKey = clickedKey;
		this.modifiers = modifiers;
	}

	/**
	 * Method for JSON.stringify()
	 * @returns {string} JSON stringified EventMessage
	 */
	toJSONString() {
		return JSON.stringify(this);
	}
}

/**
 * Creates an auto retrying WebSocket tied to current AppState
 * It's got recursion that I hope the GC can follow :D
 * @param {AppState} state - AppState in need of a new WebSocket
 * @returns {WebSocket} New WebSocket with methods assigned
 */
function createWebSocket(state) {
	const ws = new WebSocket(`ws://${window.location.host}/ws`);

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
			const clickedKey = button.dataset.value;
			const msg = new ClientEventMessage("click", clickedKey);
			console.log(msg.toJSONString());
			state.ws.send(msg.toJSONString());
		});
	});

	// TODO: Add modifier key logic

	/**
	 * The joys of using an HTML form cause it feels semantic
	 * But not wanting anything it provides
	 */
	const form = document.getElementById("form");
	form.addEventListener("submit", (event) => event.preventDefault());
})();
