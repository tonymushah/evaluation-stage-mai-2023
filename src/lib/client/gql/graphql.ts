/* eslint-disable */
import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
  BigDecimal: { input: any; output: any; }
  /**
   * ISO 8601 calendar date without timezone.
   * Format: %Y-%m-%d
   *
   * # Examples
   *
   * * `1994-11-13`
   * * `2000-02-24`
   */
  Date: { input: any; output: any; }
  /**
   * A UUID is a unique 128-bit number, stored as 16 octets. UUIDs are parsed as
   * Strings within GraphQL. UUIDs are used to assign unique identifiers to
   * entities without requiring a central allocating authority.
   *
   * # References
   *
   * * [Wikipedia: Universally Unique Identifier](http://en.wikipedia.org/wiki/Universally_unique_identifier)
   * * [RFC4122: A Universally Unique IDentifier (UUID) URN Namespace](http://tools.ietf.org/html/rfc4122)
   */
  UUID: { input: any; output: any; }
};

export type ClientChantier = {
  __typename?: 'ClientChantier';
  dateDebut?: Maybe<Scalars['Date']['output']>;
  devis: ClientDevis;
  finition: Finition;
  id: Scalars['UUID']['output'];
  typeChantier: TypeChantier;
};

export type ClientChantierQueries = {
  __typename?: 'ClientChantierQueries';
  list: ClientChantierResults;
};


export type ClientChantierQueriesListArgs = {
  pagination?: OffsetLimit;
};

export type ClientChantierResults = {
  __typename?: 'ClientChantierResults';
  data: Array<ClientChantier>;
  limit: Scalars['Int']['output'];
  offset: Scalars['Int']['output'];
  total: Scalars['Int']['output'];
};

export type ClientCreateChantier = {
  dateDebut?: InputMaybe<Scalars['Date']['input']>;
  typeChantier: Scalars['UUID']['input'];
  typeFinitionId?: InputMaybe<Scalars['UUID']['input']>;
};

export type ClientDevis = {
  __typename?: 'ClientDevis';
  items: Array<ClientDevisItem>;
  prixTotal?: Maybe<Scalars['BigDecimal']['output']>;
};

export type ClientDevisItem = {
  __typename?: 'ClientDevisItem';
  id: Scalars['UUID']['output'];
  materiel: Materiel;
  prixTotal: Scalars['BigDecimal']['output'];
  quantite: Scalars['BigDecimal']['output'];
  unite: Unite;
};

export type ClientFinitionQueries = {
  __typename?: 'ClientFinitionQueries';
  getDefault: Finition;
  list: FinitionResults;
};


export type ClientFinitionQueriesListArgs = {
  page?: OffsetLimit;
};

export type ClientTypeChantierQueries = {
  __typename?: 'ClientTypeChantierQueries';
  list: TypeChantierResults;
};


export type ClientTypeChantierQueriesListArgs = {
  page?: OffsetLimit;
};

export type Finition = {
  __typename?: 'Finition';
  designation: Scalars['String']['output'];
  duree: Scalars['BigDecimal']['output'];
  idFinition: Scalars['UUID']['output'];
  isStandard?: Maybe<Scalars['Boolean']['output']>;
  prix: Scalars['BigDecimal']['output'];
};

export type FinitionResults = {
  __typename?: 'FinitionResults';
  data: Array<Finition>;
  limit: Scalars['Int']['output'];
  offset: Scalars['Int']['output'];
  total: Scalars['Int']['output'];
};

export type FrontOfficeMutation = {
  __typename?: 'FrontOfficeMutation';
  createChantier: ClientChantier;
  login: Scalars['String']['output'];
};


export type FrontOfficeMutationCreateChantierArgs = {
  input: ClientCreateChantier;
};


export type FrontOfficeMutationLoginArgs = {
  telephone: Scalars['String']['input'];
};

export type FrontOfficeQuery = {
  __typename?: 'FrontOfficeQuery';
  chantiers: ClientChantierQueries;
  finitions: ClientFinitionQueries;
  hello: Scalars['String']['output'];
  typeChantier: ClientTypeChantierQueries;
};

export type Materiel = {
  __typename?: 'Materiel';
  code: Scalars['String']['output'];
  designation: Scalars['String']['output'];
  prixUnitaire: Scalars['BigDecimal']['output'];
};

