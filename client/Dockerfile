ARG BUILD=production
FROM node:16.13.1-alpine3.14 as base

ENV PORT=3333
ENV ADDR=0.0.0.0

WORKDIR /app

COPY . .

RUN npm i -g npm pnpm && pnpm install

FROM base as production-build

RUN pnpm run build

FROM base as development-build

FROM ${BUILD}-build as build

EXPOSE ${PORT}

CMD ["pnpm", "run", "preview"]

