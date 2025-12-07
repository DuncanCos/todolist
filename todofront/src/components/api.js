import axios from "axios";


// Crée une instance Axios avec une URL de base
const api = axios.create({
  baseURL: "http://127.0.0.1:8080", // 🔁 Remplace par ton API
});

export default api;
