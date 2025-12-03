# 🐳 Rust Docker Registry

A minimal, fully-custom **Docker Registry (Registry API v2)** implemented in **Rust** using **Axum** and **Tokio**.
Designed for learning, extensibility, and building custom container-storage backends.

---

## 🚀 Features

- Full **Docker Registry HTTP API v2** structure
- Endpoints implemented:
  - `GET /v2/_catalog`
  - **Blobs**
    - `GET /v2/<name>/blobs/<digest>`
    - `POST /v2/<name>/blobs/uploads`
    - `PATCH /v2/<name>/blobs/uploads/<uuid>`
    - `PUT /v2/<name>/blobs/uploads/<uuid>`
  - **Manifests**
    - `GET /v2/<name>/manifests/<reference>`
    - `PUT /v2/<name>/manifests/<reference>`
    - `DELETE /v2/<name>/manifests/<reference>`
- Modular code structure (`api/`, `storage/`, `config/`, `errors/`)
- Async I/O via **Tokio**
- Ready for pluggable storage engines (filesystem, S3, database, etc.)

---

## 📂 Project Structure

```
src/
 ├── main.rs
 ├── api/
 │    ├── blobs.rs
 │    ├── catalog.rs
 │    ├── manifests.rs
 ├── storage/
 │    ├── filesystem.rs
 │    ├── s3.rs (future)
 ├── config/
 │    └── mod.rs
 └── errors.rs
```

---

## 🛠 Requirements

- Rust 1.74+  
- Cargo  
- Linux / macOS / Windows

---

## ▶️ Running the Registry

```bash
cargo run
```

Default address:

```
http://0.0.0.0:5000
```

If you see:

```
Address already in use
```

Change the port in `main.rs`:

```rust
let addr = "0.0.0.0:5001".parse().unwrap();
```

---

## 🧪 Testing With Docker

You can push/pull images to your custom registry:

```bash
docker pull alpine:latest
docker tag alpine localhost:5001/alpine
docker push localhost:5001/alpine
```

If your registry is HTTP (not HTTPS):

```bash
sudo mkdir -p /etc/docker
echo '{ "insecure-registries": ["localhost:5001"] }' | sudo tee /etc/docker/daemon.json
sudo systemctl restart docker
```

---

## 📦 Future Work

- Authentication (Basic / Token)
- Garbage collection system
- S3 / MinIO storage backend
- Pagination for catalog
- Full manifest/OCI spec validation
- Chunked uploads
- Metrics + logging
- Optional SQL/NoSQL backend

---

## 🤝 Contributing

Pull requests and feature suggestions are welcome!  
This project is intentionally minimal to serve as a strong starting point.

---

## 📜 License

MIT License