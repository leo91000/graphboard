<script lang="ts" setup>
import { Menu, MenuButton, MenuItem, MenuItems } from '@headlessui/vue'
import FRFlag from 'flag-icons/flags/4x3/fr.svg?raw'
import GBFlag from 'flag-icons/flags/4x3/gb.svg?raw'

type Locale = 'en' | 'fr'

const { locale } = useI18n()

const localeDropdown: Record<Locale, String | string> = {
  en: GBFlag,
  fr: FRFlag,
}
</script>

<template>
  <Menu as="div" class="relative inline-block text-left">
    <div>
      <MenuButton class="inline-flex justify-center rounded-md shadow-sm text-sm font-medium text-gray-700">
        <div class="w-8 mt-1.5" v-html="localeDropdown[locale] ?? '?'" />
      </MenuButton>
    </div>

    <transition
      enter-active-class="transition ease-out duration-100"
      enter-from-class="transform opacity-0 scale-95"
      enter-to-class="transform opacity-100 scale-100"
      leave-active-class="transition ease-in duration-75"
      leave-from-class="transform opacity-100 scale-100"
      leave-to-class="transform opacity-0 scale-95"
    >
      <MenuItems class="origin-top-right absolute right-0 mt-2 w-12 rounded-md shadow-lg ring-1 ring-black ring-opacity-5 hover:cursor-pointer">
        <div class="bg-gray-50 dark:bg-dark-500">
          <MenuItem v-for="(flag, lang) in localeDropdown" v-slot="{ active }" :key="`LocaleDropdownItem:${lang}`" @click="locale = lang">
            <div class="px-1.5 pt-1.5 pb-0" :class="active ? 'bg-indigo-500 text-white' : locale === lang ? 'bg-indigo-700 text-white' : 'text-gray-700 dark:text-gray-50'" v-html="flag" />
          </MenuItem>
        </div>
      </MenuItems>
    </transition>
  </Menu>
</template>
