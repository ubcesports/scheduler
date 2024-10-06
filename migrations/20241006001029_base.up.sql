CREATE TABLE schedule
(
    id        BIGINT PRIMARY KEY NOT NULL,
    parent_id BIGINT REFERENCES schedule (id) DEFAULT NULL
);

CREATE TABLE schedule_assignment
(
    schedule_id BIGINT REFERENCES schedule (id) NOT NULL,
    slot_id     BIGINT REFERENCES slot (id)     NOT NULL,
    subject_id  BIGINT REFERENCES subject (id)  NOT NULL,

    PRIMARY KEY (schedule_id, slot_id, subject_id)
);

CREATE TABLE subject
(
    id     BIGINT PRIMARY KEY NOT NULL,
    w2m_id INT UNIQUE DEFAULT NULL,

    name   TEXT               NOT NULL
);

CREATE TABLE slot
(
    id     BIGINT PRIMARY KEY NOT NULL,
    w2m_id INT UNIQUE DEFAULT NULL
);

CREATE TABLE availability
(
    id         BIGINT PRIMARY KEY NOT NULL,
    created_at INTEGER DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE availability_entry
(
    availability_id BIGINT REFERENCES availability (id) NOT NULL,
    slot_id         BIGINT REFERENCES slot (id)         NOT NULL,
    subject_id      BIGINT REFERENCES subject (id)      NOT NULL,

    PRIMARY KEY (availability_id, slot_id, subject_id)
);

CREATE TABLE parameters
(
    lock         INT PRIMARY KEY NOT NULL,

    version      INT,

    availability BIGINT REFERENCES availability (id),
    schedule     BIGINT REFERENCES schedule (id),

    CONSTRAINT parameters_lock CHECK ( lock == 1 )
);

INSERT INTO parameters (lock, version)
    VALUES (1, 1);