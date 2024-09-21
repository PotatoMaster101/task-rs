CREATE SCHEMA "app";

CREATE TABLE "app"."role" (
    "id" uuid PRIMARY KEY,
    "name" varchar(1024) NOT NULL UNIQUE
);

CREATE TABLE "app"."user" (
    "id" uuid PRIMARY KEY,
    "role_id" uuid REFERENCES "app"."role"(id) NOT NULL,
    "created_at" timestamp WITH TIME ZONE NOT NULL DEFAULT now(),
    "updated_at" timestamp WITH TIME ZONE NOT NULL DEFAULT now(),
    "email" varchar(1024) NOT NULL UNIQUE,
    "auth0_id" varchar(1024) NOT NULL UNIQUE
);

CREATE TABLE "app"."task_list" (
    "id" uuid PRIMARY KEY,
    "user_id" uuid REFERENCES "app"."user"(id) NOT NULL,
    "created_at" timestamp WITH TIME ZONE NOT NULL DEFAULT now(),
    "updated_at" timestamp WITH TIME ZONE NOT NULL DEFAULT now(),
    "title" varchar(1024) NOT NULL
);

CREATE TABLE "app"."task" (
    "id" uuid PRIMARY KEY,
    "list_id" uuid REFERENCES "app"."task_list"(id) NOT NULL,
    "user_id" uuid REFERENCES "app"."user"(id) NOT NULL,
    "created_at" timestamp WITH TIME ZONE NOT NULL DEFAULT now(),
    "updated_at" timestamp WITH TIME ZONE NOT NULL DEFAULT now(),
    "title" varchar(1024) NOT NULL,
    "description" text,
    "done" bool NOT NULL DEFAULT false
);

INSERT INTO "app"."role" VALUES ('79c63692-a983-45bf-8e28-21ed0dc84b19', 'Admin');
INSERT INTO "app"."role" VALUES ('853e018a-07fe-4b1c-863f-5203d5589cab', 'User');
INSERT INTO "app"."user" VALUES ('aa56057c-e07a-400c-9da0-4534a8d7465c', '79c63692-a983-45bf-8e28-21ed0dc84b19', DEFAULT, DEFAULT, 'local@admin.com', 'local');
