/* eslint-disable */
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

export type AdminChantier = {
  __typename?: 'AdminChantier';
  dateDebut?: Maybe<Scalars['Date']['output']>;
  finition: AdminFinition;
  idChantier: Scalars['UUID']['output'];
  typeChantier: AdminTypeChantier;
};

export type AdminChantierMutations = {
  __typename?: 'AdminChantierMutations';
  upsert: AdminChantier;
  upsertBatch: Array<AdminChantier>;
};


export type AdminChantierMutationsUpsertArgs = {
  input: ChantierInput;
};


export type AdminChantierMutationsUpsertBatchArgs = {
  input: Array<ChantierInput>;
};

export type AdminChantierQuery = {
  __typename?: 'AdminChantierQuery';
  list: AdminChantierResults;
  unique: AdminChantier;
};


export type AdminChantierQueryListArgs = {
  input?: OffsetLimit;
};


export type AdminChantierQueryUniqueArgs = {
  id: Scalars['UUID']['input'];
};

export type AdminChantierResults = {
  __typename?: 'AdminChantierResults';
  data: Array<AdminChantier>;
  limit: Scalars['Int']['output'];
  offset: Scalars['Int']['output'];
  total: Scalars['Int']['output'];
};

export type AdminClient = {
  __typename?: 'AdminClient';
  telephone: Scalars['String']['output'];
};

export type AdminClientMutations = {
  __typename?: 'AdminClientMutations';
  insert: AdminClient;
  insertBatch: Array<AdminClient>;
};


export type AdminClientMutationsInsertArgs = {
  input: ClientInput;
};


export type AdminClientMutationsInsertBatchArgs = {
  input: Array<ClientInput>;
};

export type AdminClientQuery = {
  __typename?: 'AdminClientQuery';
  list: AdminClientResults;
  unique: AdminClient;
};


export type AdminClientQueryListArgs = {
  input?: OffsetLimit;
};


export type AdminClientQueryUniqueArgs = {
  telephone: Scalars['String']['input'];
};

export type AdminClientResults = {
  __typename?: 'AdminClientResults';
  data: Array<AdminClient>;
  limit: Scalars['Int']['output'];
  offset: Scalars['Int']['output'];
  total: Scalars['Int']['output'];
};

export type AdminDevis = {
  __typename?: 'AdminDevis';
  idDevis: Scalars['UUID']['output'];
  materiel: AdminMateriel;
  quantite: Scalars['BigDecimal']['output'];
  typeChantier: AdminTypeChantier;
};

export type AdminDevisMutations = {
  __typename?: 'AdminDevisMutations';
  upsert: AdminDevis;
  upsertBatch: Array<AdminDevis>;
};


export type AdminDevisMutationsUpsertArgs = {
  input: DevisInput;
};


export type AdminDevisMutationsUpsertBatchArgs = {
  input: Array<DevisInput>;
};

export type AdminDevisQuery = {
  __typename?: 'AdminDevisQuery';
  list: AdminDevisResults;
  unique: AdminDevis;
};


export type AdminDevisQueryListArgs = {
  input?: OffsetLimit;
};


export type AdminDevisQueryUniqueArgs = {
  id: Scalars['UUID']['input'];
};

export type AdminDevisResults = {
  __typename?: 'AdminDevisResults';
  data: Array<AdminDevis>;
  limit: Scalars['Int']['output'];
  offset: Scalars['Int']['output'];
  total: Scalars['Int']['output'];
};

export type AdminFinition = {
  __typename?: 'AdminFinition';
  designation: Scalars['String']['output'];
  duree: Scalars['BigDecimal']['output'];
  idFinition: Scalars['UUID']['output'];
  isStandard?: Maybe<Scalars['Boolean']['output']>;
  prix: Scalars['BigDecimal']['output'];
};

export type AdminFinitionMutations = {
  __typename?: 'AdminFinitionMutations';
  upsert: AdminFinition;
  upsertBatch: Array<AdminFinition>;
};


export type AdminFinitionMutationsUpsertArgs = {
  input: FinitionInput;
};


export type AdminFinitionMutationsUpsertBatchArgs = {
  input: Array<FinitionInput>;
};

export type AdminFinitionQuery = {
  __typename?: 'AdminFinitionQuery';
  list: AdminFinitionResults;
  unique: AdminFinition;
};


export type AdminFinitionQueryListArgs = {
  input?: OffsetLimit;
};


export type AdminFinitionQueryUniqueArgs = {
  id: Scalars['UUID']['input'];
};

export type AdminFinitionResults = {
  __typename?: 'AdminFinitionResults';
  data: Array<AdminFinition>;
  limit: Scalars['Int']['output'];
  offset: Scalars['Int']['output'];
  total: Scalars['Int']['output'];
};

export type AdminMateriel = {
  __typename?: 'AdminMateriel';
  code: Scalars['String']['output'];
  designation: Scalars['String']['output'];
  prixUnitaire: Scalars['BigDecimal']['output'];
  unite: AdminUnite;
};

export type AdminMaterielMutations = {
  __typename?: 'AdminMaterielMutations';
  upsert: AdminMateriel;
  upsertBatch: Array<AdminMateriel>;
};


export type AdminMaterielMutationsUpsertArgs = {
  input: MaterielInput;
};


export type AdminMaterielMutationsUpsertBatchArgs = {
  input: Array<MaterielInput>;
};

