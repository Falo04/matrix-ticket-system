/** To have a specific type to check for */
export class ResponseError<T = unknown> extends Error {
    override name = "ResponseError" as const;

    constructor(
        public response: Response,
        public data?: T,
        msg = "Response returned an error code",
    ) {
        super(msg);
    }

    /**
     * Gets the status code of the ResponseError
     *
     * @returns Status code
     */
    get status() {
        return this.response.status;
    }
}

// NOTE: Supports cases where `content-type` is other than `json`
const getBody = <T>(c: Response | Request): Promise<T> => {
    const contentType = c.headers.get("content-type");

    if (contentType && contentType.includes("application/json")) {
        return c.json();
    }

    if (contentType && contentType.includes("application/pdf")) {
        return c.blob() as Promise<T>;
    }

    return c.text() as Promise<T>;
};

export const customFetch = async <T>(url: string, options: RequestInit): Promise<T> => {
    const requestUrl = new URL(url, window.location.origin);
    const requestInit: RequestInit = {
        ...options,
        headers: {
            ...options.headers,
        },
    };

    const response = await fetch(requestUrl.toString(), requestInit);
    if (response.ok) {
        return await getBody<T>(response);
    }
    throw new ResponseError(response, "Response returned an error code");
};
