FROM postgres:latest

ENV POSTGRES_DB=rustify
ENV POSTGRES_USER=developer
ENV POSTGRES_PASSWORD=1357924680
ENV PGDATA=/var/lib/postgresql/data/pgdata

COPY ./init.sql /docker-entrypoint-initdb.d/

RUN chmod -R 0700 /var/lib/postgresql/data && chown -R postgres:postgres /var/lib/postgresql/data