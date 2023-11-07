CREATE DATABASE arcane_vault;

go 

use arcane_vault

CREATE TABLE adventurers (
    adventurer_id  INT PRIMARY KEY,
    name NVARCHAR(20),
    surname NVARCHAR(30),
    level TINYINT CHECK (level >= 1 AND level <=20),
);

CREATE TABLE facilities (
    facility_id INT PRIMARY KEY,
    facility_address NVARCHAR(75),
);

CREATE TABLE civilizations(
    civilization_name NVARCHAR(50) PRIMARY KEY,
    race NVARCHAR(30),
    period INT,
    religion NVARCHAR(30),
);

CREATE TABLE destinations(
    destination_id INT PRIMARY KEY,
    name NVARCHAR(75),
    coordinates NVARCHAR(20),
    difficulty TINYINT CHECK (difficulty >= 1 AND difficulty <= 20),
    civilization NVARCHAR(50),
    FOREIGN KEY (civilization) REFERENCES civilizations(civilization_name) ON DELETE CASCADE,
);

CREATE TABLE adventures(
    adventure_id INT PRIMARY KEY,
    facility_id INT,
    destination_id INT,
    date DATE,
    deadline DATE,
    status NVARCHAR(30) CHECK (status IN ('planned', 'in progress', 'finished', 'failed')),
    cost INT,
    FOREIGN KEY (facility_id) REFERENCES facilities(facility_id) ON DELETE CASCADE,
    FOREIGN KEY (destination_id) REFERENCES destinations(destination_id) ON DELETE CASCADE,
);

CREATE TABLE artifacts(
    artifact_id INT PRIMARY KEY,
    adventure_id INT,
    status NVARCHAR(30) CHECK (status IN ('Unidentfied', 'Identified', 'Sold', 'Stolen', 'Broken')),
    kind NVARCHAR(50),
    FOREIGN KEY (adventure_id) REFERENCES adventures(adventure_id) ON DELETE CASCADE,
);

CREATE TABLE participations(
    adventurer_id INT,
    adventure_id INT,
    FOREIGN KEY (adventurer_id) REFERENCES adventurers(adventurer_id) ON DELETE CASCADE,
    FOREIGN KEY (adventure_id) REFERENCES adventures(adventure_id) ON DELETE CASCADE,
    PRIMARY KEY (adventurer_id, adventure_id),
);



