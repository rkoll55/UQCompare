# Stage 1: Build the React application
FROM node:22-alpine as builder

WORKDIR /app

# Install build dependencies
COPY . .

# If you're using npm or yarn, adjust the command accordingly
RUN npm install -g pnpm@latest && pnpm install --frozen-lockfile

# Build the application
RUN pnpm run build || true

# Stage 2: Serve the application with Nginx
FROM nginx:alpine

# Copy the built assets from the builder stage
COPY --from=builder /app/dist /usr/share/nginx/html

# Overwrite the default nginx conf
COPY nginx.conf /etc/nginx/nginx.conf

# Expose port 8081
EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]
