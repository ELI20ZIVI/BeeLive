FROM casbin/casdoor:latest

WORKDIR /var/lib/casdoor

RUN mkdir db
RUN touch db/casdoor.db

EXPOSE 8000
