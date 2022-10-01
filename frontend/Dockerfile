# Build app
FROM node:16.16.0 AS build

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
COPY misc/.well-known/ /var/www/
COPY --from=build /tmp/app/dist /var/www/

CMD ["nginx", "-g", "daemon off;"]
