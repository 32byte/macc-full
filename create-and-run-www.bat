@echo off

:: load config
call config.bat

:: remove old container
echo removing old container...
docker container rm www

:: start docker container
echo starting container
docker run --sig-proxy=false -p %PORT%:%PORT% --name www 32byte/macc-www "bash" "-c" "export NODE_URL=%NODE_URL%; npm run build:webpack && cd dist && python3 -m http.server %PORT%"
pause