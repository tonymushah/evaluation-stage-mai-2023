-- Your SQL goes here
create table finition(
    id_finition UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    designation TEXT NOT NULL,
    prix decimal NOT NULL,
    duree decimal NOT NULL,
    is_standard boolean
);
