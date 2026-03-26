<script setup lang="ts">
import { ref, computed, reactive } from 'vue'
import type { SkillVariable, VariableFormValues } from '@/types'

const props = defineProps<{
  variables: SkillVariable[]
  loading?: boolean
}>()

const emit = defineEmits<{
  (e: 'submit', values: VariableFormValues): void
  (e: 'cancel'): void
}>()

const formValues = reactive<VariableFormValues>({})
const isSubmitting = ref(false)

const isValid = computed(() => {
  return props.variables.every((v) => {
    if (!v.required) return true
    const value = formValues[v.name]
    return value !== undefined && value !== null && value !== ''
  })
})

function initializeForm() {
  for (const v of props.variables) {
    if (v.default !== undefined) {
      formValues[v.name] = v.default
    } else {
      switch (v.type) {
        case 'checkbox':
          formValues[v.name] = false
          break
        case 'number':
          formValues[v.name] = v.validation?.min ?? 0
          break
        default:
          formValues[v.name] = ''
      }
    }
  }
}

initializeForm()

function handleSubmit() {
  if (!isValid.value || isSubmitting.value) return
  isSubmitting.value = true
  emit('submit', { ...formValues })
  isSubmitting.value = false
}

function handleCancel() {
  emit('cancel')
}
</script>

<template>
  <div
    class="fixed inset-0 z-50 flex items-center justify-center backdrop-blur-md"
    style="background: rgba(0, 0, 0, 0.8)"
    @keydown.escape="handleCancel"
  >
    <div
      class="w-full max-w-2xl rounded-2xl border border-gray-700 shadow-glow"
      style="background: #18181b"
    >
      <div class="flex items-center justify-between border-b border-gray-800 px-6 py-4">
        <h3 class="text-lg font-medium text-white">
          填充变量
        </h3>
        <button
          class="text-gray-400 hover:text-white"
          @click="handleCancel"
        >
          ✕
        </button>
      </div>

      <div class="max-h-[60vh] space-y-6 overflow-y-auto p-6">
        <div
          v-for="v in variables"
          :key="v.name"
          class="py-2"
        >
          <label class="mb-2 block text-sm font-medium">
            <span class="text-white">{{ v.label }}</span>
            <span
              v-if="v.required"
              class="ml-1 text-red-400"
            >*</span>
          </label>

          <template v-if="v.type === 'select'">
            <select
              v-model="formValues[v.name]"
              class="input w-full"
            >
              <option
                value=""
                disabled
              >
                请选择{{ v.label }}
              </option>
              <option
                v-for="opt in v.options"
                :key="opt.value"
                :value="opt.value"
              >
                {{ opt.label }}
              </option>
            </select>
          </template>

          <template v-else-if="v.type === 'textarea'">
            <textarea
              :value="String(formValues[v.name] ?? '')"
              :placeholder="v.placeholder"
              rows="4"
              class="input w-full resize-none"
              @input="(e) => { formValues[v.name] = (e.target as HTMLTextAreaElement).value }"
            />
          </template>

          <template v-else-if="v.type === 'number'">
            <input
              v-model.number="formValues[v.name]"
              type="number"
              :placeholder="v.placeholder"
              :min="v.validation?.min"
              :max="v.validation?.max"
              class="input w-full"
            >
          </template>

          <template v-else-if="v.type === 'date'">
            <input
              v-model="formValues[v.name]"
              type="date"
              class="input w-full"
            >
          </template>

          <template v-else-if="v.type === 'checkbox'">
            <label class="flex items-center gap-2">
              <input
                v-model="formValues[v.name]"
                type="checkbox"
                class="h-5 w-5 rounded bg-gray-700"
              >
              <span class="text-xs text-gray-400">{{ v.label }}</span>
            </label>
          </template>

          <template v-else>
            <input
              v-model="formValues[v.name]"
              type="text"
              :placeholder="v.placeholder"
              class="input w-full"
            >
          </template>
        </div>
      </div>

      <div class="flex justify-end gap-3 border-t border-gray-800 px-6 py-4">
        <button
          class="btn btn-secondary"
          @click="handleCancel"
        >
          取消
        </button>
        <button
          class="btn btn-primary"
          :disabled="!isValid || isSubmitting"
          @click="handleSubmit"
        >
          {{ isSubmitting ? '提交中...' : '确定' }}
        </button>
      </div>
    </div>
  </div>
</template>
