<!-- Admin Settings Page -->
<template>
  <div class="space-y-6">
    <!-- Page Header -->
    <div>
      <h1 class="text-3xl font-bold text-slate-900">⚙️ Configurações do Sistema</h1>
      <p class="text-sm text-slate-600 mt-1">Configure aspectos globais da plataforma Aevalo</p>
    </div>

    <!-- Settings Tabs Navigation -->
    <div class="flex items-center gap-2 border-b border-slate-200 overflow-x-auto">
      <button
        @click="activeTab = 'general'"
        :class="[
          'px-4 py-3 font-semibold text-sm transition border-b-2',
          activeTab === 'general'
            ? 'text-primary border-primary'
            : 'text-slate-600 border-transparent hover:text-slate-900'
        ]"
      >
        🌍 Geral
      </button>
      <button
        @click="activeTab = 'security'"
        :class="[
          'px-4 py-3 font-semibold text-sm transition border-b-2',
          activeTab === 'security'
            ? 'text-primary border-primary'
            : 'text-slate-600 border-transparent hover:text-slate-900'
        ]"
      >
        🔒 Segurança
      </button>
      <button
        @click="activeTab = 'email'"
        :class="[
          'px-4 py-3 font-semibold text-sm transition border-b-2',
          activeTab === 'email'
            ? 'text-primary border-primary'
            : 'text-slate-600 border-transparent hover:text-slate-900'
        ]"
      >
        📧 Email
      </button>
      <button
        @click="activeTab = 'integrations'"
        :class="[
          'px-4 py-3 font-semibold text-sm transition border-b-2',
          activeTab === 'integrations'
            ? 'text-primary border-primary'
            : 'text-slate-600 border-transparent hover:text-slate-900'
        ]"
      >
        🔗 Integrações
      </button>
      <button
        @click="activeTab = 'maintenance'"
        :class="[
          'px-4 py-3 font-semibold text-sm transition border-b-2',
          activeTab === 'maintenance'
            ? 'text-primary border-primary'
            : 'text-slate-600 border-transparent hover:text-slate-900'
        ]"
      >
        🔧 Manutenção
      </button>
    </div>

    <!-- General Settings Tab -->
    <div v-if="activeTab === 'general'" class="space-y-6">
      <!-- Site Info -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6">
        <h3 class="text-lg font-bold text-slate-900 mb-4">📋 Informações do Site</h3>

        <div class="space-y-4">
          <div>
            <label class="block text-sm font-semibold text-slate-900 mb-1">Nome da Plataforma</label>
            <input
              type="text"
              value="Aevalo"
              class="w-full px-4 py-2 rounded-lg border border-slate-200 focus:outline-none focus:ring-2 focus:ring-primary/40"
            />
          </div>

          <div>
            <label class="block text-sm font-semibold text-slate-900 mb-1">URL Base</label>
            <input
              type="text"
              value="https://aevalo.com"
              class="w-full px-4 py-2 rounded-lg border border-slate-200 focus:outline-none focus:ring-2 focus:ring-primary/40"
            />
          </div>

          <div>
            <label class="block text-sm font-semibold text-slate-900 mb-1">Descrição Meta</label>
            <textarea
              class="w-full px-4 py-2 rounded-lg border border-slate-200 focus:outline-none focus:ring-2 focus:ring-primary/40"
              rows="3"
            >Aevalo - Plataforma de avaliações de clientes com IA. Crie, distribua e analise surveys em minutos.</textarea>
          </div>

          <div class="flex items-end gap-2">
            <button class="px-4 py-2 rounded-lg bg-primary text-white text-sm font-semibold hover:bg-accent transition">
              💾 Salvar
            </button>
            <button class="px-4 py-2 rounded-lg border border-slate-200 text-slate-600 text-sm font-semibold hover:bg-slate-50 transition">
              ↩️ Cancelar
            </button>
          </div>
        </div>
      </div>

      <!-- System Limits -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6">
        <h3 class="text-lg font-bold text-slate-900 mb-4">⚡ Limites do Sistema</h3>

        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-semibold text-slate-900 mb-1">Max. Respostas por Avaliação</label>
            <input
              type="number"
              value="10000"
              class="w-full px-4 py-2 rounded-lg border border-slate-200 focus:outline-none focus:ring-2 focus:ring-primary/40"
            />
          </div>

          <div>
            <label class="block text-sm font-semibold text-slate-900 mb-1">Max. Perguntas por Survey</label>
            <input
              type="number"
              value="100"
              class="w-full px-4 py-2 rounded-lg border border-slate-200 focus:outline-none focus:ring-2 focus:ring-primary/40"
            />
          </div>

          <div>
            <label class="block text-sm font-semibold text-slate-900 mb-1">Retenção de Dados (dias)</label>
            <input
              type="number"
              value="365"
              class="w-full px-4 py-2 rounded-lg border border-slate-200 focus:outline-none focus:ring-2 focus:ring-primary/40"
            />
          </div>

          <div>
            <label class="block text-sm font-semibold text-slate-900 mb-1">Rate Limit (req/min)</label>
            <input
              type="number"
              value="120"
              class="w-full px-4 py-2 rounded-lg border border-slate-200 focus:outline-none focus:ring-2 focus:ring-primary/40"
            />
          </div>
        </div>

        <div class="flex items-end gap-2 mt-4">
          <button class="px-4 py-2 rounded-lg bg-primary text-white text-sm font-semibold hover:bg-accent transition">
            💾 Salvar
          </button>
          <button class="px-4 py-2 rounded-lg border border-slate-200 text-slate-600 text-sm font-semibold hover:bg-slate-50 transition">
            ↩️ Cancelar
          </button>
        </div>
      </div>
    </div>

    <!-- Security Settings Tab -->
    <div v-if="activeTab === 'security'" class="space-y-6">
      <!-- Password Policy -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6">
        <h3 class="text-lg font-bold text-slate-900 mb-4">🔐 Política de Senhas</h3>

        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div>
              <p class="font-semibold text-slate-900">Comprimento Mínimo</p>
              <p class="text-sm text-slate-600">Mínimo de caracteres necessários</p>
            </div>
            <input type="number" value="8" class="w-16 px-3 py-2 rounded-lg border border-slate-200" />
          </div>

          <div class="flex items-center justify-between pt-4 border-t border-slate-200">
            <div>
              <p class="font-semibold text-slate-900">Exigir Maiúsculas e Números</p>
              <p class="text-sm text-slate-600">Aumenta a força da senha</p>
            </div>
            <input type="checkbox" checked class="h-5 w-5" />
          </div>

          <div class="flex items-center justify-between pt-4 border-t border-slate-200">
            <div>
              <p class="font-semibold text-slate-900">Expiração de Sessão</p>
              <p class="text-sm text-slate-600">Em minutos de inatividade</p>
            </div>
            <input type="number" value="30" class="w-16 px-3 py-2 rounded-lg border border-slate-200" />
          </div>

          <div class="flex items-end gap-2 mt-4 pt-4 border-t border-slate-200">
            <button class="px-4 py-2 rounded-lg bg-primary text-white text-sm font-semibold hover:bg-accent transition">
              💾 Salvar
            </button>
            <button class="px-4 py-2 rounded-lg border border-slate-200 text-slate-600 text-sm font-semibold hover:bg-slate-50 transition">
              ↩️ Cancelar
            </button>
          </div>
        </div>
      </div>

      <!-- Two-Factor Authentication -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6">
        <h3 class="text-lg font-bold text-slate-900 mb-4">🔑 Autenticação de Dois Fatores</h3>

        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div>
              <p class="font-semibold text-slate-900">Habilitar 2FA Obrigatório</p>
              <p class="text-sm text-slate-600">Para contas administrativas</p>
            </div>
            <input type="checkbox" class="h-5 w-5" />
          </div>

          <div class="flex items-center justify-between pt-4 border-t border-slate-200">
            <div>
              <p class="font-semibold text-slate-900">Permitir Recuperação por SMS</p>
              <p class="text-sm text-slate-600">Código TOTP via mensagem</p>
            </div>
            <input type="checkbox" checked class="h-5 w-5" />
          </div>

          <div class="flex items-end gap-2 mt-4 pt-4 border-t border-slate-200">
            <button class="px-4 py-2 rounded-lg bg-primary text-white text-sm font-semibold hover:bg-accent transition">
              💾 Salvar
            </button>
            <button class="px-4 py-2 rounded-lg border border-slate-200 text-slate-600 text-sm font-semibold hover:bg-slate-50 transition">
              ↩️ Cancelar
            </button>
          </div>
        </div>
      </div>

      <!-- IP Whitelist -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6">
        <h3 class="text-lg font-bold text-slate-900 mb-4">🛡️ Whitelist de IP</h3>

        <div>
          <label class="block text-sm font-semibold text-slate-900 mb-2">IPs Permitidos (um por linha)</label>
          <textarea
            class="w-full px-4 py-2 rounded-lg border border-slate-200 focus:outline-none focus:ring-2 focus:ring-primary/40"
            rows="4"
          >192.168.1.1
