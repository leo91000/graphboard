<script lang="ts" setup>
import { isDark, toggleDark } from '~/composables'

const { t } = useI18n()
</script>

<template>
  <div class="text-white w-full px-4 py-2 bg-indigo-700 flex flex-row content-center justify-between items-center">
    <div class="flex flex-row items-center justify-center">
      {{ t('app.name') }}
    </div>
    <Clock />
    <div class="flex flex-row items-center justify-center">
      <LocaleDropdown class="mr-2.6" />
      <TimezoneDropdown class="mr-2" />
      <button class="flex flex-row items-center justify-center icon-btn" :title="t('button.toggle_dark')" @click="toggleDark()">
        <carbon-moon v-if="isDark" class="h-7 w-7" />
        <carbon-sun v-else class="h-7 w-7" />
      </button>
    </div>
  </div>
</template>
