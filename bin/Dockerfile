FROM ubuntu

EXPOSE 8033

# update system
RUN apt-get -y update && apt-get install -y

# install packages
RUN apt -y install wget

# download bin
RUN wget https://github.com/32byte/macc-full/releases/download/latest/macc-full_latest_x86_64-unknown-linux-musl.tar.gz -O macc.tar.gz

# upack
RUN tar -xvzf macc.tar.gz

CMD ["./macc", "run-node"]