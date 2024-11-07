# https://thecrunch.space/blog/optimizing-nuxt-3-deployment-with-bun-a-docker-based-approach/
# also with pnpm support
FROM oven/bun:latest as base
WORKDIR /usr/src/app

# Install pnpm
RUN npm install -g pnpm

# Install dependencies into a temp directory
FROM base AS install
RUN mkdir -p /temp/dev
COPY package.json /temp/dev/
RUN cd /temp/dev && pnpm install

# Install production dependencies
RUN mkdir -p /temp/prod
COPY package.json /temp/prod/
RUN cd /temp/prod && pnpm install --prod

# Build stage
FROM base AS prerelease
COPY --from=install /temp/dev/node_modules /usr/src/app/node_modules
COPY . /usr/src/app

# Set NODE_ENV to production and run the build
ENV NODE_ENV=production
RUN bun --bun run build

# Final stage
FROM base AS release
WORKDIR /usr/src/app
COPY --from=install /temp/prod/node_modules /usr/src/app/node_modules
COPY --from=prerelease /usr/src/app/.output /usr/src/app/.output
COPY package.json /usr/src/app/

# Ensure the container runs as a non-root user
USER bun

# Expose the port your app runs on
EXPOSE 3000/tcp

# Get Host from environment variable
ENV NUXT_HOST=0.0.0.0

# Run with bun
ENTRYPOINT ["sh", "-c", "bun run /usr/src/app/.output/server/index.mjs --host $NUXT_HOST"]