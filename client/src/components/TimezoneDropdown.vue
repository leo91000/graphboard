<script lang="ts" setup>
import { Menu, MenuButton, MenuItem, MenuItems } from '@headlessui/vue'
import type { Timezone } from '~/composables/timezone'
import { usePreferredTimezone } from '~/composables/timezone'

const { preferredTimezone, getSupportedTimezones } = usePreferredTimezone()

const emit = defineEmits<{
  (event: 'input', value: Timezone): void
}>()

const { t } = useI18n()
</script>

<template>
  <Menu as="div" class="relative inline-block text-left">
    <div>
      <MenuButton
        class="inline-flex justify-center w-full rounded-full shadow-sm px-1 mt-1.6 text-sm font-medium icon-btn"
      >
        <carbon-time class="h-7 w-7" aria-hidden="true" />
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
      <MenuItems
        class="h-80vh w-80 scrollbar-thin scrollbar-thumb-indigo-700 hover:scrollbar-thumb-indigo-700 scrollbar-track-blue-300 overflow-y-scroll origin-top-right absolute right-0 mt-2 rounded-md shadow-lg bg-white dark:bg-dark-500 ring-1 ring-black ring-opacity-5 focus:outline-none"
      >
        <div class="py-1">
          <MenuItem v-for="timezone in getSupportedTimezones()" v-slot="{ active }" :key="`TimezoneItem:${timezone}`" class="cursor-pointer" @click.stop="preferredTimezone = timezone">
            <span :class="[active ? 'bg-indigo-500 text-white' : preferredTimezone === timezone ? 'bg-indigo-700 text-white' : 'text-gray-700 dark:text-gray-50', 'block px-4 py-2 text-sm']">{{ t(`timezones.${timezone}`) }}</span>
          </MenuItem>
        </div>
      </MenuItems>
    </transition>
  </Menu>
</template>
