# Otus Highload

<h2>Otus Highload Homework</h2>

Simple user authentication app. 

* Backend is written in Rust. 
* Frontend is potentially written with Vue3.

<h3>DEBUG</h3>

We're using cargo watch with mirroring files you have locally to the container on debug. So changes that you make locally appear in the running container. 

In order to start the container, run the following commands:

```
docker compose build
docker compose up
```

<h3>RELEASE</h3>

To build and run the release version of the container:

```
docker compose -f docker-compose.prod.yml build
docker compose -f docker-compose.prod.yml up
```

<h3>ACCESS DB</h3>

docker exec -it otus-highload-db psql -U user -d otus_highload


PREPARE DB

cargo sqlx prepare -D postgresql://user:password@localhost:5432/otus_highload