FROM debian:buster-slim

RUN apt-get update && apt-get install -yy \
    nginx \
    psutils

ADD demo.yml /etc/cerberus.yml
ADD target/debug/cerberus /usr/local/bin/cerberus

ENV USER=root

CMD ["cerberus"]