export type AdminMaterielQuery = {
  __typename?: 'AdminMaterielQuery';
  list: AdminMaterielResults;
  unique: AdminMateriel;
};


export type AdminMaterielQueryListArgs = {
  input?: OffsetLimit;
};


export type AdminMaterielQueryUniqueArgs = {
  code: Scalars['String']['input'];
};

export type AdminMaterielResults = {
  __typename?: 'AdminMaterielResults';
  data: Array<AdminMateriel>;
  limit: Scalars['Int']['output'];
  offset: Scalars['Int']['output'];
  total: Scalars['Int']['output'];
};

export type AdminMutation = {
  __typename?: 'AdminMutation';
  chantier: AdminChantierMutations;
  clients: AdminClientMutations;
  devis: AdminDevisMutations;
  finition: AdminFinitionMutations;
  materiel: AdminMaterielMutations;
  resetDb: Scalars['Boolean']['output'];
  typeChantier: AdminTypeChantierMutations;
  unite: AdminUniteMutations;
};

export type AdminQuery = {
  __typename?: 'AdminQuery';
  chantier: AdminChantierQuery;
  clients: AdminClientQuery;
  devis: AdminDevisQuery;
  finition: AdminFinitionQuery;
  hello: Scalars['String']['output'];
  materiels: AdminMaterielQuery;
  typeChantier: AdminTypeChantierQuery;
  unite: AdminUniteQuery;
};

export type AdminTypeChantier = {
  __typename?: 'AdminTypeChantier';
  description: Scalars['String']['output'];
  duree: Scalars['BigDecimal']['output'];
  idTypeChantier: Scalars['UUID']['output'];
  nom: Scalars['String']['output'];
  prixTotal: Scalars['BigDecimal']['output'];
};

export type AdminTypeChantierMutations = {
  __typename?: 'AdminTypeChantierMutations';
  upsert: AdminTypeChantier;
  upsertBatch: Array<AdminTypeChantier>;
};


export type AdminTypeChantierMutationsUpsertArgs = {
  input: TypeChantierInput;
};


export type AdminTypeChantierMutationsUpsertBatchArgs = {
  input: Array<TypeChantierInput>;
};

export type AdminTypeChantierQuery = {
  __typename?: 'AdminTypeChantierQuery';
  list: AdminTypeChantierResults;
  unique: AdminTypeChantier;
};


export type AdminTypeChantierQueryListArgs = {
  input?: OffsetLimit;
};


export type AdminTypeChantierQueryUniqueArgs = {
  id: Scalars['UUID']['input'];
};

export type AdminTypeChantierResults = {
  __typename?: 'AdminTypeChantierResults';
  data: Array<AdminTypeChantier>;
  limit: Scalars['Int']['output'];
  offset: Scalars['Int']['output'];
  total: Scalars['Int']['output'];
};

export type AdminUnite = {
  __typename?: 'AdminUnite';
  designation: Scalars['String']['output'];
  idUnite: Scalars['UUID']['output'];
};

export type AdminUniteMutations = {
  __typename?: 'AdminUniteMutations';
  upsert: AdminUnite;
  upsertBatch: Array<AdminUnite>;
};


export type AdminUniteMutationsUpsertArgs = {
  input: UniteInput;
};


export type AdminUniteMutationsUpsertBatchArgs = {
  input: Array<UniteInput>;
};

export type AdminUniteQuery = {
  __typename?: 'AdminUniteQuery';
  list: AdminUniteResults;
  unique: AdminUnite;
};


export type AdminUniteQueryListArgs = {
  input?: OffsetLimit;
};


export type AdminUniteQueryUniqueArgs = {
  id: Scalars['UUID']['input'];
};

export type AdminUniteResults = {
  __typename?: 'AdminUniteResults';
  data: Array<AdminUnite>;
  limit: Scalars['Int']['output'];
  offset: Scalars['Int']['output'];
  total: Scalars['Int']['output'];
};

export type ChantierInput = {
  client: Scalars['String']['input'];
  dateDebut?: InputMaybe<Scalars['Date']['input']>;
  id?: InputMaybe<Scalars['UUID']['input']>;
  typeChantierId: Scalars['UUID']['input'];
  typeFinitionId: Scalars['UUID']['input'];
};

export type ClientInput = {
  telephone: Scalars['String']['input'];
};

export type DevisInput = {
  id?: InputMaybe<Scalars['UUID']['input']>;
  materielId: Scalars['String']['input'];
  quantite: Scalars['BigDecimal']['input'];
  typeChantierId: Scalars['UUID']['input'];
};

export type FinitionInput = {
  designation: Scalars['String']['input'];
  duree: Scalars['BigDecimal']['input'];
  id?: InputMaybe<Scalars['UUID']['input']>;
  isStandard?: InputMaybe<Scalars['Boolean']['input']>;
  prix: Scalars['BigDecimal']['input'];
};

export type MaterielInput = {
  code: Scalars['String']['input'];
  designation: Scalars['String']['input'];
  prixUnitaire: Scalars['BigDecimal']['input'];
  uniteId: Scalars['UUID']['input'];
};

export type OffsetLimit = {
  limit: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
};

export type TypeChantierInput = {
  description: Scalars['String']['input'];
  duree: Scalars['BigDecimal']['input'];
  id?: InputMaybe<Scalars['UUID']['input']>;
  nom: Scalars['String']['input'];
};

export type UniteInput = {
  designation: Scalars['String']['input'];
  id?: InputMaybe<Scalars['UUID']['input']>;
};
