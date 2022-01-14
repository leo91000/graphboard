<script lang="ts" setup>
import { formatInTimeZone } from 'date-fns-tz'
import { useJobs } from '~/composables'
import { usePreferredTimezone } from '~/composables/timezone'

const { t } = useI18n()

const { jobs, count } = await useJobs()

const { preferredTimezone } = usePreferredTimezone()

function formatDatetime(v?: Date) {
  if (!v) return v
  return formatInTimeZone(v, preferredTimezone.value, t('datetime-format'))
}

const headers = ref<Array<{ value: string; formatter?: (v: any) => any }>>([
  { value: 'id' },
  { value: 'queueName' },
  { value: 'taskIdentifier' },
  { value: 'payload' },
  { value: 'priority' },
  { value: 'runAt', formatter: formatDatetime },
  { value: 'attempts' },
  { value: 'maxAttempts' },
  { value: 'lastError' },
  { value: 'createdAt', formatter: formatDatetime },
  { value: 'updatedAt', formatter: formatDatetime },
  { value: 'key' },
  { value: 'lockedAt' },
  { value: 'lockedBy' },
  { value: 'revision' },
  { value: 'flags' },
])

const formattedJobs = ref<Record<string, unknown>[]>([])

function formatJobs() {
  const result: Record<string, unknown>[] = []
  for (const job of jobs.value) {
    const current: Record<string, unknown> = {}
    for (const header of headers.value) {
      // @ts-ignore
      if (!header.formatter) current[header.value] = job[header.value] ?? undefined
      // @ts-ignore
      else current[header.value] = header.formatter(job[header.value] ?? undefined)
    }
    result.push(current)
  }
  formattedJobs.value = result
}

watch([preferredTimezone, headers, jobs], formatJobs, {
  immediate: true,
})
</script>

<template>
  <div class="flex flex-col">
    <div class="-my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
      <div class="py-2 align-middle inline-block min-w-full sm:px-6 lg:px-8">
        <div class="shadow overflow-hidden border-b border-gray-200 dark:border-gray-800 sm:rounded-lg">
          <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-800">
            <thead class="bg-gray-50 dark:bg-dark-600">
              <tr>
                <th v-for="(header, i) in headers" :key="`TableHeader[${i}]`" scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-50 uppercase tracking-wider">
                  {{ t(`jobs.table.headers.${header.value}`) }}
                </th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(job, i) in formattedJobs" :key="`TableRow[${i}]`" class="odd:bg-white even:bg-gray-50 dark:odd:bg-dark-600 dark:even:bg-dark-400 hover:bg-gray-100 hover:dark:bg-dark-300">
                <td v-for="(header, j) in headers" :key="`TableCell[${i}:${j}]`" class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-gray-200">
                  {{ job[header.value] }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>
