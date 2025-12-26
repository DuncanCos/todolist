import axios from "axios";

const api_url = import.meta.env.VITE_BACKEND_URL;

// Crée une instance Axios avec une URL de base
const api = axios.create({
  baseURL: api_url,
});

export default api;
