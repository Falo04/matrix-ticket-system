import React from "react";
import { clsx } from "clsx";

export function Text({ className, ...props }: React.ComponentPropsWithoutRef<"p">) {
    return (
        <p data-slot="text" {...props} className={clsx(className, "text-muted-foreground text-sm/6 sm:text-base/6")} />
    );
}

export function ErrorMessage({ className, ...props }: React.ComponentPropsWithoutRef<"p">) {
    return (
        <p data-slot="text" {...props} className={clsx(className, "text-destructive-foreground text-nowrap text-sm")} />
    );
}
