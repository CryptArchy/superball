FROM buildpack-deps:jessie
MAINTAINER Chris Andrews <candrews@luc.id>

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app
COPY ./target/release /usr/src/app
COPY ./pub /usr/src/app
COPY log4rs.toml /usr/src/app

EXPOSE 8080

CMD [ "/usr/src/app/superball" ]
