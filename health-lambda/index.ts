import {EventBridgeEvent, EventBridgeHandler} from "aws-lambda";

interface Result {
    ok: boolean;
    status: number;
    body: string;
}

export const handler: EventBridgeHandler<any, any, Result> = async (event) => {
    const result = await fetch(process.env.HEALTH_URL!, {
        headers: {
            "Cache-Control": "no-cache",
            "Pragma": "no-cache",
            "User-Agent": "aws-lambda-health-check/1.0"
        },
        signal: AbortSignal.timeout(10000)
    });

    if (!result.ok) throw new Error(`Health check failed with status ${result.status}`);

    const body: { status: string } = await result.json();

    if (body.status === "degraded") return {
        ok: false,
        status: result.status,
        body: JSON.stringify(body),
    };

    return {
        ok: true,
        status: result.status,
        body: JSON.stringify(body),
    };
}