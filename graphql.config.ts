import { IGraphQLConfig } from "graphql-config";

const config: IGraphQLConfig = {
  projects: {
    admin: {
      documents: [
        "./src/lib/admin/**/*.svelte",
        "./src/lib/admin/**/*.ts",
        "./src/routes/admin/**/*.svelte",
      ],
      schema: "./src/schemas/admin.graphqls",
      includes: ["./src/lib/admin/**/*", "./src/routes/admin/**/*"],
    },
    frontOffice: {
      documents: [
        "./src/lib/client/**/*.svelte",
        "./src/lib/client/**/*.ts",
        "./src/routes/client/**/*.svelte",
      ],
      schema: "./src/schemas/frontOffice.graphqls",
      includes: ["./src/lib/client/**/*", "./src/routes/client/**/*"],
    },
  },
};

export default config;
