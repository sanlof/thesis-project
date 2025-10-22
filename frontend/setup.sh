#!/bin/bash

# Frontend Setup Script for Thesis Project
# This script sets up the complete React + TypeScript + Vite frontend

set -e  # Exit on any error

echo "🚀 Setting up Frontend for Police-Hospital Data Sharing System"
echo "=============================================================="

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if we're in the right directory
if [ ! -f "package.json" ]; then
    echo -e "${YELLOW}Warning: package.json not found. Please run this script from the frontend directory.${NC}"
    exit 1
fi

# Step 1: Install dependencies
echo -e "\n${BLUE}📦 Step 1: Installing dependencies...${NC}"
npm install

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Dependencies installed successfully${NC}"
else
    echo -e "${YELLOW}✗ Failed to install dependencies${NC}"
    exit 1
fi

# Step 2: Setup environment variables
echo -e "\n${BLUE}⚙️  Step 2: Setting up environment variables...${NC}"
if [ ! -f ".env" ]; then
    cp .env.example .env
    echo -e "${GREEN}✓ Created .env file from .env.example${NC}"
    echo -e "${YELLOW}   Please review and update .env if your backend ports are different${NC}"
else
    echo -e "${YELLOW}   .env file already exists, skipping...${NC}"
fi

# Step 3: Create necessary directories
echo -e "\n${BLUE}📁 Step 3: Creating project directories...${NC}"
mkdir -p src/components/police
mkdir -p src/components/hospital
mkdir -p src/components/shared
mkdir -p src/components/layout
mkdir -p src/services
mkdir -p src/hooks
mkdir -p src/context
mkdir -p src/utils
mkdir -p public

echo -e "${GREEN}✓ Project directories created${NC}"

# Step 4: Type checking
echo -e "\n${BLUE}🔍 Step 4: Running TypeScript type check...${NC}"
npm run type-check

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Type checking passed${NC}"
else
    echo -e "${YELLOW}⚠  Type errors found (this is expected if components aren't implemented yet)${NC}"
fi

# Summary
echo -e "\n${GREEN}=============================================================="
echo -e "✅ Frontend setup complete!"
echo -e "==============================================================${NC}"
echo -e "\n${BLUE}Next steps:${NC}"
echo -e "  1. Review and update ${YELLOW}.env${NC} file if needed"
echo -e "  2. Ensure backend services are running:"
echo -e "     ${YELLOW}curl http://localhost:8000/health${NC} (Police)"
echo -e "     ${YELLOW}curl http://localhost:8001/health${NC} (Hospital)"
echo -e "  3. Start the development server:"
echo -e "     ${YELLOW}npm run dev${NC}"
echo -e "  4. Open ${YELLOW}http://localhost:5173${NC} in your browser"
echo -e "\n${BLUE}Available commands:${NC}"
echo -e "  ${YELLOW}npm run dev${NC}        - Start development server"
echo -e "  ${YELLOW}npm run build${NC}      - Build for production"
echo -e "  ${YELLOW}npm run preview${NC}    - Preview production build"
echo -e "  ${YELLOW}npm run lint${NC}       - Run ESLint"
echo -e "  ${YELLOW}npm run type-check${NC} - Run TypeScript type checking"
echo -e "\n${GREEN}Happy coding! 🎉${NC}\n"
