interface Result {
	rate: number;
}

let socket: WebSocket | null = null;

export const getHeartbeat = (onMessage: (data: string) => void) => {
	// Close the previous socket, if it exists
	if (socket !== null) {
		socket.close();
	}

	// Create a new WebSocket connection
	socket = new WebSocket("wss://api.heartrate.nyaw.xyz");

	// Set up the onmessage handler
	socket.onmessage = (event) => {
		const data: Result = JSON.parse(event.data);
		onMessage(data.rate.toString());
	};

	// Set up the onclose handler
	socket.onclose = () => {
		console.log("WebSocket closed");
		onMessage("~");
	};

	// Set up the onerror handler
	socket.onerror = (error) => {
		console.error(`WebSocket error: ${error}`);
	};
};
