FROM node:18.1.0-alpine

RUN apk update \
  && apk --no-cache --update add bash

WORKDIR /opt/app_dev

CMD ["bash", "-c", "./run.sh"]
