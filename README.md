# 🌤️ ClimaRust Backend

This is the backend API for [ClimaRust](https://climarust.vercel.app), a weather-checking application. It is built using **Rust** and the **Actix Web** framework and fetches real-time weather information based on a city name and optional country code.

---

## 📦 Features

- 🌍 Geolocation lookup using Open-Meteo's Geocoding API
- 🌤️ Weather data from Open-Meteo's Forecast API
- ✅ Supports optional country filtering
- 🔁 CORS enabled (for frontend communication)
- 🚀 Easily deployable (Render, Fly.io, Railway, etc.)

---

## 📌 API Endpoint

### `GET /weather`

**Query Parameters:**

| Parameter | Type   | Required | Description                     |
|-----------|--------|----------|---------------------------------|
| `city`    | string | ✅       | Name of the city                |
| `country` | string | ❌       | Optional 2-letter country code  |

**Example:**
```http
GET /weather?city=Delhi&country=IN```

---

### ▶️ Backend (Rust)

To run the backend locally:

```bash
cd backend
cargo run

