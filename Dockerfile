FROM debian:buster-slim

RUN apt-get update && apt-get install -yy \
    nginx \
    procps \
    curl

ADD demo/nginx.yml /etc/cerberus.yml
ADD demo demo
ADD target/debug/cerberus /usr/local/bin/cerberus

ENV USER=root

CMD ["cerberus"]
