FROM node:20-bookworm-slim AS builder

WORKDIR /app

COPY package.json package-lock.json ./
RUN npm ci

COPY index.ts ./
RUN npx esbuild index.ts --bundle --minify --platform=node --target=node20 --outfile=index.js

FROM public.ecr.aws/lambda/nodejs:20

WORKDIR ${LAMBDA_TASK_ROOT}

COPY --from=builder /app/index.js ./

CMD ["index.handler"]
