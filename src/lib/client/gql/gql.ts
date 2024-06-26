/* eslint-disable */
import * as types from './graphql';
import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 */
const documents = {
    "\n    query clientNewTypeChantier($page: OffsetLimit) {\n      typeChantier {\n        list(page: $page) {\n          data {\n            duree\n            description\n            idTypeChantier\n            nom\n            prixTotal\n          }\n          total\n          offset\n          limit\n        }\n      }\n    }\n  ": types.ClientNewTypeChantierDocument,
    "\n    query clientListChantier($page: OffsetLimit!) {\n      chantiers {\n        list(pagination: $page) {\n          data {\n            id\n            dateDebut\n            finition {\n              duree\n              designation\n              prix\n            }\n            typeChantier {\n              nom\n            }\n            devis {\n              prixTotal\n            }\n          }\n          offset\n          total\n          limit\n        }\n      }\n    }\n  ": types.ClientListChantierDocument,
    "\n  mutation clientLogin($telephone: String!) {\n    login(telephone: $telephone)\n  }\n": types.ClientLoginDocument,
};

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = graphql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
 */
export function graphql(source: string): unknown;

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n    query clientNewTypeChantier($page: OffsetLimit) {\n      typeChantier {\n        list(page: $page) {\n          data {\n            duree\n            description\n            idTypeChantier\n            nom\n            prixTotal\n          }\n          total\n          offset\n          limit\n        }\n      }\n    }\n  "): (typeof documents)["\n    query clientNewTypeChantier($page: OffsetLimit) {\n      typeChantier {\n        list(page: $page) {\n          data {\n            duree\n            description\n            idTypeChantier\n            nom\n            prixTotal\n          }\n          total\n          offset\n          limit\n        }\n      }\n    }\n  "];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n    query clientListChantier($page: OffsetLimit!) {\n      chantiers {\n        list(pagination: $page) {\n          data {\n            id\n            dateDebut\n            finition {\n              duree\n              designation\n              prix\n            }\n            typeChantier {\n              nom\n            }\n            devis {\n              prixTotal\n            }\n          }\n          offset\n          total\n          limit\n        }\n      }\n    }\n  "): (typeof documents)["\n    query clientListChantier($page: OffsetLimit!) {\n      chantiers {\n        list(pagination: $page) {\n          data {\n            id\n            dateDebut\n            finition {\n              duree\n              designation\n              prix\n            }\n            typeChantier {\n              nom\n            }\n            devis {\n              prixTotal\n            }\n          }\n          offset\n          total\n          limit\n        }\n      }\n    }\n  "];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation clientLogin($telephone: String!) {\n    login(telephone: $telephone)\n  }\n"): (typeof documents)["\n  mutation clientLogin($telephone: String!) {\n    login(telephone: $telephone)\n  }\n"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;