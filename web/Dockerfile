
FROM node:20-alpine AS builder
WORKDIR /app
COPY package*.json .
RUN yarn
COPY . .
RUN yarn run build

FROM node:20-alpine
WORKDIR /app
COPY --from=builder /app/build build/
COPY --from=builder /app/node_modules node_modules/
COPY --from=builder /app/.env .env
COPY package.json .
EXPOSE 3000
ENV NODE_ENV=production
CMD node -r dotenv/config build
