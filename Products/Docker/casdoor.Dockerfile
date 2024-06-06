FROM casbin/casdoor:latest

WORKDIR /var/lib/casdoor

RUN mkdir db
RUN touch db/casdoor.db

WORKDIR /

EXPOSE 8000
