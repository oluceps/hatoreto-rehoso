import { Frame } from "../grpc/rate";

let socket: WebSocket | null = null;

export const getHeartbeat = (onMessage: (data: string) => void) => {
	// Close the previous socket, if it exists
	if (socket !== null) {
		socket.close();
	}

	// Create a new WebSocket connection to local Rust backend
	socket = new WebSocket("ws://localhost:3010/ws");
	socket.binaryType = "arraybuffer";

	// Set up the onmessage handler
	socket.onmessage = (event) => {
		try {
			const buffer = new Uint8Array(event.data);
			const frame = Frame.decode(buffer);

			if (frame.rate) {
				onMessage(frame.rate.value.toString());
			} else if (frame.status) {
				console.log("Status update:", frame.status);
			}
		} catch (e) {
			console.error("Failed to decode frame", e);
		}
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
