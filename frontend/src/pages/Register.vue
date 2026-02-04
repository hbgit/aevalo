<template>
  <div class="min-h-screen bg-slate-100 flex items-center justify-center px-4 py-10">
    <div class="w-full max-w-sm bg-white rounded-3xl shadow-xl overflow-hidden">
      <div class="bg-gradient-to-r from-orange-500 via-rose-500 to-purple-600 px-8 pt-10 pb-12 text-center text-white">
        <div class="mx-auto h-16 w-16 rounded-2xl bg-white/20 backdrop-blur flex items-center justify-center">
          <span class="text-2xl font-semibold">A</span>
        </div>
        <h1 class="mt-4 text-2xl font-semibold">Aevalo</h1>
        <p class="text-sm text-white/80">Avaliações inteligentes para seu negócio</p>
      </div>

      <div class="px-8 pb-8 -mt-8">
        <div class="mt-6">
          <h2 class="text-xl font-semibold text-slate-900">Criar sua conta</h2>
          <p class="text-sm text-slate-500 mt-1">Junte-se ao Aevalo e comece agora</p>
        </div>

        <form class="mt-6 space-y-4">
          <div>
            <label class="block text-sm font-medium text-slate-700 mb-2">Nome Completo <span class="text-orange-500">*</span></label>
            <div class="relative">
              <span class="absolute left-3 top-1/2 -translate-y-1/2 text-slate-400">
                <svg class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                  <path d="M10 10a4 4 0 100-8 4 4 0 000 8zm-6 8a6 6 0 0112 0H4z" />
                </svg>
              </span>
              <input
                type="text"
                placeholder="Seu Nome Completo"
                class="w-full border border-slate-200 rounded-xl pl-10 pr-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-primary/30"
              />
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-slate-700 mb-2">Email <span class="text-orange-500">*</span></label>
            <div class="relative">
              <span class="absolute left-3 top-1/2 -translate-y-1/2 text-slate-400">
                <svg class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                  <path d="M2.94 5.5 10 9.82l7.06-4.32A2 2 0 0015.82 4H4.18a2 2 0 00-1.24 1.5z" />
                  <path d="M18 8.08l-7.46 4.56a1 1 0 01-1.08 0L2 8.08V14a2 2 0 002 2h12a2 2 0 002-2V8.08z" />
                </svg>
              </span>
              <input
                type="email"
                placeholder="seu.email@exemplo.com"
                class="w-full border border-slate-200 rounded-xl pl-10 pr-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-primary/30"
              />
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-slate-700 mb-2">Senha <span class="text-orange-500">*</span></label>
            <div class="relative">
              <span class="absolute left-3 top-1/2 -translate-y-1/2 text-slate-400">
                <svg class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                  <path fill-rule="evenodd" d="M5 8a5 5 0 1110 0v2h1a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6a1 1 0 011-1h1V8zm2 2h6V8a3 3 0 00-6 0v2z" clip-rule="evenodd" />
                </svg>
              </span>
              <input
                type="password"
                placeholder="Crie sua senha"
                class="w-full border border-slate-200 rounded-xl pl-10 pr-10 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-primary/30"
              />
              <button type="button" class="absolute right-3 top-1/2 -translate-y-1/2 text-slate-400">
                <svg class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                  <path d="M10 3c4.418 0 8 4 8 7s-3.582 7-8 7-8-4-8-7 3.582-7 8-7zm0 2c-3.21 0-6 3-6 5s2.79 5 6 5 6-3 6-5-2.79-5-6-5zm0 2a3 3 0 110 6 3 3 0 010-6z" />
                </svg>
              </button>
            </div>
          </div>

          <div class="text-xs text-slate-500 space-y-1">
            <p>A senha deve conter:</p>
            <ul class="list-disc list-inside space-y-1">
              <li>Mínimo de 8 caracteres</li>
              <li>Uma letra maiúscula</li>
              <li>Um número</li>
              <li>Um caractere especial (!@#$%...)</li>
            </ul>
          </div>

          <div>
            <label class="block text-sm font-medium text-slate-700 mb-2">Confirmar Senha <span class="text-orange-500">*</span></label>
            <div class="relative">
              <span class="absolute left-3 top-1/2 -translate-y-1/2 text-slate-400">
                <svg class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                  <path fill-rule="evenodd" d="M5 8a5 5 0 1110 0v2h1a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6a1 1 0 011-1h1V8zm2 2h6V8a3 3 0 00-6 0v2z" clip-rule="evenodd" />
                </svg>
              </span>
              <input
                type="password"
                placeholder="Confirme sua senha"
                class="w-full border border-slate-200 rounded-xl pl-10 pr-10 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-primary/30"
              />
              <button type="button" class="absolute right-3 top-1/2 -translate-y-1/2 text-slate-400">
                <svg class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                  <path d="M10 3c4.418 0 8 4 8 7s-3.582 7-8 7-8-4-8-7 3.582-7 8-7zm0 2c-3.21 0-6 3-6 5s2.79 5 6 5 6-3 6-5-2.79-5-6-5zm0 2a3 3 0 110 6 3 3 0 010-6z" />
                </svg>
              </button>
            </div>
          </div>

          <label class="flex items-start gap-3 rounded-xl border border-orange-200 bg-orange-50 px-3 py-2 text-xs text-slate-600">
            <input type="checkbox" class="mt-0.5 h-4 w-4 rounded border-orange-300 text-orange-500 focus:ring-orange-400" />
            <span>
              Eu concordo com os
              <a href="#" class="text-purple-600 font-semibold">Termos de Uso</a>
              e
              <a href="#" class="text-purple-600 font-semibold">Política de Privacidade</a>
            </span>
          </label>

          <button type="submit" class="w-full bg-orange-500 hover:bg-orange-600 text-white font-semibold py-3 rounded-xl flex items-center justify-center gap-2">
            Criar Conta
            <svg class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l5 5a1 1 0 010 1.414l-5 5a1 1 0 01-1.414-1.414L13.586 11H4a1 1 0 110-2h9.586l-3.293-3.293a1 1 0 010-1.414z" clip-rule="evenodd" />
            </svg>
          </button>
        </form>

        <div class="mt-6">
          <div class="flex items-center gap-3 text-xs text-slate-400">
            <span class="h-px flex-1 bg-slate-200"></span>
            ou cadastre-se com
            <span class="h-px flex-1 bg-slate-200"></span>
          </div>

          <div class="mt-4 grid grid-cols-2 gap-3">
            <button class="border border-slate-200 rounded-xl py-2.5 text-sm font-semibold text-slate-700 flex items-center justify-center gap-2">
              <span class="text-red-500">G</span>
              Google
            </button>
            <button class="border border-slate-200 rounded-xl py-2.5 text-sm font-semibold text-slate-700 flex items-center justify-center gap-2">
              <svg class="h-4 w-4" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 2C6.477 2 2 6.485 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.161-1.11-1.47-1.11-1.47-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.339-2.22-.253-4.555-1.114-4.555-4.957 0-1.094.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.026 2.747-1.026.546 1.378.203 2.397.1 2.65.64.7 1.028 1.594 1.028 2.688 0 3.852-2.338 4.701-4.566 4.95.359.31.678.921.678 1.856 0 1.338-.012 2.418-.012 2.747 0 .268.18.58.688.482A10.02 10.02 0 0022 12.017C22 6.485 17.523 2 12 2z" />
              </svg>
              GitHub
            </button>
          </div>
        </div>

        <p class="mt-6 text-center text-xs text-slate-500">
          Já tem uma conta?
          <a href="/login" class="text-purple-600 font-semibold">Faça login</a>
        </p>
      </div>
    </div>
  </div>
</template>
