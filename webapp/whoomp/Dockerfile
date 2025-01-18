# -----------------------------------------
# 1) BUILD STAGE
# -----------------------------------------
FROM node:18-alpine AS build

WORKDIR /app

COPY package*.json ./
COPY tsconfig*.json ./
COPY vite.config.* ./
COPY svelte.config.js ./
COPY tailwind.config.js ./
COPY postcss.config.js ./

RUN npm install

COPY . .

RUN npm run build  # => /app/dist

# -----------------------------------------
# 2) PRODUCTION STAGE
# -----------------------------------------
FROM nginx:alpine

# Copy compiled front-end
COPY --from=build /app/dist /usr/share/nginx/html

# Copy custom NGINX config
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
