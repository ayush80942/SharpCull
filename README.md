# 📸 SharpCull – Smart Image Culling API in Rust

Cullify is a fast and modular backend API built with **Rust** and **Axum** to automatically analyze the quality of uploaded images. It detects blurry or poorly lit photos by computing **blurriness** (via Laplacian variance) and **brightness** (average pixel intensity). Ideal for auto-curating photo galleries or building smart upload filters.

---

## 🚀 Features

- 📤 Accepts image uploads via `multipart/form-data`
- 🔍 Calculates:
  - **Blurriness Score** (Laplacian filter-based)
  - **Brightness Score** (average grayscale intensity)
- ⚡ Asynchronous processing with `tokio`
- ✅ Modular Axum architecture
- 📦 JSON API responses

---

## 📁 Folder Structure

```
cullify/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── routes/
    │   └── analyze.rs
    ├── services/
    │   └── image_analysis.rs
    └── utils/
        └── file.rs
```

---

## 🔧 Setup & Usage

### 1. Clone the Repository

```bash
git clone https://github.com/your-username/cullify.git
cd cullify
```

### 2. Build and Run

```bash
cargo run
```

The server will run at:  
`http://localhost:3000`

---

## 🧪 API Usage

### ✅ Health Check

```http
GET /health
```

Response: `"OK"`

---

### 📤 Analyze Image

```http
POST /analyze
Content-Type: multipart/form-data
```

**Body:**  
- `file`: The image file (JPG, PNG)

**Example using curl:**

```bash
curl -X POST http://localhost:3000/analyze \
  -F "file=@sample.jpg"
```

**Response:**
```json
{
  "blur_score": 143.25,
  "brightness": 127.84
}
```

---

## 🛠 Dependencies

- [Axum](https://crates.io/crates/axum)
- [Tokio](https://tokio.rs)
- [Image](https://crates.io/crates/image)
- [Serde](https://serde.rs)
- [UUID](https://crates.io/crates/uuid)
- [Tracing](https://crates.io/crates/tracing)

---

## 📌 Project Status

✅ MVP Complete — ready for resume and portfolio  
🧩 Possible extensions:
- Perceptual hashing to detect near-duplicate images  
- SQLite/DB integration for tracking analyzed results  
- Web frontend / UI for visual reports  
- Dockerization for easy deployment  

---

## 📄 License

MIT License.  
© 2025 Ayush Aggarwal

---
```
