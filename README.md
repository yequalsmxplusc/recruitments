# JU E-Cell Coordinator Recruitment Portal [2026]

## Overview
This portal facilitates the recruitment process for JU E-Cell coordinators. It includes a backend for managing applicant data and a frontend for user interaction. The system integrates with Google Sheets for real-time data updates.

## Techstack Used
- **Rust**: Backend (Actix-Web) and Frontend (Yew + WASM)
- **Tailwind CSS**: Styling via CDN
- **Google API**: For Sheets and Drive integration
- **JWT Authentication**: Secure user sessions

## System Architecture
```
Frontend (Yew + WASM)  <--HTTP-->  Backend (Actix-Web)  <--Google Sheets API-->
    ↑                                 ↑
 Browser (Yew)                Real‑time applicant data
```

### Key Features
- JWT authentication
- Pre-loaded applicant data from Google Sheets
- Mobile/desktop responsive design
- Integration with Google Drive for file uploads

## File Structure

### Backend
```
backend/
├── src/
│   ├── main.rs                # Entry point for the backend
│   ├── auth/                  # Authentication module
│   │   ├── models.rs          # JWT claims and login models
│   │   ├── middleware.rs      # JWT validation middleware
│   │   └── service.rs         # Login endpoint
│   ├── applicants/            # Applicant management
│   │   └── service.rs         # CRUD operations for applicants
│   ├── google_api.rs          # Google Sheets and Drive integration
├── gas-repl-key.json          # Google Service Account credentials
├── Cargo.toml                 # Rust dependencies
```

### Frontend
```
frontend/
├── src/
│   ├── main.rs                # Entry point for the frontend
│   ├── routers.rs             # Route definitions
│   ├── components/            # UI components
│   │   ├── applicant_list.rs  # Displays list of applicants
│   │   ├── login.rs           # Login form
│   │   └── ...                # Other components
│   ├── services/              # API service
│   │   └── api.rs             # Fetch/update applicant data
│   ├── auth/                  # Authentication context
│   │   ├── context.rs         # Auth state management
│   │   └── models.rs          # Auth models
├── index.html                 # HTML template
├── Trunk.toml                 # Trunk configuration
├── Cargo.toml                 # Rust dependencies
```

## Relevant JSON Configuration

### Google Service Account Key (`gas-repl-key.json`)
This file is used for authenticating with Google APIs. Example:
```json
{
  "type": "service_account",
  "project_id": "your-project-id",
  "private_key_id": "your-private-key-id",
  "private_key": "-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----\n",
  "client_email": "your-service-account-email",
  "client_id": "your-client-id",
  "auth_uri": "https://accounts.google.com/o/oauth2/auth",
  "token_uri": "https://oauth2.googleapis.com/token",
  "auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
  "client_x509_cert_url": "https://www.googleapis.com/robot/v1/metadata/x509/your-service-account-email"
}
```

### Applicant Data Example
```json
{
  "id": "123",
  "name": "John Doe",
  "email": "john.doe@example.com",
  "grad_year": "2026",
  "mobile": "1234567890",
  "gender": "Male",
  "faculty": "Engineering",
  "department": "CSE",
  "skills": "Rust, Yew",
  "event_participation": true,
  "why_apply": "To contribute to E-Cell",
  "event_experience": "Organized hackathons",
  "submission1_url": "https://drive.google.com/file/d/abc",
  "submission2_url": "https://drive.google.com/file/d/xyz",
  "interview_slot": "2026-05-10T10:00:00Z",
  "isSelected": false,
  "isAdmin": false,
  "status": "Pending",
  "round": "1"
}
```

## How to Use

### Backend
1. Configure environment variables:
```bash
cd backend
cat <<EOT > .env
JWT_SECRET=your_very_secure_secret_here
ADMIN_USERNAME=admin
ADMIN_PASSWORD_HASH=$(cargo run --bin hash_password)
FRONTEND_URI=http://localhost:3000
SHEET_URL=your_google_sheet_url
DRIVE_URL=your_google_drive_url
GOOGLE_SERVICE_ACCOUNT_JSON=gas-repl-key.json
EOT
```
2. Build and run the backend:
```bash
cargo run --release
```
The backend will start on `http://127.0.0.1:8080`.

### Frontend
1. Install Trunk (if not already):
```bash
cargo install trunk wasm-bindgen-cli
```
2. Build the frontend:
```bash
cd frontend
trunk build --release
```
3. Serve the `dist` folder with any static file server, or use `trunk serve` for local development.

## Deployment Instructions

### Backend
- Ensure `.env` is correctly configured.
- Use a process manager like `systemd` or `pm2` for production.

### Frontend
1. Build the frontend:
```bash
trunk build --release
```
2. Deploy the `dist` folder to a static hosting service (e.g., Vercel).

### Vercel Configuration
- **Root Directory**: `frontend`
- **Build Command**: `trunk build --release`
- **Output Directory**: `dist`
- Add environment variables (e.g., `BACKEND_URL`) in the Vercel dashboard.

## Notes
- The backend binary size can be reduced by stripping debug symbols:
```bash
strip target/release/recruitment-backend
```
- Optimize dependencies in `Cargo.toml` for production builds.
