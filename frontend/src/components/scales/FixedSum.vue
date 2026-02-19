<template>
  <div class="space-y-4">
    <!-- Items with inputs -->
    <div class="space-y-3">
      <div v-for="(item, index) in items" :key="index" class="space-y-2">
        <div class="flex items-center justify-between">
          <label class="text-sm font-medium text-slate-200">{{ item.label }}</label>
          <div class="flex items-center gap-2">
            <button
              @click="decrementValue(index)"
              class="px-2 py-1 rounded bg-slate-700 hover:bg-slate-600 transition text-slate-300 text-sm"
              :disabled="item.value === 0"
            >
              −
            </button>
            <input
              type="number"
              :value="item.value"
              @change="(e: any) => updateValue(index, parseInt(e.target.value) || 0)"
              class="w-12 px-2 py-1 rounded bg-slate-700 text-center text-sm font-semibold text-white border border-slate-600 focus:border-purple-500 focus:outline-none"
              min="0"
              max="100"
            />
            <button
              @click="incrementValue(index)"
              class="px-2 py-1 rounded bg-slate-700 hover:bg-slate-600 transition text-slate-300 text-sm"
              :disabled="total === 100 && item.value === items[index].value"
            >
              +
            </button>
            <span class="text-xs text-slate-400 w-8 text-right">{{ item.value }}%</span>
          </div>
        </div>

        <!-- Progress bar -->
        <div class="h-2 bg-slate-700 rounded-full overflow-hidden">
          <div
            class="h-full transition-all duration-200"
            :style="{ width: `${item.value}%` }"
            :class="[
              item.value === 0
                ? 'bg-slate-600'
                : 'bg-gradient-to-r from-orange-500 to-purple-600'
            ]"
          />
        </div>
      </div>
    </div>

    <!-- Total status -->
    <div class="flex items-center justify-between p-3 rounded-lg bg-slate-700/50 border border-slate-600">
      <span class="text-sm font-medium text-slate-300">Total:</span>
      <div class="flex items-center gap-2">
        <span class="text-lg font-bold" :class="statusColor">{{ total }}/100</span>
        <span class="text-lg" :class="statusSymbol">{{ statusIcon }}</span>
      </div>
    </div>

    <!-- Quick action buttons -->
    <div class="flex gap-2">
      <button
        @click="distributeEqually"
        class="flex-1 px-3 py-2 rounded text-sm bg-slate-700 hover:bg-slate-600 transition text-slate-300"
      >
        Distribuir Igualmente
      </button>
      <button
        @click="resetAll"
        class="px-3 py-2 rounded text-sm bg-slate-700 hover:bg-slate-600 transition text-slate-300"
      >
        Limpar
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'

interface Item {
  label: string
  value: number
}

interface Props {
  modelValue?: Record<string, number>
  labels?: string[]
}

interface Emits {
  (e: 'update:modelValue', value: Record<string, number>): void
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: () => ({}),
  labels: () => ['Atributo 1', 'Atributo 2', 'Atributo 3']
})

const emit = defineEmits<Emits>()

const items = ref<Item[]>(
  props.labels.map((label, index) => ({
    label,
    value: props.modelValue[`attr_${index}`] || 0
  }))
)

const total = computed(() => {
  return items.value.reduce((sum, item) => sum + item.value, 0)
})

const statusIcon = computed(() => {
  if (total.value === 100) return '✅'
  if (total.value < 100) return '⚠️'
  return '❌'
})

const statusColor = computed(() => {
  if (total.value === 100) return 'text-green-400'
  if (total.value < 100) return 'text-orange-400'
  return 'text-red-400'
})

const statusSymbol = computed(() => {
  if (total.value === 100) return 'text-green-400'
  if (total.value < 100) return 'text-orange-400'
  return 'text-red-400'
})

const updateValue = (index: number, newValue: number) => {
  const clamped = Math.max(0, Math.min(100, newValue))
  items.value[index].value = clamped
  emitUpdate()
}

const incrementValue = (index: number) => {
  if (total.value < 100) {
    items.value[index].value = Math.min(100, items.value[index].value + 1)
    emitUpdate()
  }
}

const decrementValue = (index: number) => {
  items.value[index].value = Math.max(0, items.value[index].value - 1)
  emitUpdate()
}

const distributeEqually = () => {
  const perItem = Math.floor(100 / items.value.length)
  const remainder = 100 % items.value.length
  items.value.forEach((item, index) => {
    item.value = perItem + (index < remainder ? 1 : 0)
  })
  emitUpdate()
}

const resetAll = () => {
  items.value.forEach(item => {
    item.value = 0
  })
  emitUpdate()
}

const emitUpdate = () => {
  const result: Record<string, number> = {}
  items.value.forEach((item, index) => {
    result[`attr_${index}`] = item.value
  })
  emit('update:modelValue', result)
}
</script>

<style scoped>
input[type='number']::-webkit-outer-spin-button,
input[type='number']::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

input[type='number'] {
  -moz-appearance: textfield;
}
</style>
