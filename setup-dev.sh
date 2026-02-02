#!/bin/bash

# Setup script for Aevalo development environment

set -e

echo "🚀 Setting up Aevalo development environment..."

# Check prerequisites
echo "📋 Checking prerequisites..."

if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Please install Docker."
    exit 1
fi

if ! command -v docker compose &> /dev/null; then
    echo "❌ Docker Compose is not installed. Please install Docker Compose."
    exit 1
fi

# Create environment files
echo "📝 Creating environment files..."
cp backend/.env.example backend/.env
cp frontend/.env.example frontend/.env
cp .env.example .env

echo "✏️  Please update .env files with your configuration:"
echo "   - backend/.env"
echo "   - frontend/.env"
echo "   - .env"
echo ""

# Build Docker images
echo "🐳 Building Docker images..."
docker compose build

# Start services
echo "🚀 Starting services..."
docker compose up -d

echo ""
echo "✅ Setup complete!"
echo ""
echo "📍 Access points:"
echo "   - Frontend: http://localhost"
echo "   - Backend: http://localhost:3000/graphql"
echo "   - Prometheus: http://localhost:9090"
echo "   - Grafana: http://localhost:3001 (admin/admin)"
echo ""
echo "📚 Next steps:"
echo "   1. Review and update .env files"
echo "   2. Run database migrations (if needed)"
echo "   3. Start developing!"
echo ""
echo "💡 Useful commands:"
echo "   - docker compose logs -f backend    # View backend logs"
echo "   - docker compose logs -f frontend   # View frontend logs"
echo "   - docker compose down                # Stop all services"
echo "   - docker compose ps                  # Check service status"
