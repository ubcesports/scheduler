CREATE TABLE schedule
(
    id        UUID PRIMARY KEY NOT NULL,
    parent_id UUID REFERENCES schedule (id) DEFAULT NULL
);

CREATE TABLE subject
(
    id     UUID    PRIMARY KEY NOT NULL,
    w2m_id INTEGER UNIQUE DEFAULT NULL,

    name   TEXT NOT NULL
);

CREATE TABLE slot
(
    id     UUID PRIMARY KEY NOT NULL,
    w2m_id INT  UNIQUE DEFAULT NULL
);

CREATE TABLE schedule_assignment
(
    schedule_id UUID REFERENCES schedule (id) NOT NULL,
    slot_id     UUID REFERENCES slot (id)     NOT NULL,
    subject_id  UUID REFERENCES subject (id)  NOT NULL,

    PRIMARY KEY (schedule_id, slot_id, subject_id)
);

CREATE TABLE availability
(
    id         UUID        PRIMARY KEY NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE availability_entry
(
    availability_id UUID REFERENCES availability (id) NOT NULL,
    slot_id         UUID REFERENCES slot (id)         NOT NULL,
    subject_id      UUID REFERENCES subject (id)      NOT NULL,

    PRIMARY KEY (availability_id, slot_id, subject_id)
);

CREATE TABLE parameters
(
    lock         INTEGER PRIMARY KEY NOT NULL GENERATED ALWAYS AS (1) STORED UNIQUE,
    version      INTEGER NOT NULL DEFAULT 1,

    availability UUID REFERENCES availability (id),
    schedule     UUID REFERENCES schedule (id),

    CONSTRAINT parameters_lock CHECK (lock = 1)
);

INSERT INTO parameters DEFAULT VALUES;
