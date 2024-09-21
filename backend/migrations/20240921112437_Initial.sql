CREATE SCHEMA "app";

CREATE TABLE "app"."user" (
    "id" uuid PRIMARY KEY,
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

CREATE INDEX "idx_task_list_user_id" ON "app"."task_list" ("user_id");
CREATE INDEX "idx_task_user_id" ON "app"."task" ("user_id");
CREATE INDEX "idx_task_list_id" ON "app"."task" ("list_id");
