import { defineConfig } from "orval";

export default defineConfig({
    api: {
        input: {
            target: "http://webserver:8080/docs/openapi.json",
        },
        output: {
            mode: "single",
            workspace: "./src/api/generated",
            target: "./index.ts",
            clean: true,
            client: "fetch",
            prettier: true,
            override: {
                fetch: {
                    includeHttpResponseReturnType: false,
                },
                mutator: {
                    path: "../custom-fetch.ts",
                    name: "customFetch",
                },
            },
        },
        hooks: {
            afterAllFilesWrite: "prettier --write",
        },
    },
});
