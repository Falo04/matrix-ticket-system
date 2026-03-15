import * as React from "react";
import { Outlet, createRootRoute, ErrorComponentProps } from "@tanstack/react-router";
import { ErrorContext } from "src/context/error-context";
import { useTranslation } from "react-i18next";
import { Heading } from "src/components/ui/heading";
import { Button } from "src/components/ui/button";
import { Text } from "src/components/ui/text";

/**
 * The root error component
 */
function ErrorComponent(props: ErrorComponentProps) {
    return (
        <div className={"flex h-screen w-full items-center justify-center"}>
            <div
                className={
                    "min-w-sm flex max-w-xl flex-col gap-6 rounded-lg border border-zinc-300 bg-white p-12 dark:border-zinc-800 dark:bg-zinc-900"
                }
            >
                <Heading>{props.error.toString()}</Heading>
                <Text>{props.info?.componentStack}</Text>

                <Button className={"w-full"} onClick={() => props.reset()}>
                    Try again
                </Button>

                <Button onClick={() => history.back()}>Back</Button>
            </div>
        </div>
    );
}

export const Route = createRootRoute({
    component: () => (
        <>
            <ErrorContext />
            <Outlet />
        </>
    ),
    errorComponent: (err) => <ErrorComponent {...err} />,
});
