FROM node:20-alpine
WORKDIR /app

EXPOSE 80
EXPOSE 443

COPY . .
RUN npm ci
RUN npm run build

CMD ["node", "-r", "dotenv/config", "build" ]