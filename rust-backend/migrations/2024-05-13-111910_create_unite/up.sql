-- Your SQL goes here
create table unite(
    id_unite UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    designation TEXT NOT NULL
);
