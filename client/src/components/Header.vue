<script lang="ts" setup>
import { isDark, toggleDark } from '~/composables'

const { t, availableLocales, locale } = useI18n()

const toggleLocales = () => {
  // change to some real logic
  const locales = availableLocales
  locale.value = locales[(locales.indexOf(locale.value) + 1) % locales.length]
}
</script>

<template>
  <div class="w-full px-4 py-3 bg-light-blue-400 dark:bg-light-blue-800 flex flex-row content-center justify-between">
    <div>
      {{ t('app.name') }}
    </div>
    <div class="flex flex-row items-center justify-center">
      <button class="flex flex-row items-center justify-center icon-btn !outline-none" :title="t('button.toggle_dark')" @click="toggleDark()">
        <carbon-moon v-if="isDark" />
        <carbon-sun v-else />
      </button>

      <a class="flex flex-row items-center justify-center icon-btn mx-2" :title="t('button.toggle_langs')" @click="toggleLocales">
        <carbon-language />
      </a>
    </div>
  </div>
</template>
