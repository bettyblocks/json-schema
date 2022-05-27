# Run image

FROM node:18.1.0-alpine

WORKDIR /usr/local/src
COPY . .

RUN rm -rf node_modules
RUN yarn --frozen-lockfile

CMD ["yarn", "start"]

