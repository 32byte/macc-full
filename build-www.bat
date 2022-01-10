:: CACHEBUST is to always keep the git repo up to date
docker build --build-arg CACHEBUST=%RANDOM% -t 32byte/macc-www .
pause