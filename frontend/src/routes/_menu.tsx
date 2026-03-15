import { createFileRoute, Outlet } from "@tanstack/react-router";
import React, { Suspense } from "react";
import { useTranslation } from "react-i18next";
import { AccountProvider } from "src/context/account";
import { BaseLayout } from "src/components/layouts/base-layout";
import {
    Sidebar,
    SidebarContent,
    SidebarGroup,
    SidebarGroupContent,
    SidebarGroupLabel,
    SidebarHeader,
    SidebarItem,
    SidebarMenu,
    SidebarMenuItem,
} from "src/components/ui/sidebar";
import { Heading, Subheading } from "src/components/ui/heading";
import { SettingsIcon } from "lucide-react";

/**
 * The properties for {@link TicketSystemMenu}
 */
export type TicketSystemMenuProps = {};

/**
 * Ticket system menu
 */
export default function TicketSystemMenu(props: TicketSystemMenuProps) {
    const [t] = useTranslation("menu");

    return (
        <BaseLayout
            sidebar={
                <Sidebar variant={"inset"}>
                    <SidebarHeader>
                        <SidebarMenu>
                            <SidebarMenuItem className={"flex items-center justify-center gap-2"}>
                                <Heading className="truncate font-semibold">{t("heading.app-title")}</Heading>
                            </SidebarMenuItem>
                        </SidebarMenu>
                    </SidebarHeader>
                    <SidebarContent>
                        <SidebarGroup>
                            <SidebarGroupLabel>{t("label.label")}</SidebarGroupLabel>
                            <SidebarGroupContent>
                                <SidebarMenu></SidebarMenu>
                            </SidebarGroupContent>
                        </SidebarGroup>
                        <SidebarGroup>
                            <SidebarGroupLabel>{t("label.personal")}</SidebarGroupLabel>
                            <SidebarGroupContent>
                                <SidebarMenu>
                                    <SidebarItem href={"/app/settings"}>
                                        <SettingsIcon className={"size-4"} />
                                        <Subheading>{t("label.settings")}</Subheading>
                                    </SidebarItem>
                                </SidebarMenu>
                            </SidebarGroupContent>
                        </SidebarGroup>
                    </SidebarContent>
                </Sidebar>
            }
            header={<div></div>}
        >
            <Suspense>
                <Outlet />
            </Suspense>
        </BaseLayout>
    );
}

export const Route = createFileRoute("/_menu")({
    component: () => (
        <AccountProvider>
            <TicketSystemMenu />
        </AccountProvider>
    ),
});
