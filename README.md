# JU E-Cell Coordinator Recruitment Portal [2025]
- Implementation incorporating all requirements:
 ## Techstack Used:
 - Rust
 - Tailwind CSS via CDN
 - Used gtag for analytics, and JSON control on backend

 ### Features:
  [x] JWT authentication
  [x] Pre-loaded applicant data
  [x] Mobile/desktop responsive design
  [x] Tailwind CSS via CDN


 ## System Architecture:
 ```
 Frontend (Yew/WASM)  <--HTTP-->  Backend (Actix-Web)  <-->  data.json
       ↑                                  ↑
    JWT Auth                          Token Validation
```
 ## Backend Implementation:
 ```
 backend/
├── src/
│   ├── main.rs
│   ├── auth/
│   │   ├── models.rs
│   │   ├── middleware.rs
│   │   └── service.rs
│   └── applicants/
│       └── service.rs
├── data/
│   └── data.json
└── Cargo.toml
```
 ## Frontend Implementation:
 ```
 frontend/
├── src/
│   ├── main.rs
│   ├── router.rs
│   ├── components/
│   │   ├── mod.rs
│   │   ├── login.rs
│   │   └── applicant_list.rs
│   ├── models/
│   │   ├── mod.rs
│   │   └── applicant.rs
│   ├── services/
│   │   ├── mod.rs
│   │   └── api.rs
│   ├── auth/
│   │   ├── mod.rs
│   │   ├── context.rs
│   │   └── models.rs
│   └── utils/
│       ├── mod.rs
│       └── storage.rs
├── index.html
├── Cargo.toml
└── .env
```
 ## Deployment Instructions
  - Set up backend environment:

```
cd backend
echo "JWT_SECRET=your_very_secure_secret_here" > .env
echo "ADMIN_USERNAME=admin" >> .env
echo "ADMIN_PASSWORD_HASH=$(cargo run --bin hash_password)" >> .env
echo "FRONTEND_URI=..." >>.env
```

 - Prepare frontend:

```
cd ../frontend
cargo build
trunk serve --port 3000 
```
 - Run the system:

Backend: ```cargo run``` (from backend directory)

Frontend: Serve the dist folder from any web server