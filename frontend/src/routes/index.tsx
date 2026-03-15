import { createFileRoute, Navigate } from "@tanstack/react-router";
import { AccountProvider } from "src/context/account";

export const Route = createFileRoute("/")({
    component: () => (
        <AccountProvider>
            <Navigate to={"/settings"} />
        </AccountProvider>
    ),
});
