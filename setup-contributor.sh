#!/usr/bin/env bash
# 🚀 Quick Setup Script for Aevalo Contributors
# Este script ajuda novos contribuidores a configurar seu ambiente

set -e

echo "🚀 Bem-vindo ao Aevalo!"
echo "========================"
echo ""

# Check if git is installed
if ! command -v git &> /dev/null; then
    echo "❌ Git não está instalado. Por favor, instale git primeiro."
    exit 1
fi

echo "✅ Git detectado"
echo ""

# Get user info
read -p "👤 Seu nome GitHub: " GITHUB_USER
read -p "📧 Seu email: " EMAIL

echo ""
echo "🔧 Configurando seu fork..."
echo ""

# Check if we're in the right directory
if [ ! -f "CONTRIBUTING.md" ]; then
    echo "❌ Por favor, execute este script na raiz do repositório aevalo"
    exit 1
fi

echo "📝 Configurando Git..."
git config user.email "$EMAIL"
git config user.name "$GITHUB_USER"

echo "🔗 Adicionando upstream remoto..."
if ! git remote get-url upstream &> /dev/null; then
    git remote add upstream https://github.com/aevalo/aevalo.git
    echo "✅ Upstream adicionado"
else
    echo "ℹ️ Upstream já configurado"
fi

echo ""
echo "📚 Recursos Importantes:"
echo "  📖 Guia de Contribuição: CONTRIBUTING.md"
echo "  🏠 Hub de Comunidade: COMMUNITY.md"
echo "  💬 Código de Conduta: CODE_OF_CONDUCT.md"
echo ""

echo "🎯 Próximos passos:"
echo "  1. Leia CONTRIBUTING.md completamente"
echo "  2. Procure por issues com 'good first issue'"
echo "  3. Crie uma branch: git checkout -b feature/seu-nome"
echo "  4. Faça suas mudanças"
echo "  5. Abra um Pull Request"
echo ""

echo "✨ Pronto! Você está configurado para contribuir!"
echo ""
echo "Dúvidas? Consulte COMMUNITY.md ou abra uma discussion no GitHub"
