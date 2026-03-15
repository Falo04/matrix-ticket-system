import type React from "react";
import { SidebarInset, SidebarProvider } from "src/components/ui/sidebar";

/**
 * The BaseLayout
 */
export function BaseLayout({
    sidebar,
    header,
    children,
}: React.PropsWithChildren<{
    /** Sidebar */
    sidebar: React.ReactNode;
    /** Header */
    header: React.ReactNode;
}>) {
    return (
        <SidebarProvider>
            {sidebar}

            <SidebarInset>
                <div className={"flex flex-1 flex-col"}>
                    <div className="@container/main relative flex flex-1 flex-col gap-2">
                        <div className={"bg-background sticky top-0 z-40 rounded-full px-4 py-2"}>{header}</div>
                        <div className={"flex h-full w-full flex-1 flex-col px-4"}>{children}</div>
                    </div>
                </div>
            </SidebarInset>
        </SidebarProvider>
    );
}
