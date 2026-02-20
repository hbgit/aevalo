<template>
  <teleport to="body">
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm p-4">
      <div class="bg-gradient-to-br from-slate-800 to-slate-900 rounded-2xl border border-slate-700 max-w-lg w-full max-h-[90vh] overflow-y-auto shadow-2xl">
        <!-- Header -->
        <div class="sticky top-0 bg-slate-800/90 backdrop-blur p-6 border-b border-slate-700">
          <div class="flex items-center justify-between">
            <h2 class="text-2xl font-bold text-white flex items-center gap-2">
              🔗 Compartilhar Avaliação
            </h2>
            <button
              @click="$emit('close')"
              class="text-slate-400 hover:text-white transition"
            >
              ✕
            </button>
          </div>
        </div>

        <!-- Content -->
        <div class="p-6 space-y-6">
          <!-- Public Link -->
          <div class="space-y-2">
            <label class="block text-sm font-semibold text-slate-200">Link Público</label>
            <div class="flex gap-2">
              <input
                :value="publicLink"
                type="text"
                readonly
                class="flex-1 px-4 py-3 rounded-lg bg-slate-700 border border-slate-600 text-white text-sm"
              />
              <button
                @click="copyToClipboard(publicLink)"
                class="px-4 py-3 rounded-lg bg-purple-600 text-white hover:bg-purple-700 transition flex items-center gap-2"
                :class="[copied ? 'bg-green-600' : '']"
              >
                {{ copied ? '✓ Copiado' : '📋 Copiar' }}
              </button>
            </div>
          </div>

          <!-- QR Code -->
          <div class="space-y-3">
            <label class="block text-sm font-semibold text-slate-200">Código QR</label>
            <div class="flex justify-center p-4 bg-slate-700/50 rounded-lg border border-slate-600">
              <div
                class="w-48 h-48 bg-white p-4 rounded-lg flex items-center justify-center"
                v-html="qrCode"
              ></div>
            </div>
            <div class="flex gap-2 justify-center">
              <button
                @click="downloadQR('png')"
                class="px-4 py-2 rounded-lg bg-slate-700 text-slate-200 hover:bg-slate-600 transition text-sm"
              >
                ⬇️ PNG
              </button>
              <button
                @click="downloadQR('svg')"
                class="px-4 py-2 rounded-lg bg-slate-700 text-slate-200 hover:bg-slate-600 transition text-sm"
              >
                ⬇️ SVG
              </button>
            </div>
          </div>

          <!-- Social Share -->
          <div class="space-y-3">
            <label class="block text-sm font-semibold text-slate-200">Compartilhar via</label>
            <div class="grid grid-cols-2 gap-2">
              <button
                @click="shareVia('email')"
                class="px-4 py-2 rounded-lg bg-slate-700 text-slate-200 hover:bg-slate-600 transition text-sm flex items-center gap-2 justify-center"
              >
                📧 Email
              </button>
              <button
                @click="shareVia('twitter')"
                class="px-4 py-2 rounded-lg bg-slate-700 text-slate-200 hover:bg-slate-600 transition text-sm flex items-center gap-2 justify-center"
              >
                🐦 Twitter
              </button>
              <button
                @click="shareVia('linkedin')"
                class="px-4 py-2 rounded-lg bg-slate-700 text-slate-200 hover:bg-slate-600 transition text-sm flex items-center gap-2 justify-center"
              >
                💼 LinkedIn
              </button>
              <button
                @click="shareVia('whatsapp')"
                class="px-4 py-2 rounded-lg bg-slate-700 text-slate-200 hover:bg-slate-600 transition text-sm flex items-center gap-2 justify-center"
              >
                💬 WhatsApp
              </button>
            </div>
          </div>

          <!-- Embed Code -->
          <div class="space-y-2">
            <label class="block text-sm font-semibold text-slate-200">Código de Incorporação</label>
            <div class="flex gap-2">
              <textarea
                :value="embedCode"
                readonly
                rows="3"
                class="flex-1 px-4 py-3 rounded-lg bg-slate-700 border border-slate-600 text-white text-xs font-mono resize-none"
              />
              <button
                @click="copyToClipboard(embedCode)"
                class="px-4 rounded-lg bg-purple-600 text-white hover:bg-purple-700 transition flex items-center gap-2"
              >
                📋
              </button>
            </div>
          </div>

          <!-- Email Modal -->
          <div v-if="showEmailModal" class="fixed inset-0 z-60 flex items-center justify-center bg-black/50 backdrop-blur-sm p-4">
            <div class="bg-slate-800 rounded-xl border border-slate-700 max-w-md w-full p-6 space-y-4">
              <h3 class="text-xl font-bold text-white">Enviar Convites por Email</h3>

              <div class="space-y-2">
                <label class="block text-sm text-slate-200">Para (e-mails separados por vírgula)</label>
                <input
                  v-model="emailAddresses"
                  type="text"
                  placeholder="user1@example.com, user2@example.com"
                  class="w-full px-4 py-2 rounded-lg bg-slate-700 border border-slate-600 text-white placeholder-slate-400 focus:border-purple-500 focus:outline-none"
                />
              </div>

              <div class="space-y-2">
                <label class="block text-sm text-slate-200">Assunto</label>
                <input
                  v-model="emailSubject"
                  type="text"
                  class="w-full px-4 py-2 rounded-lg bg-slate-700 border border-slate-600 text-white focus:border-purple-500 focus:outline-none"
                />
              </div>

              <div class="space-y-2">
                <label class="block text-sm text-slate-200">Mensagem</label>
                <textarea
                  v-model="emailMessage"
                  rows="4"
                  class="w-full px-4 py-2 rounded-lg bg-slate-700 border border-slate-600 text-white focus:border-purple-500 focus:outline-none resize-none"
                />
              </div>

              <div class="flex gap-2 justify-end">
                <button
                  @click="showEmailModal = false"
                  class="px-4 py-2 rounded-lg bg-slate-700 text-slate-200 hover:bg-slate-600 transition"
                >
                  Cancelar
                </button>
                <button
                  @click="sendEmails"
                  class="px-4 py-2 rounded-lg bg-purple-600 text-white hover:bg-purple-700 transition"
                >
                  Enviar
                </button>
              </div>
            </div>
          </div>
        </div>

        <!-- Footer -->
        <div class="sticky bottom-0 bg-slate-800/90 backdrop-blur p-6 border-t border-slate-700 flex gap-3">
          <button
            @click="$emit('close')"
            class="flex-1 px-4 py-3 rounded-lg bg-slate-700 text-white hover:bg-slate-600 transition"
          >
            Fechar
          </button>
        </div>
      </div>
    </div>
  </teleport>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

