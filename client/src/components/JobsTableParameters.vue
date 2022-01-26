<script lang="ts" setup>
import type { Job } from '~/api/jobs'

const props = defineProps<{
  headers: Array<{ value: keyof Job }>
  selectedHeaders: Array<keyof Job>
  itemsPerPage: number
}>()

defineEmits<{
  (event: 'update:selectedHeaders', value: (typeof props)['selectedHeaders']): void
  (event: 'update:itemsPerPage', value: (typeof props)['itemsPerPage']): void
}>()

const { selectedHeaders: selectedHeadersModel, itemsPerPage: itemsPerPageModel } = useVModels(props)
</script>

<template>
  <div>
    Parameters : {{ selectedHeadersModel }} / {{ itemsPerPageModel }}
  </div>
</template>
