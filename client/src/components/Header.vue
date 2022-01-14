<script lang="ts" setup>
import { isDark, toggleDark } from '~/composables'
import LocaleDropdown from '~/components/LocaleDropdown.vue'

const { t, availableLocales, locale } = useI18n()

const toggleLocales = () => {
  // change to some real logic
  const locales = availableLocales
  locale.value = locales[(locales.indexOf(locale.value) + 1) % locales.length]
}
</script>

<template>
  <div class="text-white w-full px-4 py-2 bg-indigo-700 flex flex-row content-center justify-between">
    <div class="flex flex-row items-center justify-center">
      {{ t('app.name') }}
    </div>
    <div class="flex flex-row items-center justify-center">
      <LocaleDropdown />
      <button class="flex flex-row items-center justify-center icon-btn ml-3 mr-1.3" :title="t('button.toggle_dark')" @click="toggleDark()">
        <carbon-moon v-if="isDark" class="h-7 w-7" />
        <carbon-sun v-else class="h-7 w-7" />
      </button>
      <TimezoneDropdown />
    </div>
  </div>
</template>
