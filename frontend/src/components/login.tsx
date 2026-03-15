import { useTranslation } from "react-i18next";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { Soup } from "lucide-react";
import { ButtonLink } from "src/components/ui/button";

/**
 * The properties for {@link Login}
 */
export type LoginProps = {};

/**
 * The Login view
 */
export function Login() {
    const [t] = useTranslation("login");

    return (
        <div className="flex min-h-svh w-full items-center justify-center p-6 md:p-10">
            <div className="w-full max-w-sm">
                <div className="flex flex-col gap-6">
                    <Card>
                        <CardHeader>
                            <div className={"flex items-center justify-center gap-4"}>
                                <div className="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg">
                                    <Soup className="size-4" />
                                </div>
                                <CardTitle className="text-2xl">{t("login.title")}</CardTitle>
                            </div>
                        </CardHeader>
                        <CardContent className={"mt-4"}>
                            <ButtonLink className={"w-full"} href={"/api/v1/oidc/begin-login"}>
                                {t("button.sign-in-with-sso")}
                            </ButtonLink>
                        </CardContent>
                    </Card>
                </div>
            </div>
        </div>
    );
}
