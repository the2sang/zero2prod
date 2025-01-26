-- Add migration script here
CREATE TABLE subscriptions(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email TEXT NOT NULL,
    name TEXT NOT NULL,
    subscribe_at timestamptz NOT NULL
);