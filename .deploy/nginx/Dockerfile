
FROM node:22 AS build
WORKDIR /app
COPY frontend ./frontend
WORKDIR /app/frontend
RUN npm install
RUN npm run build

FROM nginx:stable
COPY .deploy/nginx/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /app/frontend/dist/.vite /usr/share/vite
COPY --from=build /app/frontend/dist /usr/share/nginx/html
RUN rm /usr/share/nginx/html/index.html