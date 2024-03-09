# Otus Highload

<h2>Otus Highload Homework</h2>

Simple user authentication app in Rust.

<h3>BUILD FOR DEBUG</h3>

We're using cargo watch with mirroring files you have locally to the container on debug. So changes that you make locally appear in the running container. 

In order to start the container, run the following command:

```
docker compose up --build
```

<h3>BUILD FOR RELEASE</h3>

To build and run the release version of the container (in case you don't wanna have a 2GB sized image locally and just need to check things out in prod mode): 

```
docker compose -f docker-compose.prod.yml build
docker compose -f docker-compose.prod.yml up
```

<h3>ACCESS DB</h3>

From the docker container: docker exec -it otus-highload-db psql -U user -d otus_highload

PREPARE DB -- not needed now, cause we don't use compile-time checked queries. May need some day.

cargo sqlx prepare -D postgresql://user:password@localhost:5432/otus_highload