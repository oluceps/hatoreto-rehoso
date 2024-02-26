import ky from 'ky';

const url = 'http://localhost:7000';

interface Result {
    rate: number;
}

export const getHeartbeat = async () => {
    // return { "rate": number }
    const json = await ky.get(url).json<Result>();
    return json.rate;
};
