<script lang="ts" setup>
const { t } = useI18n()
</script>

<template>
  <main class="text-gray-700 dark:text-gray-200">
    <Header />
    <router-view />
    <Footer class="text-center" />
  </main>
</template>
