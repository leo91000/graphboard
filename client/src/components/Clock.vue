<script lang="ts" setup>
import { formatInTimeZone } from 'date-fns-tz'
import { usePreferredTimezone } from '~/composables/timezone'

const { t } = useI18n()
const { preferredTimezone } = usePreferredTimezone()

const now = ref(Date.now())

useIntervalFn(() => {
  now.value = Date.now()
})

const formattedNow = computed(() => formatInTimeZone(now.value, preferredTimezone.value, t('datetime-format')))
</script>

<template>
  <div class="font-light">
    {{ formattedNow }}
  </div>
</template>
