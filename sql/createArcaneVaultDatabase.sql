CREATE DATABASE arcane_vault;

go 

use arcane_vault

CREATE TABLE travelers (
    traveler_id  INT IDENTITY(1,1) PRIMARY KEY,
    first_name NVARCHAR(20),
    last_name NVARCHAR(30),
    level TINYINT CHECK (level >= 1 AND level <=20),
);

CREATE TABLE facilities (
    facility_id INT IDENTITY(1,1) PRIMARY KEY,
    facility_address NVARCHAR(75),
);

CREATE TABLE civilizations(
    civilization_name NVARCHAR(50) PRIMARY KEY,
    race NVARCHAR(30),
    period INT,
    religion NVARCHAR(30),
);

CREATE TABLE expedition_destinations(
    place_id INT IDENTITY(1,1) PRIMARY KEY,
    place_name NVARCHAR(50),
    coordinates GEOGRAPHY,
    civilization_name NVARCHAR(50),
    FOREIGN KEY (civilization_name) REFERENCES civilizations(civilization_name) ON DELETE CASCADE,
);

CREATE TABLE expeditions(
    expedition_id INT IDENTITY(1,1) PRIMARY KEY,
    start_date DATE,
    deadline DATE,
    status NVARCHAR(30) CHECK (status IN ('planned', 'in progress', 'finished', 'failed')),
    expense INT,
    facility_id INT,
    FOREIGN KEY (facility_id) REFERENCES facilities(facility_id) ON DELETE CASCADE,
    place_id INT,
    FOREIGN KEY (place_id) REFERENCES expedition_destinations(place_id) ON DELETE CASCADE,
);

CREATE TABLE artifacts(
    artifact_id INT IDENTITY(1,1) PRIMARY KEY,
    status NVARCHAR(30) CHECK (status IN ('Unidentfied', 'Identified', 'Sold', 'Stolen', 'Broken')),
    type NVARCHAR(50),
    expedition_id INT,
    FOREIGN KEY (expedition_id) REFERENCES expeditions(expedition_id) ON DELETE CASCADE,
);

CREATE TABLE expedition_participations(
    traveler_id INT,
    expedition_id INT,
    FOREIGN KEY (traveler_id) REFERENCES travelers(traveler_id) ON DELETE CASCADE,
    FOREIGN KEY (expedition_id) REFERENCES expeditions(expedition_id) ON DELETE CASCADE,
    PRIMARY KEY (traveler_id, expedition_id),
);



