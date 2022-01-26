<script lang="ts" setup>
import { formatInTimeZone } from 'date-fns-tz'
import { useJobs } from '~/composables'
import { usePreferredTimezone } from '~/composables/timezone'
import type { Job } from '~/api/jobs'

const { t } = useI18n()

const { jobs, count, itemsPerPage, page, next, previous, loading } = useJobs()

const { preferredTimezone } = usePreferredTimezone()

function formatDatetime(v?: Date) {
  if (!v) return v
  return formatInTimeZone(v, preferredTimezone.value, t('datetime-format'))
}

const headers = ref<Array<{ value: keyof Job; formatter?: (v: any) => any }>>([
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
const selectedHeaders = ref<Array<keyof Job>>(['queueName', 'taskIdentifier', 'payload', 'priority', 'runAt', 'attempts'])
const formattedHeaders = computed(() => headers.value.filter(h => selectedHeaders.value.includes(h.value)))

const formattedJobs = ref<Record<string, unknown>[]>([])

function formatJobs() {
  const result: Record<string, unknown>[] = []
  for (const job of jobs.value) {
    const current: Record<string, unknown> = {}
    for (const header of formattedHeaders.value) {
      if (!header.formatter) current[header.value] = job[header.value] ?? undefined
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
  <div class="flex flex-col sm:px-6 lg:px-8">
    <JobsTableParameters v-model:selected-headers="selectedHeaders" v-model:items-per-page="itemsPerPage" />
    <div class="overflow-x-auto sm:-mx-6 lg:-mx-8">
      <div class="align-middle inline-block min-w-full">
        <div class="shadow overflow-hidden border-b border-gray-200 dark:border-gray-800 sm:rounded-lg">
          <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-800">
            <thead class="bg-gray-50 dark:bg-dark-600">
              <tr>
                <th v-for="(header, i) in formattedHeaders" :key="`TableHeader[${i}]`" scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-50 uppercase tracking-wider">
                  {{ t(`jobs.table.headers.${header.value}`) }}
                </th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="i in itemsPerPage" :key="`TableRow[${i}]`" :class="{ 'table-loading': loading }" class="odd:bg-white even:bg-gray-50 dark:odd:bg-dark-600 dark:even:bg-dark-400 hover:bg-gray-100 hover:dark:bg-dark-300">
                <td v-if="loading" :colspan="formattedHeaders.length" class="px-4 py-2 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-gray-200 align-center text-center">
                  &nbsp;
                </td>
                <td v-else-if="!formattedJobs[i - 1]" :colspan="formattedHeaders.length" class="px-4 py-2 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-gray-200 align-center text-center">
                  &#8212;
                </td>
                <template v-else>
                  <td v-for="(header, j) in formattedHeaders" :key="`TableCell[${i}:${j}]`" class="px-4 py-2 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-gray-200">
                    {{ formattedJobs[i - 1][header.value] }}
                  </td>
                </template>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
    <div class="my-3 w-full flex flex-row items-center justify-center">
      <TablePagination
        :page="page"
        :count="count"
        :page-count="jobs.length"
        :items-per-page="itemsPerPage"
        @next="next"
        @previous="previous"
      />
    </div>
  </div>
</template>

<style scoped>
.table-loading {
  --light-mode-gradient: rgba(200,200,200,.2);
  background: linear-gradient(90deg, rgba(0,0,0,0) 0%, rgba(0,0,0,0) 23%, var(--light-mode-gradient) 25%, rgba(0,0,0,0) 27%, rgba(0,0,0,0) 48%, var(--light-mode-gradient) 50%, rgba(0,0,0,0) 52%, rgba(0,0,0,0) 73%, var(--light-mode-gradient) 75%, rgba(0,0,0,0) 77%, rgba(0,0,0,0) 100%);
  background-size: 200% 200%;
  animation: gradient 5s linear infinite;
}

.dark .table-loading {
  --dark-mode-gradient: rgba(50,50,50,.2);
  background: linear-gradient(90deg, rgba(0,0,0,0) 0%, rgba(0,0,0,0) 23%, var(--dark-mode-gradient) 25%, rgba(0,0,0,0) 27%, rgba(0,0,0,0) 48%, var(--dark-mode-gradient) 50%, rgba(0,0,0,0) 52%, rgba(0,0,0,0) 73%, var(--dark-mode-gradient) 75%, rgba(0,0,0,0) 77%, rgba(0,0,0,0) 100%);
  background-size: 200% 200%;
  animation: gradient 5s linear infinite;
}

@keyframes gradient {
  0% {
    background-position: 100% 50%;
  }
  100% {
    background-position: 0% 50%;
  }
}
</style>