interface Props {
  evaluationId: string
  evaluationTitle: string
}

const props = defineProps<Props>()
defineEmits<{
  close: []
  shared: [method: string]
}>()

const copied = ref(false)
const showEmailModal = ref(false)
const emailAddresses = ref('')
const emailSubject = ref(`Participe da avaliação: ${props.evaluationTitle}`)
const emailMessage = ref('Convidamos você a participar de uma avaliação importante. Clique no link abaixo para começar:')

const publicLink = computed(() => {
  return `${window.location.origin}/e/${props.evaluationId}`
})

const qrCode = computed(() => {
  // Placeholder for QR code generation
  // In production, use a library like qrcode.vue
  return '<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200" viewBox="0 0 200 200"><rect width="200" height="200" fill="white"/><text x="100" y="100" text-anchor="middle" font-size="12" fill="black">QR Code</text></svg>'
})

const embedCode = computed(() => {
  return `<iframe src="${publicLink.value}" width="100%" height="600px" frameborder="0" allow="geolocation"></iframe>`
})

const copyToClipboard = async (text: string) => {
  try {
    await navigator.clipboard.writeText(text)
    copied.value = true
    setTimeout(() => {
      copied.value = false
    }, 2000)
  } catch (err) {
    console.error('Erro ao copiar:', err)
  }
}

const downloadQR = (format: 'png' | 'svg') => {
  // Implement QR code download
  console.log(`Downloading QR code as ${format}`)
}

const shareVia = (method: string) => {
  if (method === 'email') {
    showEmailModal.value = true
    return
  }

  const text = `Participe da avaliação: ${props.evaluationTitle}`
  const url = publicLink.value

  const urls: Record<string, string> = {
    twitter: `https://twitter.com/intent/tweet?text=${encodeURIComponent(text)}&url=${encodeURIComponent(url)}`,
    linkedin: `https://www.linkedin.com/sharing/share-offsite/?url=${encodeURIComponent(url)}`,
    whatsapp: `https://wa.me/?text=${encodeURIComponent(text + ' ' + url)}`
  }

  if (urls[method]) {
    window.open(urls[method], '_blank', 'width=600,height=400')
  }
}

const sendEmails = async () => {
  try {
    const emails = emailAddresses.value
      .split(',')
      .map(e => e.trim())
      .filter(e => e)

    if (emails.length === 0) {
      alert('Por favor, insira pelo menos um email')
      return
    }

    // API call to send emails
    const response = await fetch('/api/evaluations/share', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        evaluationId: props.evaluationId,
        emails,
        subject: emailSubject.value,
        message: emailMessage.value,
        link: publicLink.value
      })
    })

    if (!response.ok) throw new Error('Erro ao enviar emails')

    showEmailModal.value = false
    alert('Convites enviados com sucesso!')
  } catch (err) {
    alert('Erro ao enviar emails: ' + (err instanceof Error ? err.message : 'Tente novamente'))
  }
}
</script>
