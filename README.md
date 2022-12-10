start
```
cp .env.example .env
docker compose up --build
```

access

frontend:
[localhost:8080](http://localhost:8080/)

api:
[localhost:8000](http://localhost:8000/)

db:
psql -h $(hostname -i) -p 5436 -d changethis -U changethis
