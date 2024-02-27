import ky from "ky";
import { stringify } from "postcss";

const url = "http://localhost:7000";

interface Result {
	rate: number;
}

export const getHeartbeat = async () => {
	try {
		// return { "rate": number }
		const json = await ky.get(url).json<Result>();
		return json.rate.toString();
	} catch (e) {
		console.error(e);
		return "∞";
	}
};
