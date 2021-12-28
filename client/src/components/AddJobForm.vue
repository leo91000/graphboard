<script lang="ts" setup>
import type { AddJobData } from '~/api/jobs'
import { addJob } from '~/api/jobs'

const job = reactive<AddJobData>({
  taskIdentifier: '',
  payload: null,
  queueName: null,
  runAt: null,
  maxAttempts: null,
  jobKey: null,
  priority: null,
  flags: null,
  jobKeyMode: null,
})

const { t } = useI18n()
</script>

<template>
  <form name="add-job" @submit.prevent="addJob({ job })">
    <label for="task-identifier-input">{{ t('jobs.forms.add.taskIdentifier') }}</label>
    <input id="task-identifier-input" v-model="job.taskIdentifier" type="text">
  </form>
</template>
