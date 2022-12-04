# rust-demo-webserver


This repo contains two very basic web servers written in Rust.

Both webservers have a `GET /hello` endpoint.
`server1` is calling `server2` and pass through it's result to the callee.
`server2` is just returning a string


### Intention

The sole purpose of this repo is to have to services that communicate with each other
to have a test service e.g. in a Kubernetes setup where two services should work together.

### Commands

#### Start servers on local machine

```sh
cd server1
# Starts server1 on localhost:8080
./start.sh


cd server2
# Starts server2 on localhost:8081
./start.sh
```


#### Docker
```sh

# Build container
cd server1
docker build -t abszissex/server1 .

cd server2
docker build -t abszissex/server2 .

# Run containers
docker run --rm -p 8080:8080 --network="host" --name server1 abzissex/server1
docker run --rm -p 8080:8081 --network="host" --name server2 abzissex/server2

## Push containers to DockerHub
docker push abszissex/server1
docker push abszissex/server2

## Start and ssh into container
docker run -d --rm --name keep abszissex/server1 && docker exec -it keep /bin/sh
## Custom entry point
docker run -d --rm --name keep abszissex/server1 tail -f /dev/null && docker exec -it keep /bin/sh

```
