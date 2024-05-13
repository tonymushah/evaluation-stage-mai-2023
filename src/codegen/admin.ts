import type { CodegenConfig } from "@graphql-codegen/cli";

const config: CodegenConfig = {
  schema: "./src/schemas/admin.graphqls",
  documents: [
    "./src/lib/admin/**/*.svelte",
    "src/lib/admin/**/*.ts",
    "src/routes/admin/**/*.svelte",
    "src/routes/admin/**/*.ts",
  ],
  ignoreNoDocuments: true, // for better experience with the watcher
  generates: {
    "./src/lib/admin/gql/": {
      preset: "client",
      config: {
        useTypeImports: true,
      },
      plugins: [],
    },
  },
};

export default config;
