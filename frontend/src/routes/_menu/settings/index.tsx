import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { useForm } from "@tanstack/react-form";
import i18n from "i18next";
import { toast } from "react-toastify";
import { Api } from "src/api/api";
import HeadingLayout from "src/components/layouts/heading-layout";
import { Form } from "src/components/ui/input";
import { Field, FieldGroup, FieldLabel, FieldSet } from "src/components/ui/field";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "src/components/ui/select";
import { Separator } from "src/components/ui/separator";
import { Button } from "src/components/ui/button";

/**
 * The properties for {@link Settings}
 */
export type SettingsProps = object;

/**
 * Settings component
 */
export function Settings() {
    const [t] = useTranslation("settings");

    const navigate = useNavigate();

    const form = useForm({
        defaultValues: {
            language: i18n.language,
            appearance: localStorage.getItem("theme") ?? "dark",
        },
        onSubmit: ({ value }) => {
            i18n.changeLanguage(value.language.toLowerCase()).then();
            if (value.appearance === "light") {
                localStorage.setItem("theme", "light");
            } else {
                localStorage.setItem("theme", "dark");
            }

            if (
                localStorage.theme === "dark" ||
                (!("theme" in localStorage) && window.matchMedia("(prefers-color-scheme: dark").matches)
            ) {
                document.documentElement.classList.add("dark");
            } else {
                document.documentElement.classList.remove("dark");
            }
            toast.success(t("toast.success"));
        },
    });

    const logout = async () => {
        await Api.oidc.logout();
        await navigate({ to: "/" });
    };

    return (
        <HeadingLayout heading={t("heading.title")} description={t("heading.description")}>
            <Form onSubmit={form.handleSubmit}>
                <FieldSet>
                    <FieldGroup>
                        <form.Field name="language">
                            {(field) => (
                                <Field className={"grid grid-cols-3 gap-3 sm:col-span-2"}>
                                    <FieldLabel htmlFor="language">{t("label.language")}</FieldLabel>
                                    <Select
                                        value={field.state.value}
                                        onValueChange={(e) => (e ? field.handleChange(e) : undefined)}
                                    >
                                        <SelectTrigger id={"language"} className={"col-span-2 w-full"}>
                                            <SelectValue />
                                        </SelectTrigger>
                                        <SelectContent>
                                            <SelectItem value={"de"} key={"de"}>
                                                {t("label.de")}
                                            </SelectItem>
                                            <SelectItem value={"en"} key={"en"}>
                                                {t("label.en")}
                                            </SelectItem>
                                        </SelectContent>
                                    </Select>
                                </Field>
                            )}
                        </form.Field>
                        <form.Field name="appearance">
                            {(field) => (
                                <Field className={"grid grid-cols-3 gap-3 sm:col-span-2"}>
                                    <FieldLabel htmlFor="appearance">{t("label.appearance")}</FieldLabel>
                                    <Select
                                        value={field.state.value}
                                        onValueChange={(e) => (e ? field.handleChange(e) : undefined)}
                                    >
                                        <SelectTrigger id={"appearance"} className={"col-span-2 w-full"}>
                                            <SelectValue />
                                        </SelectTrigger>
                                        <SelectContent>
                                            <SelectItem value={"dark"} key={"dark"}>
                                                {t("label.dark")}
                                            </SelectItem>
                                            <SelectItem value={"light"} key={"light"}>
                                                {t("label.light")}
                                            </SelectItem>
                                        </SelectContent>
                                    </Select>
                                </Field>
                            )}
                        </form.Field>
                    </FieldGroup>
                </FieldSet>
                <Separator />
                <div className={"flex justify-between pt-4"}>
                    <Button type={"button"} variant={"ghost"} onClick={() => logout()}>
                        {t("button.logout")}
                    </Button>
                    <Button type={"submit"} className={"mr-4"}>
                        {t("button.update")}
                    </Button>
                </div>
            </Form>
        </HeadingLayout>
    );
}

export const Route = createFileRoute("/_menu/settings/")({
    component: Settings,
});
