# Build app
FROM node:13.11.0 AS build

WORKDIR /tmp/app/

COPY package* .

RUN npm ci

COPY . .

RUN npm run build

# Build nginx image
FROM nginx:alpine
LABEL app.name=app

RUN rm -v /etc/nginx/nginx.conf

COPY misc/nginx.conf /etc/nginx/
COPY misc/configuration.json /var/www/.well-known/
COPY --from=build /tmp/bundle /var/www/

CMD ["nginx", "-g", "daemon off;"]
