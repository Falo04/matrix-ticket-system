import * as React from "react";
import { Outlet, createRootRoute } from "@tanstack/react-router";

function RootComponent() {
    return (
        <React.Fragment>
            <div>Hello "__root"!</div>
            <Outlet />
        </React.Fragment>
    );
}

export const Route = createRootRoute({
    component: RootComponent,
});
