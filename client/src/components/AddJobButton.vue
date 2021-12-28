<script lang="ts" setup>
const { t } = useI18n()

const show = ref(false)
</script>

<template>
  <Modal v-model="show" />
  <button class="btn" @click="show = true">
    <carbon-add /> {{ t('jobs.add.button') }}
  </button>
</template>
