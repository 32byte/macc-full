# Building WWW using docker

## Requirements
[Install docker](https://docs.docker.com/engine/install/) on your platform


## Building
```cmd
in cmd:
.\build-www.bat
```


## Configuring
Edit config.bat:
- NODE_URL: is the url of your miner / node you're getting info from. Use `host.docker.internal:8033` if you want it to communicate with the miner running on your system, make sure the port is correct.
- PORT: the port to run the webserver on

If you make any changes to the config you will need to recreate the image with the `create-and-run-www.bat` script.


## Running
This will create a new container and run it
```cmd
in cmd:
.\create-and-run-www.bat
```
If you get an error that the container is already running, stop it with:
```cmd
docker kill www
```
If you already created a container and want to just start it again:
```cmd
docker start www
```