203.0.113.0/24
198.51.100.15</textarea>
          <p class="text-xs text-slate-600 mt-2">Deixe vazio para desabilitar. Suporta CIDR notation.</p>
        </div>

        <div class="flex items-end gap-2 mt-4">
          <button class="px-4 py-2 rounded-lg bg-primary text-white text-sm font-semibold hover:bg-accent transition">
            💾 Salvar
          </button>
          <button class="px-4 py-2 rounded-lg border border-slate-200 text-slate-600 text-sm font-semibold hover:bg-slate-50 transition">
            ↩️ Cancelar
          </button>
        </div>
      </div>
    </div>

    <!-- Email Settings Tab -->
    <div v-if="activeTab === 'email'" class="space-y-6">
      <!-- SMTP Configuration -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6">
        <h3 class="text-lg font-bold text-slate-900 mb-4">📧 Configuração SMTP</h3>

        <div class="space-y-4">
          <div>
            <label class="block text-sm font-semibold text-slate-900 mb-1">Host SMTP</label>
            <input
              type="text"
              value="smtp.gmail.com"
              class="w-full px-4 py-2 rounded-lg border border-slate-200 focus:outline-none focus:ring-2 focus:ring-primary/40"
            />
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-semibold text-slate-900 mb-1">Porta</label>
              <input
                type="number"
                value="587"
                class="w-full px-4 py-2 rounded-lg border border-slate-200 focus:outline-none focus:ring-2 focus:ring-primary/40"
              />
            </div>
            <div>
              <label class="block text-sm font-semibold text-slate-900 mb-1">Criptografia</label>
              <select class="w-full px-4 py-2 rounded-lg border border-slate-200 focus:outline-none focus:ring-2 focus:ring-primary/40">
                <option>TLS</option>
                <option>SSL</option>
                <option>Nenhuma</option>
              </select>
            </div>
          </div>

          <div>
            <label class="block text-sm font-semibold text-slate-900 mb-1">Usuário / Email</label>
            <input
              type="email"
              value="noreply@aevalo.com"
              class="w-full px-4 py-2 rounded-lg border border-slate-200 focus:outline-none focus:ring-2 focus:ring-primary/40"
            />
          </div>

          <div>
            <label class="block text-sm font-semibold text-slate-900 mb-1">Senha</label>
            <input
              type="password"
              placeholder="••••••••"
              class="w-full px-4 py-2 rounded-lg border border-slate-200 focus:outline-none focus:ring-2 focus:ring-primary/40"
            />
          </div>

          <div class="flex items-end gap-2">
            <button class="px-4 py-2 rounded-lg bg-slate-200 text-slate-900 text-sm font-semibold hover:bg-slate-300 transition">
              🧪 Testar
            </button>
            <button class="px-4 py-2 rounded-lg bg-primary text-white text-sm font-semibold hover:bg-accent transition">
              💾 Salvar
            </button>
            <button class="px-4 py-2 rounded-lg border border-slate-200 text-slate-600 text-sm font-semibold hover:bg-slate-50 transition">
              ↩️ Cancelar
            </button>
          </div>
        </div>
      </div>

      <!-- Email Templates -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6">
        <h3 class="text-lg font-bold text-slate-900 mb-4">🎨 Templates de Email</h3>

        <div class="space-y-3">
          <button class="w-full px-4 py-3 text-left font-semibold text-slate-900 bg-slate-50 hover:bg-slate-100 rounded-lg transition flex items-center justify-between">
            <span>📨 Email de Verificação</span>
            <span class="text-slate-600">→</span>
          </button>
          <button class="w-full px-4 py-3 text-left font-semibold text-slate-900 bg-slate-50 hover:bg-slate-100 rounded-lg transition flex items-center justify-between">
            <span>🔑 Reset de Senha</span>
            <span class="text-slate-600">→</span>
          </button>
          <button class="w-full px-4 py-3 text-left font-semibold text-slate-900 bg-slate-50 hover:bg-slate-100 rounded-lg transition flex items-center justify-between">
            <span>🎉 Bem-vindo</span>
            <span class="text-slate-600">→</span>
          </button>
          <button class="w-full px-4 py-3 text-left font-semibold text-slate-900 bg-slate-50 hover:bg-slate-100 rounded-lg transition flex items-center justify-between">
            <span>⚠️ Aviso de Cobrança</span>
            <span class="text-slate-600">→</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Integrations Settings Tab -->
    <div v-if="activeTab === 'integrations'" class="space-y-6">
      <!-- API Keys -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6">
        <h3 class="text-lg font-bold text-slate-900 mb-4">🔑 Chaves de API</h3>

        <div class="space-y-3">
          <div class="flex items-center justify-between p-4 bg-slate-50 rounded-lg">
            <div>
              <p class="font-semibold text-slate-900">API Key Pública</p>
              <p class="text-xs text-slate-600 font-mono">pk_live_5f3a9b2c...</p>
            </div>
            <div class="flex gap-2">
              <button class="px-3 py-1 text-xs text-slate-600 hover:bg-slate-200 rounded transition">
                📋 Copiar
              </button>
              <button class="px-3 py-1 text-xs text-slate-600 hover:bg-slate-200 rounded transition">
                🔄 Regenerar
              </button>
            </div>
          </div>

          <div class="flex items-center justify-between p-4 bg-slate-50 rounded-lg">
            <div>
              <p class="font-semibold text-slate-900">API Key Secreta</p>
              <p class="text-xs text-slate-600 font-mono">••••••••••••••••••••••••</p>
            </div>
            <div class="flex gap-2">
              <button class="px-3 py-1 text-xs text-slate-600 hover:bg-slate-200 rounded transition">
                👁️ Mostrar
              </button>
              <button class="px-3 py-1 text-xs text-slate-600 hover:bg-slate-200 rounded transition">
                🔄 Regenerar
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Third-party Services -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6">
        <h3 class="text-lg font-bold text-slate-900 mb-4">🌐 Serviços de Terceiros</h3>

        <div class="space-y-4">
          <div class="flex items-center justify-between p-4 border border-slate-200 rounded-lg">
            <div class="flex items-center gap-3">
              <span class="text-2xl">📊</span>
              <div>
                <p class="font-semibold text-slate-900">Google Analytics</p>
                <p class="text-xs text-slate-600">ID: G-XXXXXXXXX</p>
              </div>
            </div>
            <div class="flex items-center gap-2">
              <span class="h-2 w-2 rounded-full bg-green-500"></span>
              <button class="text-sm text-slate-600 hover:text-primary transition">Editar</button>
            </div>
          </div>

          <div class="flex items-center justify-between p-4 border border-slate-200 rounded-lg">
            <div class="flex items-center gap-3">
              <span class="text-2xl">🤖</span>
              <div>
                <p class="font-semibold text-slate-900">Google Gemini API</p>
                <p class="text-xs text-slate-600">Para geração de sugestões</p>
              </div>
            </div>
            <div class="flex items-center gap-2">
              <span class="h-2 w-2 rounded-full bg-green-500"></span>
              <button class="text-sm text-slate-600 hover:text-primary transition">Editar</button>
            </div>
          </div>

          <div class="flex items-center justify-between p-4 border border-slate-200 rounded-lg">
            <div class="flex items-center gap-3">
              <span class="text-2xl">💳</span>
              <div>
                <p class="font-semibold text-slate-900">Stripe</p>
                <p class="text-xs text-slate-600">Para processamento de pagamentos</p>
              </div>
            </div>
            <div class="flex items-center gap-2">
              <span class="h-2 w-2 rounded-full bg-green-500"></span>
              <button class="text-sm text-slate-600 hover:text-primary transition">Editar</button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Maintenance Settings Tab -->
    <div v-if="activeTab === 'maintenance'" class="space-y-6">
      <!-- System Health -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6">
        <h3 class="text-lg font-bold text-slate-900 mb-4">🏥 Saúde do Sistema</h3>

        <div class="space-y-3">
          <div class="flex items-center justify-between">
            <span class="font-semibold text-slate-900">Database</span>
            <span class="text-sm text-green-600 font-semibold">✅ Saudável (45ms)</span>
          </div>
          <div class="flex items-center justify-between">
            <span class="font-semibold text-slate-900">Cache (Redis)</span>
            <span class="text-sm text-green-600 font-semibold">✅ Saudável</span>
          </div>
          <div class="flex items-center justify-between">
            <span class="font-semibold text-slate-900">Queue Workers</span>
            <span class="text-sm text-green-600 font-semibold">✅ 4 Workers Ativos</span>
          </div>
          <div class="flex items-center justify-between">
            <span class="font-semibold text-slate-900">Storage</span>
            <span class="text-sm text-yellow-600 font-semibold">⚠️ 78% Utilizado</span>
          </div>
        </div>
      </div>

      <!-- Maintenance Mode -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6">
        <h3 class="text-lg font-bold text-slate-900 mb-4">🔧 Modo Manutenção</h3>

        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div>
              <p class="font-semibold text-slate-900">Ativar Modo Manutenção</p>
              <p class="text-sm text-slate-600">Bloqueia acesso de usuários normais</p>
            </div>
            <input type="checkbox" class="h-5 w-5" />
          </div>

          <div class="pt-4 border-t border-slate-200">
            <label class="block text-sm font-semibold text-slate-900 mb-2">Mensagem de Manutenção</label>
            <textarea
              class="w-full px-4 py-2 rounded-lg border border-slate-200 focus:outline-none focus:ring-2 focus:ring-primary/40"
              rows="3"
            >Estamos realizando manutenção. Voltaremos em breve!</textarea>
          </div>

          <div class="flex items-end gap-2">
            <button class="px-4 py-2 rounded-lg bg-primary text-white text-sm font-semibold hover:bg-accent transition">
              💾 Salvar
            </button>
            <button class="px-4 py-2 rounded-lg border border-slate-200 text-slate-600 text-sm font-semibold hover:bg-slate-50 transition">
              ↩️ Cancelar
            </button>
          </div>
        </div>
      </div>

      <!-- Backup & Restore -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6">
        <h3 class="text-lg font-bold text-slate-900 mb-4">💾 Backup e Restauração</h3>

        <div class="space-y-3">
          <button class="w-full px-4 py-3 text-left font-semibold text-slate-900 bg-slate-50 hover:bg-slate-100 rounded-lg transition flex items-center justify-between">
            <span>📥 Criar Backup Agora</span>
            <span class="text-slate-600">→</span>
          </button>
          <button class="w-full px-4 py-3 text-left font-semibold text-slate-900 bg-slate-50 hover:bg-slate-100 rounded-lg transition flex items-center justify-between">
            <span>📂 Backups Anteriores</span>
            <span class="text-slate-600">→</span>
          </button>
          <button class="w-full px-4 py-3 text-left font-semibold text-slate-900 bg-slate-50 hover:bg-slate-100 rounded-lg transition flex items-center justify-between">
            <span>⚙️ Agendar Backups</span>
            <span class="text-slate-600">→</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const activeTab = ref<'general' | 'security' | 'email' | 'integrations' | 'maintenance'>('general')
</script>

<style scoped>
/* Admin Settings specific styles */
</style>
