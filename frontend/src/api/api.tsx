import { ERROR_STORE } from "src/context/error-context";
import { ResponseError } from "src/api/custom-fetch";
import { getMe, logout } from "src/api/generated";

/** Hyphen separated uuid */
export type UUID = string;

export const Api = {
    account: {
        getMe: () => handleError(getMe()),
    },
    oidc: {
        logout: () => handleError(logout()),
    },
};

/**
 * Wraps a promise returned by the generated SDK which handles its errors and returns a {@link Result}
 *
 * @param promise The promise to wrap. This should be a promise defined in the generated part of the API
 *
 * @returns a new promise with a result that wraps errors from the API
 */
export async function handleError<T>(promise: Promise<T>): Promise<T> {
    try {
        return await promise;
    } catch (e) {
        let msg;
        if (e instanceof ResponseError) {
            msg = e;
        } else {
            // eslint-disable-next-line no-console
            console.error("Unknown error occurred:", e);
            msg = "Unknown error occurred";
        }
        ERROR_STORE.report(msg);
        throw msg;
    }
}
