import type { CodegenConfig } from "@graphql-codegen/cli";

const config: CodegenConfig = {
  schema: "./src/schemas/frontOffice.graphqls",
  documents: [
    "src/lib/client/**/*.svelte",
    "src/lib/client/**/*.ts",
    "src/routes/client/**/*.svelte",
    "src/routes/client/**/*.ts",
  ],
  ignoreNoDocuments: false, // for better experience with the watcher
  generates: {
    "./src/lib/client/gql/": {
      preset: "client",
      config: {
        useTypeImports: true,
      },
      plugins: [],
    },
  },
};

export default config;
