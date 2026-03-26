import * as React from "react";
import { Input as InputPrimitive } from "@base-ui/react/input";
import { clsx } from "clsx";

/**
 * The properties for {@link Form}
 */
export type FormProps = {
    /** Class names */
    className?: string;

    /** The action that should run when pressing submitting the form */
    onSubmit: () => void;

    /** The child elements of the form */
    children?: React.ReactNode | Array<React.ReactNode>;
};

/**
 * A simple form to make declaring easier
 */
function Form(props: FormProps) {
    return (
        <form
            {...props}
            method={"post"}
            onSubmit={(e) => {
                e.preventDefault();
                props.onSubmit();
            }}
            className={clsx(props.className, "w-full space-y-8")}
        >
            {props.children}
        </form>
    );
}

function Input({ className, type, ...props }: React.ComponentProps<"input">) {
    return (
        <InputPrimitive
            type={type}
            data-slot="input"
            className={clsx(
                "border-input file:text-foreground placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 disabled:bg-input/50 aria-invalid:border-destructive aria-invalid:ring-3 aria-invalid:ring-destructive/20 dark:bg-input/30 dark:disabled:bg-input/80 dark:aria-invalid:border-destructive/50 dark:aria-invalid:ring-destructive/40 h-8 w-full min-w-0 rounded-lg border bg-transparent px-2.5 py-1 text-base outline-none transition-colors file:inline-flex file:h-6 file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
                className,
            )}
            {...props}
        />
    );
}

export { Input, Form };
