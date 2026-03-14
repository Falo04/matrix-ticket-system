import { defineConfig } from "orval";

export default defineConfig({
    api: {
        input: {
            target: "http://webserver:8080/openapi.json",
        },
        output: {
            mode: "single",
            workspace: "src/api/generated/",
            target: "./index.ts",
            schemas: "./models",
            clean: true,
            client: "fetch",
            prettier: true,
            fileExtension: ".gen.ts",
            baseUrl: {
                getBaseUrlFromSpecification: true,
                variables: {
                    environment: "api.dev",
                },
            },
        },
        hooks: {
            afterAllFilesWrite: "prettier --write",
        },
    },
});
