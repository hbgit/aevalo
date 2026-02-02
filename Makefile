version: 1

tasks:
  setup-dev:
    description: "Initialize development environment"
    steps:
      - cp backend/.env.example backend/.env
      - cp frontend/.env.example frontend/.env
      - cp .env.example .env

  build-backend:
    description: "Build Rust backend"
    steps:
      - cd backend && cargo build --release

  build-frontend:
    description: "Build Vue.js frontend"
    steps:
      - cd frontend && npm ci && npm run build

  start-dev:
    description: "Start development environment"
    steps:
      - docker-compose up -d

  stop-dev:
    description: "Stop development environment"
    steps:
      - docker-compose down

  test-backend:
    description: "Run backend tests"
    steps:
      - cd backend && cargo test

  test-frontend:
    description: "Run frontend tests"
    steps:
      - cd frontend && npm run test

  lint:
    description: "Run linters"
    steps:
      - cd backend && cargo fmt && cargo clippy
      - cd frontend && npm run lint

  format:
    description: "Format code"
    steps:
      - cd backend && cargo fmt
      - cd frontend && npx prettier --write src

  migrate-db:
    description: "Run database migrations"
    steps:
      - cd backend && sqlx migrate run

  generate-graphql-types:
    description: "Generate GraphQL types from schema"
    steps:
      - cd frontend && npm run generate-types