export type OffsetLimit = {
  limit: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
};

export type TypeChantier = {
  __typename?: 'TypeChantier';
  description: Scalars['String']['output'];
  duree: Scalars['BigDecimal']['output'];
  idTypeChantier: Scalars['UUID']['output'];
  nom: Scalars['String']['output'];
  prixTotal: Scalars['BigDecimal']['output'];
};

export type TypeChantierResults = {
  __typename?: 'TypeChantierResults';
  data: Array<TypeChantier>;
  limit: Scalars['Int']['output'];
  offset: Scalars['Int']['output'];
  total: Scalars['Int']['output'];
};

export type Unite = {
  __typename?: 'Unite';
  designation: Scalars['String']['output'];
  idUnite: Scalars['UUID']['output'];
};

export type ClientNewTypeChantierQueryVariables = Exact<{
  page?: InputMaybe<OffsetLimit>;
}>;


export type ClientNewTypeChantierQuery = { __typename?: 'FrontOfficeQuery', typeChantier: { __typename?: 'ClientTypeChantierQueries', list: { __typename?: 'TypeChantierResults', total: number, offset: number, limit: number, data: Array<{ __typename?: 'TypeChantier', duree: any, description: string, idTypeChantier: any, nom: string }> } } };

export type ClientListChantierQueryVariables = Exact<{
  page: OffsetLimit;
}>;


export type ClientListChantierQuery = { __typename?: 'FrontOfficeQuery', chantiers: { __typename?: 'ClientChantierQueries', list: { __typename?: 'ClientChantierResults', offset: number, total: number, limit: number, data: Array<{ __typename?: 'ClientChantier', id: any, dateDebut?: any | null, finition: { __typename?: 'Finition', duree: any, designation: string, prix: any }, typeChantier: { __typename?: 'TypeChantier', nom: string }, devis: { __typename?: 'ClientDevis', prixTotal?: any | null } }> } } };

export type ClientLoginMutationVariables = Exact<{
  telephone: Scalars['String']['input'];
}>;


export type ClientLoginMutation = { __typename?: 'FrontOfficeMutation', login: string };


export const ClientNewTypeChantierDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"clientNewTypeChantier"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"page"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"OffsetLimit"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"typeChantier"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"list"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"page"},"value":{"kind":"Variable","name":{"kind":"Name","value":"page"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"data"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"duree"}},{"kind":"Field","name":{"kind":"Name","value":"description"}},{"kind":"Field","name":{"kind":"Name","value":"idTypeChantier"}},{"kind":"Field","name":{"kind":"Name","value":"nom"}}]}},{"kind":"Field","name":{"kind":"Name","value":"total"}},{"kind":"Field","name":{"kind":"Name","value":"offset"}},{"kind":"Field","name":{"kind":"Name","value":"limit"}}]}}]}}]}}]} as unknown as DocumentNode<ClientNewTypeChantierQuery, ClientNewTypeChantierQueryVariables>;
export const ClientListChantierDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"clientListChantier"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"page"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"OffsetLimit"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"chantiers"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"list"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"pagination"},"value":{"kind":"Variable","name":{"kind":"Name","value":"page"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"data"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"dateDebut"}},{"kind":"Field","name":{"kind":"Name","value":"finition"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"duree"}},{"kind":"Field","name":{"kind":"Name","value":"designation"}},{"kind":"Field","name":{"kind":"Name","value":"prix"}}]}},{"kind":"Field","name":{"kind":"Name","value":"typeChantier"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"nom"}}]}},{"kind":"Field","name":{"kind":"Name","value":"devis"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"prixTotal"}}]}}]}},{"kind":"Field","name":{"kind":"Name","value":"offset"}},{"kind":"Field","name":{"kind":"Name","value":"total"}},{"kind":"Field","name":{"kind":"Name","value":"limit"}}]}}]}}]}}]} as unknown as DocumentNode<ClientListChantierQuery, ClientListChantierQueryVariables>;
export const ClientLoginDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"clientLogin"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"telephone"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"login"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"telephone"},"value":{"kind":"Variable","name":{"kind":"Name","value":"telephone"}}}]}]}}]} as unknown as DocumentNode<ClientLoginMutation, ClientLoginMutationVariables>;