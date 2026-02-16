<!-- Landing Page Pública - Marketing -->
<template>
  <div class="w-full bg-white">
    <!-- Navigation Header -->
    <nav class="sticky top-0 z-40 border-b border-slate-200 bg-white/95 backdrop-blur-sm">
      <div class="max-w-7xl mx-auto px-4 py-4 flex items-center justify-between">
        <!-- Logo -->
        <div class="flex items-center gap-2">
          <div class="h-8 w-8 rounded-lg bg-gradient-to-br from-primary to-accent text-white font-bold flex items-center justify-center text-sm">
            E
          </div>
          <span class="font-bold text-slate-900">Aevalo</span>
        </div>

        <!-- Desktop Menu -->
        <div class="hidden md:flex items-center gap-8">
          <a href="#features" class="text-sm text-slate-600 hover:text-primary transition">Recursos</a>
          <a href="#pricing" class="text-sm text-slate-600 hover:text-primary transition">Preços</a>
          <a href="#" class="text-sm text-slate-600 hover:text-primary transition">Docs</a>
        </div>

        <!-- CTA Buttons -->
        <div class="flex items-center gap-3">
          <RouterLink to="/login" class="text-sm font-semibold text-primary hover:text-accent transition">
            Entrar
          </RouterLink>
          <RouterLink to="/register" class="px-4 py-2 rounded-lg bg-gradient-to-r from-primary to-accent text-white text-sm font-semibold shadow-md hover:shadow-lg transition">
            Começar Grátis
          </RouterLink>
        </div>
      </div>
    </nav>

    <!-- Hero Section -->
    <section class="relative overflow-hidden">
      <!-- Gradient Background -->
      <div class="absolute inset-0 bg-gradient-to-br from-primary/5 via-accent/5 to-purple-200/10"></div>

      <div class="relative max-w-7xl mx-auto px-4 py-20 lg:py-32 flex flex-col lg:flex-row items-center gap-12">
        <!-- Left Content -->
        <div class="flex-1 space-y-6">
          <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-primary/10 border border-primary/20">
            <span class="h-2 w-2 rounded-full bg-secondary animate-pulse"></span>
            <span class="text-xs font-semibold text-primary">✨ Alimentado por IA</span>
          </div>

          <h1 class="text-4xl lg:text-5xl font-bold text-slate-900 leading-tight">
            Avaliações Científicas
            <span class="block text-transparent bg-clip-text bg-gradient-to-r from-primary to-accent">
              Alimentadas por IA
            </span>
          </h1>

          <p class="text-lg text-slate-600 leading-relaxed max-w-md">
            Crie pesquisas profissionais em minutos com escalas validadas cientificamente. Sem conhecimento técnico necessário.
          </p>

          <!-- CTA Section -->
          <div class="flex flex-col sm:flex-row items-start sm:items-center gap-4 pt-4">
            <div class="flex-1 flex items-center gap-2">
              <input
                type="email"
                placeholder="seu@email.com"
                class="flex-1 px-4 py-3 rounded-lg border border-slate-200 focus:outline-none focus:ring-2 focus:ring-primary/40 focus:border-primary text-sm"
              />
              <button class="px-6 py-3 rounded-lg bg-gradient-to-r from-primary to-accent text-white font-semibold shadow-md hover:shadow-lg transition whitespace-nowrap">
                Começar →
              </button>
            </div>
          </div>

          <!-- Trust Badges -->
          <div class="flex flex-wrap items-center gap-4 pt-2">
            <div class="flex items-center gap-2 text-sm text-slate-600">
              <svg class="h-4 w-4 text-green-500" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
              </svg>
              Gratuito por 14 dias
            </div>
            <div class="flex items-center gap-2 text-sm text-slate-600">
              <svg class="h-4 w-4 text-green-500" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
              </svg>
              Sem cartão de crédito
            </div>
            <div class="flex items-center gap-2 text-sm text-slate-600">
              <svg class="h-4 w-4 text-green-500" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
              </svg>
              IA inclusa
            </div>
          </div>
        </div>

        <!-- Right - Illustration/Preview -->
        <div class="flex-1 hidden lg:block">
          <div class="relative">
            <!-- Gradient orbs background -->
            <div class="absolute -top-20 -right-20 w-64 h-64 bg-primary/10 rounded-full blur-3xl"></div>
            <div class="absolute -bottom-20 -left-20 w-64 h-64 bg-secondary/10 rounded-full blur-3xl"></div>

            <!-- Mock Dashboard Card -->
            <div class="relative bg-white rounded-2xl shadow-2xl p-6 space-y-4 border border-slate-200">
              <div class="flex items-center justify-between">
                <div class="text-sm font-semibold text-slate-900">Suas Avaliações</div>
                <span class="px-2 py-1 text-xs rounded-full bg-green-100 text-green-700">3 Ativas</span>
              </div>

              <!-- Mock evaluation items -->
              <div v-for="i in 3" :key="i" class="flex items-center justify-between p-3 rounded-lg bg-slate-50 hover:bg-slate-100 transition">
                <div class="flex-1">
                  <div class="text-sm font-medium text-slate-900">Pesquisa {{ i === 1 ? 'NPS' : i === 2 ? 'Satisfação' : 'Clima' }}</div>
                  <div class="text-xs text-slate-500 mt-1">{{ Math.floor(Math.random() * 50 + 10) }}/{{ Math.floor(Math.random() * 50 + 50) }} respostas</div>
                </div>
                <div class="h-1.5 w-16 rounded-full bg-slate-200 overflow-hidden">
                  <div :style="`width: ${Math.random() * 100}%`" class="h-full bg-gradient-to-r from-primary to-accent"></div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Features Section -->
    <section id="features" class="max-w-7xl mx-auto px-4 py-20 space-y-12">
      <div class="text-center space-y-4">
        <h2 class="text-3xl lg:text-4xl font-bold text-slate-900">Por que escolher Aevalo?</h2>
        <p class="text-lg text-slate-600 max-w-2xl mx-auto">Combinamos o poder da IA com rigor científico para criar avaliações profissionais</p>
      </div>

      <!-- Features Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <!-- Feature 1: AI -->
        <div class="group p-6 rounded-2xl border border-slate-200 hover:border-primary hover:shadow-lg hover:shadow-primary/10 transition">
          <div class="h-12 w-12 rounded-lg bg-primary/10 flex items-center justify-center text-2xl mb-4 group-hover:scale-110 transition">
            🤖
          </div>
          <h3 class="font-semibold text-slate-900 mb-2">IA Inteligente</h3>
          <p class="text-sm text-slate-600">Gere perguntas profissionais em 30 segundos com Gemini</p>
        </div>

        <!-- Feature 2: 7 Types -->
        <div class="group p-6 rounded-2xl border border-slate-200 hover:border-primary hover:shadow-lg hover:shadow-primary/10 transition">
          <div class="h-12 w-12 rounded-lg bg-accent/10 flex items-center justify-center text-2xl mb-4 group-hover:scale-110 transition">
            📊
          </div>
          <h3 class="font-semibold text-slate-900 mb-2">7 Tipos de Escalas</h3>
          <p class="text-sm text-slate-600">Likert, Fixed Sum, Paired Comparison e mais</p>
        </div>

        <!-- Feature 3: Speed -->
        <div class="group p-6 rounded-2xl border border-slate-200 hover:border-primary hover:shadow-lg hover:shadow-primary/10 transition">
          <div class="h-12 w-12 rounded-lg bg-purple-500/10 flex items-center justify-center text-2xl mb-4 group-hover:scale-110 transition">
            ⚡
          </div>
          <h3 class="font-semibold text-slate-900 mb-2">Super Rápido</h3>
          <p class="text-sm text-slate-600">Publique em menos de 5 minutos</p>
        </div>

        <!-- Feature 4: Real-time -->
        <div class="group p-6 rounded-2xl border border-slate-200 hover:border-primary hover:shadow-lg hover:shadow-primary/10 transition">
          <div class="h-12 w-12 rounded-lg bg-green-500/10 flex items-center justify-center text-2xl mb-4 group-hover:scale-110 transition">
            📈
          </div>
          <h3 class="font-semibold text-slate-900 mb-2">Real-Time Analytics</h3>
          <p class="text-sm text-slate-600">Veja respostas chegando em tempo real</p>
        </div>

        <!-- Feature 5: Security -->
        <div class="group p-6 rounded-2xl border border-slate-200 hover:border-primary hover:shadow-lg hover:shadow-primary/10 transition">
          <div class="h-12 w-12 rounded-lg bg-blue-500/10 flex items-center justify-center text-2xl mb-4 group-hover:scale-110 transition">
            🔒
          </div>
          <h3 class="font-semibold text-slate-900 mb-2">Seguro & Compliant</h3>
          <p class="text-sm text-slate-600">SOC2 Type II e LGPD/GDPR compliant</p>
        </div>

        <!-- Feature 6: Export -->
        <div class="group p-6 rounded-2xl border border-slate-200 hover:border-primary hover:shadow-lg hover:shadow-primary/10 transition">
          <div class="h-12 w-12 rounded-lg bg-orange-500/10 flex items-center justify-center text-2xl mb-4 group-hover:scale-110 transition">
            📥
          </div>
          <h3 class="font-semibold text-slate-900 mb-2">Exporte Facilmente</h3>
          <p class="text-sm text-slate-600">CSV, PDF, JSON com um clique</p>
        </div>

        <!-- Feature 7: Customize -->
        <div class="group p-6 rounded-2xl border border-slate-200 hover:border-primary hover:shadow-lg hover:shadow-primary/10 transition">
          <div class="h-12 w-12 rounded-lg bg-pink-500/10 flex items-center justify-center text-2xl mb-4 group-hover:scale-110 transition">
            🎨
          </div>
          <h3 class="font-semibold text-slate-900 mb-2">Design Moderno</h3>
          <p class="text-sm text-slate-600">Interface limpa e intuitiva</p>
        </div>

        <!-- Feature 8: Support -->
        <div class="group p-6 rounded-2xl border border-slate-200 hover:border-primary hover:shadow-lg hover:shadow-primary/10 transition">
          <div class="h-12 w-12 rounded-lg bg-indigo-500/10 flex items-center justify-center text-2xl mb-4 group-hover:scale-110 transition">
            🤝
          </div>
          <h3 class="font-semibold text-slate-900 mb-2">Suporte Dedicado</h3>
          <p class="text-sm text-slate-600">Equipe pronta para ajudar</p>
        </div>
      </div>
    </section>

    <!-- How It Works -->
    <section class="max-w-7xl mx-auto px-4 py-20 space-y-12">
      <div class="text-center space-y-4">
        <h2 class="text-3xl lg:text-4xl font-bold text-slate-900">Como funciona</h2>
        <p class="text-lg text-slate-600 max-w-2xl mx-auto">3 passos simples para publicar sua avaliação</p>
      </div>

      <!-- Steps -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-8 relative">
        <!-- Connecting lines (hidden on mobile) -->
        <div class="hidden md:block absolute top-1/4 left-1/3 right-1/3 h-1 bg-gradient-to-r from-primary to-accent transform -translate-y-1/2"></div>

        <!-- Step 1 -->
        <div class="relative">
          <div class="flex flex-col items-center text-center space-y-4">
            <div class="h-16 w-16 rounded-full bg-gradient-to-br from-primary to-accent text-white flex items-center justify-center text-2xl font-bold shadow-lg">
              1
            </div>
            <h3 class="text-xl font-semibold text-slate-900">Descreva</h3>
            <p class="text-slate-600">"Pesquisa de NPS B2B" ou use templates prontos</p>
          </div>
        </div>

        <!-- Step 2 -->
        <div class="relative">
          <div class="flex flex-col items-center text-center space-y-4">
            <div class="h-16 w-16 rounded-full bg-gradient-to-br from-accent to-primary text-white flex items-center justify-center text-2xl font-bold shadow-lg">
              2
            </div>
            <h3 class="text-xl font-semibold text-slate-900">Customize</h3>
            <p class="text-slate-600">Edite perguntas e escolha escalas científicas</p>
          </div>
        </div>

        <!-- Step 3 -->
        <div class="relative">
          <div class="flex flex-col items-center text-center space-y-4">
            <div class="h-16 w-16 rounded-full bg-gradient-to-br from-purple-600 to-primary text-white flex items-center justify-center text-2xl font-bold shadow-lg">
              3
            </div>
            <h3 class="text-xl font-semibold text-slate-900">Compartilhe</h3>
            <p class="text-slate-600">Link, QR Code, ou integre no seu site</p>
          </div>
        </div>
      </div>

      <div class="text-center pt-8">
        <button class="px-6 py-3 rounded-lg border border-primary text-primary font-semibold hover:bg-primary hover:text-white transition">
          Ver Demo Interativa →
        </button>
      </div>
    </section>

    <!-- Use Cases -->
    <section class="bg-slate-50 py-20">
      <div class="max-w-7xl mx-auto px-4 space-y-12">
        <div class="text-center space-y-4">
          <h2 class="text-3xl lg:text-4xl font-bold text-slate-900">Casos de uso</h2>
          <p class="text-lg text-slate-600 max-w-2xl mx-auto">Usado por diferentes equipes e indústrias</p>
        </div>

        <!-- Use cases grid -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <!-- Use case 1 -->
          <div class="bg-white rounded-2xl p-6 border border-slate-200 hover:border-primary transition">
            <div class="text-3xl mb-3">👔</div>
            <h3 class="text-xl font-semibold text-slate-900 mb-2">RH & People Ops</h3>
            <p class="text-slate-600 mb-4">Pulse surveys, eNPS, clima organizacional</p>
            <ul class="space-y-2 text-sm text-slate-600">
              <li>✓ Engajamento de colaboradores</li>
              <li>✓ Satisfação com liderança</li>
              <li>✓ Feedback 360°</li>
            </ul>
          </div>

          <!-- Use case 2 -->
          <div class="bg-white rounded-2xl p-6 border border-slate-200 hover:border-primary transition">
            <div class="text-3xl mb-3">🛍️</div>
            <h3 class="text-xl font-semibold text-slate-900 mb-2">Produto & UX</h3>
            <p class="text-slate-600 mb-4">Satisfação, usabilidade, feature requests</p>
            <ul class="space-y-2 text-sm text-slate-600">
              <li>✓ CSAT & NPS</li>
              <li>✓ Testes de usabilidade</li>
              <li>✓ Feature prioritization</li>
            </ul>
          </div>

          <!-- Use case 3 -->
          <div class="bg-white rounded-2xl p-6 border border-slate-200 hover:border-primary transition">
            <div class="text-3xl mb-3">📚</div>
            <h3 class="text-xl font-semibold text-slate-900 mb-2">Educação & Pesquisa</h3>
            <p class="text-slate-600 mb-4">Coleta de dados acadêmicos com rigor</p>
            <ul class="space-y-2 text-sm text-slate-600">
              <li>✓ Pesquisas científicas</li>
              <li>✓ Avaliação de alunos</li>
              <li>✓ Qualidade de disciplinas</li>
            </ul>
          </div>

          <!-- Use case 4 -->
          <div class="bg-white rounded-2xl p-6 border border-slate-200 hover:border-primary transition">
            <div class="text-3xl mb-3">🎯</div>
            <h3 class="text-xl font-semibold text-slate-900 mb-2">Marketing</h3>
            <p class="text-slate-600 mb-4">Brand awareness, NPS, customer journey</p>
            <ul class="space-y-2 text-sm text-slate-600">
              <li>✓ Brand tracking</li>
              <li>✓ Market research</li>
              <li>✓ Customer satisfaction</li>
            </ul>
          </div>
        </div>
      </div>
    </section>

    <!-- Social Proof -->
    <section class="max-w-7xl mx-auto px-4 py-20 space-y-12">
      <div class="text-center space-y-4">
        <h2 class="text-3xl lg:text-4xl font-bold text-slate-900">Confiado por equipes de</h2>
      </div>

      <!-- Logos - Mock -->
      <div class="flex justify-center items-center gap-8 flex-wrap">
        <div class="h-10 px-4 rounded-lg bg-slate-100 flex items-center justify-center text-slate-600 font-semibold text-sm">
          TechCorp
        </div>
        <div class="h-10 px-4 rounded-lg bg-slate-100 flex items-center justify-center text-slate-600 font-semibold text-sm">
          StartUp AI
        </div>
        <div class="h-10 px-4 rounded-lg bg-slate-100 flex items-center justify-center text-slate-600 font-semibold text-sm">
          DataFlow Inc
        </div>
        <div class="h-10 px-4 rounded-lg bg-slate-100 flex items-center justify-center text-slate-600 font-semibold text-sm">
          Cloud Solutions
        </div>
        <div class="h-10 px-4 rounded-lg bg-slate-100 flex items-center justify-center text-slate-600 font-semibold text-sm">
          Research Lab
        </div>
      </div>

      <!-- Testimonial -->
      <div class="bg-gradient-to-br from-primary/5 to-accent/5 border border-primary/20 rounded-2xl p-8 max-w-2xl mx-auto text-center space-y-4">
        <div class="flex justify-center gap-1">
          <span v-for="i in 5" :key="i" class="text-xl">⭐</span>
        </div>
        <p class="text-lg text-slate-900 font-medium">
          "Reduzimos o tempo de criação de pesquisas de 2 horas para 5 minutos com Aevalo."
        </p>
        <p class="text-sm text-slate-600">— Ana Silva, Head of UX @ TechCorp</p>
        <p class="text-xs text-slate-500">4.9/5 de 200+ reviews</p>
      </div>
    </section>

    <!-- Pricing Section -->
    <section id="pricing" class="bg-slate-50 py-20">
      <div class="max-w-7xl mx-auto px-4 space-y-12">
        <div class="text-center space-y-4">
          <h2 class="text-3xl lg:text-4xl font-bold text-slate-900">Planos simples e transparentes</h2>
          <p class="text-lg text-slate-600 max-w-2xl mx-auto">Escolha o plano certo para sua equipe</p>
        </div>

        <!-- Pricing Cards -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
          <!-- Free Plan -->
          <div class="bg-white rounded-2xl border border-slate-200 p-8 space-y-6 hover:border-primary transition">
            <div>
              <h3 class="text-xl font-bold text-slate-900">Gratuito</h3>
              <p class="text-4xl font-bold text-slate-900 mt-2">R$ 0</p>
              <p class="text-sm text-slate-600 mt-1">/mês</p>
            </div>

            <ul class="space-y-4">
              <li class="flex items-start gap-3">
                <svg class="h-5 w-5 text-green-500 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>
                <span class="text-sm text-slate-600">3 avaliações</span>
              </li>
              <li class="flex items-start gap-3">
                <svg class="h-5 w-5 text-green-500 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>
                <span class="text-sm text-slate-600">50 respostas</span>
              </li>
              <li class="flex items-start gap-3">
                <svg class="h-5 w-5 text-green-500 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>
                <span class="text-sm text-slate-600">IA básica</span>
              </li>
            </ul>

            <RouterLink to="/register" class="block w-full py-2 px-4 rounded-lg border border-slate-200 text-center text-sm font-semibold text-slate-900 hover:bg-slate-50 transition">
              Começar
            </RouterLink>
          </div>

          <!-- Pro Plan (Featured) -->
          <div class="bg-white rounded-2xl border-2 border-primary p-8 space-y-6 relative shadow-lg">
            <div class="absolute -top-4 right-8 px-4 py-1 bg-gradient-to-r from-primary to-accent text-white text-xs font-bold rounded-full">
              Mais Popular
            </div>

            <div>
              <h3 class="text-xl font-bold text-slate-900">Pro</h3>
              <p class="text-4xl font-bold text-slate-900 mt-2">R$ 49</p>
              <p class="text-sm text-slate-600 mt-1">/mês</p>
            </div>

            <ul class="space-y-4">
              <li class="flex items-start gap-3">
                <svg class="h-5 w-5 text-green-500 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>
                <span class="text-sm text-slate-600">Ilimitado</span>
              </li>
              <li class="flex items-start gap-3">
                <svg class="h-5 w-5 text-green-500 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>
                <span class="text-sm text-slate-600">1.000 respostas</span>
              </li>
              <li class="flex items-start gap-3">
                <svg class="h-5 w-5 text-green-500 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>
                <span class="text-sm text-slate-600">IA avançada</span>
              </li>
              <li class="flex items-start gap-3">
                <svg class="h-5 w-5 text-green-500 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>
                <span class="text-sm text-slate-600">Exportar dados</span>
              </li>
              <li class="flex items-start gap-3">
                <svg class="h-5 w-5 text-green-500 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>
                <span class="text-sm text-slate-600">Analytics avançado</span>
              </li>
            </ul>

            <RouterLink to="/register" class="block w-full py-2 px-4 rounded-lg bg-gradient-to-r from-primary to-accent text-white text-center text-sm font-semibold hover:shadow-lg transition">
              Começar
            </RouterLink>
          </div>

          <!-- Team Plan -->
          <div class="bg-white rounded-2xl border border-slate-200 p-8 space-y-6 hover:border-primary transition">
            <div>
              <h3 class="text-xl font-bold text-slate-900">Team</h3>
              <p class="text-4xl font-bold text-slate-900 mt-2">R$ 149</p>
              <p class="text-sm text-slate-600 mt-1">/mês</p>
            </div>

            <ul class="space-y-4">
              <li class="flex items-start gap-3">
                <svg class="h-5 w-5 text-green-500 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>
                <span class="text-sm text-slate-600">Tudo do Pro</span>
              </li>
              <li class="flex items-start gap-3">
                <svg class="h-5 w-5 text-green-500 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>
                <span class="text-sm text-slate-600">5.000 respostas</span>
              </li>
              <li class="flex items-start gap-3">
                <svg class="h-5 w-5 text-green-500 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>
                <span class="text-sm text-slate-600">White label</span>
              </li>
              <li class="flex items-start gap-3">
                <svg class="h-5 w-5 text-green-500 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>
                <span class="text-sm text-slate-600">SSO & SAML</span>
              </li>
              <li class="flex items-start gap-3">
                <svg class="h-5 w-5 text-green-500 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>
                <span class="text-sm text-slate-600">Suporte prioritário</span>
              </li>
            </ul>

            <button class="block w-full py-2 px-4 rounded-lg bg-slate-900 text-white text-center text-sm font-semibold hover:bg-slate-800 transition">
              Falar com Vendas
            </button>
          </div>
        </div>
      </div>
    </section>

    <!-- FAQ Section -->
    <section class="max-w-4xl mx-auto px-4 py-20 space-y-12">
      <div class="text-center space-y-4">
        <h2 class="text-3xl lg:text-4xl font-bold text-slate-900">Perguntas Frequentes</h2>
      </div>

      <!-- FAQ Items -->
      <div class="space-y-4">
        <details class="group border border-slate-200 rounded-lg p-4 hover:border-primary transition">
          <summary class="flex items-center justify-between cursor-pointer">
            <span class="font-semibold text-slate-900">Preciso de cartão para testar?</span>
            <span class="text-2xl group-open:rotate-180 transition">▶</span>
          </summary>
          <p class="mt-4 text-slate-600">
            Não! Você pode criar uma conta e testar completamente gratuitamente por 14 dias, sem necessidade de cartão de crédito.
          </p>
        </details>

        <details class="group border border-slate-200 rounded-lg p-4 hover:border-primary transition">
          <summary class="flex items-center justify-between cursor-pointer">
            <span class="font-semibold text-slate-900">Como funciona a geração por IA?</span>
            <span class="text-2xl group-open:rotate-180 transition">▶</span>
          </summary>
          <p class="mt-4 text-slate-600">
            Você descreve o que precisa (ex: "Pesquisa de NPS para B2B") e nossa IA Gemini cria automaticamente perguntas profissionais em cerca de 30 segundos. Você pode editar tudo depois.
          </p>
        </details>

        <details class="group border border-slate-200 rounded-lg p-4 hover:border-primary transition">
          <summary class="flex items-center justify-between cursor-pointer">
            <span class="font-semibold text-slate-900">Posso exportar os dados?</span>
            <span class="text-2xl group-open:rotate-180 transition">▶</span>
          </summary>
          <p class="mt-4 text-slate-600">
            Sim! Planos Pro e acima podem exportar dados em CSV, PDF ou JSON. Dados são sempre seus.
          </p>
        </details>

        <details class="group border border-slate-200 rounded-lg p-4 hover:border-primary transition">
          <summary class="flex items-center justify-between cursor-pointer">
            <span class="font-semibold text-slate-900">É compatível com LGPD/GDPR?</span>
            <span class="text-2xl group-open:rotate-180 transition">▶</span>
          </summary>
          <p class="mt-4 text-slate-600">
            Totalmente! Aevalo é SOC2 Type II compliant e segue rigorosamente LGPD (Brasil) e GDPR (Europa).
          </p>
        </details>

        <details class="group border border-slate-200 rounded-lg p-4 hover:border-primary transition">
          <summary class="flex items-center justify-between cursor-pointer">
            <span class="font-semibold text-slate-900">Quanto tempo leva para criar uma avaliação?</span>
            <span class="text-2xl group-open:rotate-180 transition">▶</span>
          </summary>
          <p class="mt-4 text-slate-600">
            Com IA: ~5 minutos. Manualmente: depende de você. Com templates: 2-3 minutos. Todos podem ser publicados imediatamente.
          </p>
        </details>

        <details class="group border border-slate-200 rounded-lg p-4 hover:border-primary transition">
          <summary class="flex items-center justify-between cursor-pointer">
            <span class="font-semibold text-slate-900">Vocês têm API?</span>
            <span class="text-2xl group-open:rotate-180 transition">▶</span>
          </summary>
          <p class="mt-4 text-slate-600">
            Sim! Nossa API REST é disponível para planos Team. Documentação completa em aevalo.app/docs
          </p>
        </details>
      </div>
    </section>

    <!-- CTA Final Section -->
    <section class="max-w-4xl mx-auto px-4 py-20 text-center space-y-6">
      <h2 class="text-4xl font-bold text-slate-900">Pronto para começar?</h2>
      <p class="text-lg text-slate-600">Crie sua primeira avaliação em minutos, sem cartão de crédito</p>
      <div class="flex flex-col sm:flex-row items-center justify-center gap-4">
        <RouterLink to="/register" class="px-8 py-3 rounded-lg bg-gradient-to-r from-primary to-accent text-white font-semibold shadow-md hover:shadow-lg transition">
          Começar Grátis →
        </RouterLink>
        <button class="px-8 py-3 rounded-lg border-2 border-primary text-primary font-semibold hover:bg-primary hover:text-white transition">
          Agendar Demo
        </button>
      </div>
    </section>

    <!-- Footer -->
    <footer class="bg-slate-900 text-slate-200">
      <div class="max-w-7xl mx-auto px-4 py-12 space-y-8">
        <!-- Footer Top -->
        <div class="grid grid-cols-2 md:grid-cols-4 gap-8">
          <!-- Product -->
          <div class="space-y-4">
            <h4 class="font-semibold text-white">Produto</h4>
            <ul class="space-y-2 text-sm">
              <li><a href="#" class="hover:text-white transition">Recursos</a></li>
              <li><a href="#" class="hover:text-white transition">Preços</a></li>
              <li><a href="#" class="hover:text-white transition">Roadmap</a></li>
              <li><a href="#" class="hover:text-white transition">Changelog</a></li>
            </ul>
          </div>

          <!-- Resources -->
          <div class="space-y-4">
            <h4 class="font-semibold text-white">Recursos</h4>
            <ul class="space-y-2 text-sm">
              <li><a href="#" class="hover:text-white transition">Templates</a></li>
              <li><a href="#" class="hover:text-white transition">IA</a></li>
              <li><a href="#" class="hover:text-white transition">Analytics</a></li>
              <li><a href="#" class="hover:text-white transition">API</a></li>
            </ul>
          </div>

          <!-- Support -->
          <div class="space-y-4">
            <h4 class="font-semibold text-white">Suporte</h4>
            <ul class="space-y-2 text-sm">
              <li><a href="#" class="hover:text-white transition">Documentação</a></li>
              <li><a href="#" class="hover:text-white transition">Status</a></li>
              <li><a href="#" class="hover:text-white transition">Contato</a></li>
              <li><a href="#" class="hover:text-white transition">Comunidade</a></li>
            </ul>
          </div>

          <!-- Company -->
          <div class="space-y-4">
            <h4 class="font-semibold text-white">Empresa</h4>
            <ul class="space-y-2 text-sm">
              <li><a href="#" class="hover:text-white transition">Sobre</a></li>
              <li><a href="#" class="hover:text-white transition">Blog</a></li>
              <li><a href="#" class="hover:text-white transition">Carreiras</a></li>
              <li><a href="#" class="hover:text-white transition">Legal</a></li>
            </ul>
          </div>
        </div>

        <!-- Footer Bottom -->
        <div class="border-t border-slate-700 pt-8 flex flex-col sm:flex-row items-center justify-between gap-4">
          <p class="text-sm">© 2026 Aevalo. Todos os direitos reservados.</p>

          <!-- Social Links -->
          <div class="flex items-center gap-6">
            <a href="#" class="hover:text-white transition">Twitter</a>
            <a href="#" class="hover:text-white transition">LinkedIn</a>
            <a href="#" class="hover:text-white transition">GitHub</a>
            <a href="#" class="hover:text-white transition">YouTube</a>
          </div>
        </div>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { RouterLink } from 'vue-router'
</script>

<style scoped>
/* Smooth scroll behavior */
html {
  scroll-behavior: smooth;
}

/* Gradient text animation */
@keyframes gradient-flow {
  0% {
    background-position: 0% 50%;
  }
  50% {
    background-position: 100% 50%;
  }
  100% {
    background-position: 0% 50%;
  }
}

/* Pulse animation for badges */
@keyframes pulse-custom {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}
</style>
