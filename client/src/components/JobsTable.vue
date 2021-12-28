<script lang="ts" setup>
import { useJobs } from '~/composables'

const { t } = useI18n()

const { jobs, count } = await useJobs()

const headers = ref([
  'id',
  'queueName',
  'taskIdentifier',
  'payload',
  'priority',
  'run_at',
  'attempts',
  'maxAttempts',
  'lastError',
  'createdAt',
  'updatedAt',
  'key',
  'lockedAt',
  'lockedBy',
  'revision',
  'flags',
])
</script>

<template>
  <div>
    <div class="table w-full">
      <div class="table-caption">
        {{ t('jobs.table.name', { count }) }}
      </div>
      <div class="table-header-group">
        <div class="table-row">
          <div v-for="(header, i) in headers" :key="`TableHeader${i}`" class="table-cell">
            {{ t(`jobs.table.headers.${header}`) }}
          </div>
        </div>
      </div>
      <div v-for="job in jobs" :key="`TableRow${job.id}`" class="table-row-group">
        <div class="table-row">
          <div v-for="(header, i) in headers" :key="`TableCell${i}`" class="table-cell">
            {{ job[header] }}
          </div>
        </div>
      </div>
      <div class="table-footer-group">
        <div class="table-row">
          <div class="table-cell">
            TBD
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
