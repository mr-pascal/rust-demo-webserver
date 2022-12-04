APP=server2
docker build -t abszissex/$APP -f ../Dockerfile --build-arg APP=$APP .
