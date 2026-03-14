import React, { useEffect } from "react";
import { useTranslation } from "react-i18next";

/**
 * Function which should trigger in case of error
 */
type ErrorListener = (error: unknown) => void;

/**
 * Handler for errors and has to be a singleton
 */
class ErrorStore {
    private listener: ErrorListener | null = null;

    /**
     * Register a new ErrorListener
     *
     * @param listener ErrorListener
     */
    subscribe(listener: ErrorListener) {
        if (!this.listener) {
            this.listener = listener;
        }
    }

    /**
     * Triggers for all listener the ErrorListener
     *
     * @param error error
     */
    report(error: unknown) {
        if (this.listener) {
            this.listener(error);
        }
    }

    /**
     * Triggers for all listener the ErrorListener
     *
     * @param error defined error enum
     */
    reportEnum(error: ErrorEnum) {
        if (this.listener) {
            this.listener(error);
        }
    }
}

export enum ErrorEnum {
    MissingPassword = "missing password",
    RequestingNewCredentialsFailed = "requesting new credentials failed",
    GettingCredentialsFailed = "getting credentials failed",
    Unknown = "unknown",
}

export const ERROR_STORE = new ErrorStore();

/**
 * The properties for {@link ErrorContext}
 */
export type ErrorContextProps = {};

/**
 * Error Container which stays in the root and has to be a singleton
 */
export function ErrorContext(props: ErrorContextProps) {
    const [t] = useTranslation("error-context");

    const [error, setError] = React.useState<unknown>(null);

    useEffect(() => {
        ERROR_STORE.subscribe(setError);
    }, []);

    if (error === "Unauthorized") {
        window.location.href = "/api/frontend/v1/oidc/begin-login";
        return;
    }

    if (typeof error === "string" && Object.values(ErrorEnum).includes(error as ErrorEnum)) {
        const enumErr = error as ErrorEnum;
        switch (enumErr) {
            case ErrorEnum.MissingPassword:
                throw new Error(t("error.missing-password"));
            case ErrorEnum.RequestingNewCredentialsFailed:
                throw new Error(t("error.requestion-new-credentials-failed"));
            case ErrorEnum.Unknown:
            default:
                throw new Error(t("error.unknown"));
        }
    }

    if (error) {
        throw error;
    }

    return undefined;
}
