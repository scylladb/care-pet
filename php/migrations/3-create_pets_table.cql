CREATE TABLE IF NOT EXISTS carepet.pet
(
    owner_id UUID,
    pet_id   UUID,
    chip_id  TEXT,
    species  TEXT,
    breed    TEXT,
    color    TEXT,
    gender   TEXT,
    age      INT,
    weight   FLOAT,
    address  TEXT,
    name     TEXT,
    PRIMARY KEY (owner_id, pet_id)
);