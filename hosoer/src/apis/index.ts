import ky from "ky";

const url = "https://api.heartrate.nyaw.xyz";

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
