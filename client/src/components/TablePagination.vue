<script lang="ts" setup>
const props = defineProps<{
  page: number
  pageCount: number
  count: number
  itemsPerPage: number
}>()

defineEmits<{
  (event: 'next'): void
  (event: 'previous'): void
}>()

const maxPage = computed(() => Math.floor(props.count / props.itemsPerPage) + 1)
const hasNextPage = computed(() => props.page >= maxPage.value)
const hasPreviousPage = computed(() => props.page > 1)
</script>

<template>
  <div class="flex flex-row">
    <button :disabled="!hasPreviousPage" @click="$emit('previous')">
      <carbon-chevron-left class="h-full" />
    </button>
    <div>
      {{ page }} / {{ maxPage }}
    </div>
    <button :disabled="!hasNextPage" @click="$emit('next')">
      <carbon-chevron-right class="h-full" />
    </button>
  </div>
</template>
