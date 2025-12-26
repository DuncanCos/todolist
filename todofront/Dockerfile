# ÉTAPE 1 : Build (Node.js)
FROM node:20-alpine as builder

WORKDIR /app

# On copie d'abord les fichiers de dépendances pour profiter du cache Docker
COPY package*.json ./

# Installation propre des dépendances (npm ci est plus fiable que npm install pour les builds)
RUN npm ci

# Copie du reste du code source
COPY . .

# Construction de l'app (génère le dossier /dist)
RUN npm run build

# ÉTAPE 2 : Serveur (Nginx)
FROM nginx:alpine

# Copie des fichiers buildés depuis l'étape précédente vers le dossier par défaut de Nginx
COPY --from=builder /app/dist /usr/share/nginx/html

# Copie de notre configuration Nginx personnalisée
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Exposition du port 80
EXPOSE 80

# Démarrage de Nginx
CMD ["nginx", "-g", "daemon off;"]