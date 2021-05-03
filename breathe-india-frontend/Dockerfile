FROM node:16.0.0-alpine3.13 
WORKDIR app
COPY package.json .
COPY yarn.lock .
RUN yarn
COPY . .
RUN yarn build
ENTRYPOINT ["./docker-entrypoint.sh"]
