-- migrate:up
CREATE TABLE fixture.person (
    id         BIGINT NOT NULL,
    login_fold TEXT NOT NULL,
    CONSTRAINT pk_person PRIMARY KEY (id),
    CONSTRAINT uq_person_login UNIQUE (login_fold)
);

-- migrate:down
DROP TABLE fixture.person;
