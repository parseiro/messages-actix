CREATE TABLE users (
    id serial PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    phonenumber VARCHAR (30) NOT NULL,
    verified boolean NOT NULL,
    created_at timestamp NOT NULL,
    senha VARCHAR(30) NOT NULL
)