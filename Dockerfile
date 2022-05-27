FROM node:18.1.0-alpine

WORKDIR /opt/app_dev

CMD ["bash", "-c", "./run.sh"]
