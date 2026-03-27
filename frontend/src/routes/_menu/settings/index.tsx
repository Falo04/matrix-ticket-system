import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { useForm } from "@tanstack/react-form";
import i18n from "i18next";
import { toast } from "react-toastify";
import { Api } from "src/api/api";
import HeadingLayout from "src/components/layouts/heading-layout";
import { Form, Input } from "src/components/ui/input";
import { Field, FieldGroup, FieldLabel, FieldSet } from "src/components/ui/field";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "src/components/ui/select";
import { Separator } from "src/components/ui/separator";
import { Button } from "src/components/ui/button";
import { ErrorMessage } from "src/components/ui/text";
import ACCOUNT_CONTEXT from "src/context/account";
import React from "react";

/**
 * The properties for {@link Settings}
 */
export type SettingsProps = object;

/**
 * Settings component
 */
export function Settings() {
    const [t] = useTranslation("settings");

    const { account, reset } = React.useContext(ACCOUNT_CONTEXT);
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

    const matrixForm = useForm({
        defaultValues: {
            matrix_user_id: account.matrix_id ?? "",
        },
        validators: {
            onSubmitAsync: async ({ value }) => {
                if (!value.matrix_user_id.includes(":")) {
                    return {
                        fields: {
                            matrix_user_id: t("error.id-not-correct-format"),
                        },
                    };
                }

                await Api.account.updateMatrixId(value.matrix_user_id);
                reset();
                toast.success(t("toast.user-id-success"));
            },
        },
    });

    const logout = async () => {
        await Api.oidc.logout();
        await navigate({ to: "/" });
    };

    return (
        <div className={"flex flex-col gap-4"}>
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
                    <div className={"flex justify-between"}>
                        <Button type={"button"} variant={"ghost"} onClick={() => logout()}>
                            {t("button.logout")}
                        </Button>
                        <Button type={"submit"} className={"mr-4"}>
                            {t("button.update")}
                        </Button>
                    </div>
                </Form>
            </HeadingLayout>
            <HeadingLayout heading={t("heading.matrix-setting")} description={"description.matrix-setting"}>
                <Form onSubmit={matrixForm.handleSubmit}>
                    <FieldSet>
                        <FieldGroup>
                            <matrixForm.Field name={"matrix_user_id"}>
                                {(fieldApi) => (
                                    <Field className={"grid grid-cols-3 gap-3 sm:col-span-2"}>
                                        <FieldLabel>{t("label.matrix-user-id")}</FieldLabel>
                                        <Input
                                            placeholder={t("placeholder.matrix-user-id")}
                                            value={fieldApi.state.value}
                                            required={true}
                                            onChange={(e) => fieldApi.handleChange(e.target.value)}
                                            aria-invalid={fieldApi.state.meta.errors.length !== 0}
                                        />
                                        {fieldApi.state.meta.errors.map((err) => (
                                            <ErrorMessage key={err}>{err}</ErrorMessage>
                                        ))}
                                    </Field>
                                )}
                            </matrixForm.Field>
                        </FieldGroup>
                    </FieldSet>
                    <Separator />
                    <div className={"flex justify-between"}>
                        <Button type={"submit"} className={"mr-4"}>
                            {t("button.update")}
                        </Button>
                    </div>
                </Form>
            </HeadingLayout>
        </div>
    );
}

export const Route = createFileRoute("/_menu/settings/")({
    component: Settings,
});
