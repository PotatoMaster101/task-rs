# Task App
Small task app in Rust to play around with actix web.

**WARNING**: This project is for demo only. Do not deploy this onto the internet, as the configs are insecurly configured.

## Build & Run Backend
1. Create a tenant on [auth0](https://auth0.com/)
2. Update `.env.sample` to the tenant
3. Run below
```shell
cp .env.sample .env
docker compose up -d --build
docker exec -it backend sqlx database setup --database-url "postgres://postgres:P@ssw0rd123!!!@postgres"

# or migrate on local machine:
sqlx database setup --database-url "postgres://postgres:P@ssw0rd123!!!@localhost"
```

## Build & Run Frontend
```shell
npm run dev --prefix ./frontend
```

## Usage
### Task List
Create a task list:
```shell
curl -X POST 'localhost:8080/api/task-lists' -H 'Content-Type: application/json' -d '{"title": "test"}'
```

Get a task list:
```shell
curl -X GET 'localhost:8080/api/task-lists/00000000-0000-0000-0000-000000000000'
```

Paginate task lists:
```shell
curl -X GET 'localhost:8080/api/task-lists?last=00000000-0000-0000-0000-000000000000&count=100'
```

Update a task list:
```shell
curl -X PUT 'localhost:8080/api/task-lists/00000000-0000-0000-0000-000000000000' -H 'Content-Type: application/json' -d '{"title": "updated"}'
```

Delete a task list:
```shell
curl -X DELETE 'localhost:8080/api/task-lists/00000000-0000-0000-0000-000000000000'
```

### Task
Create a task:
```shell
curl -X POST 'localhost:8080/api/tasks' -H 'Content-Type: application/json' -d '{"list_id": "00000000-0000-0000-0000-000000000000", "title": "test", "description": "desc", "done": false}'
```

Get a task:
```shell
curl -X GET 'localhost:8080/api/tasks/00000000-0000-0000-0000-000000000000'
```

Paginate tasks:
```shell
curl -X GET 'localhost:8080/api/tasks?last=00000000-0000-0000-0000-000000000000&count=100&list_id=00000000-0000-0000-0000-000000000000'
```

Update a task:
```shell
curl -X PUT 'localhost:8080/api/tasks/00000000-0000-0000-0000-000000000000' -H 'Content-Type: application/json' -d '{"title": "updated", "description": "desc", "done": false}'
```

Delete a task:
```shell
curl -X DELETE 'localhost:8080/api/tasks/00000000-0000-0000-0000-000000000000'
```

### User
Login user:
```shell
curl -X GET 'localhost:8080/api/user'
```

## TODO
- [ ] Fix frontend + docker
- [ ] Mem cache JWKS
- [ ] Finish auth
- [ ] HTTPS proxy
