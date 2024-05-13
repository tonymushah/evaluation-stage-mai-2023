-- Your SQL goes here
create table materiels (
    code varchar(25) PRIMARY KEY,
    designation text not null,
    prix_unitaire decimal,
    unite_id UUID NOT NULL REFERENCES unite(id_unite)
);