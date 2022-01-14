<script lang="ts" setup>
import { addJob } from '~/api/jobs'
import { usePreferredTimezone } from '~/composables/timezone'

defineEmits<{
  (event: 'cancel'): void
}>()

interface AddJobFormModel {
  taskIdentifier: string
  payload: string
  queueName: string
  runAt: string
  maxAttempts: string
  jobKey: string
  priority: string
  flags: string
  jobKeyMode: string
}

const job = reactive<AddJobFormModel>({
  taskIdentifier: '',
  payload: '',
  queueName: '',
  runAt: '',
  maxAttempts: '',
  jobKey: '',
  priority: '',
  flags: '',
  jobKeyMode: '',
})

const { preferredTimezone } = usePreferredTimezone()

async function submit() {
  return addJob({
    job: {
      taskIdentifier: job.taskIdentifier,
      payload: job.payload || null,
      runAt: job.runAt ? new Date(`${job.runAt}:00${preferredTimezone}`) : null,
      maxAttempts: job.maxAttempts ? parseInt(job.maxAttempts) : null,
      jobKey: job.jobKey || null,
      priority: job.priority ? parseInt(job.priority) : null,
      flags: job.flags || null,
      jobKeyMode: job.jobKeyMode || null,
    },
  })
}

const { t } = useI18n()
</script>

<template>
  <form name="add-job" @submit.prevent="submit">
    <div class="space-y-4 sm:space-y-3">
      <div class="sm:grid sm:grid-cols-3 sm:gap-4 sm:items-start sm:border-t sm:border-gray-200 sm:dark:border-gray-700 sm:pt-3">
        <label for="task-identifier" class="block text-sm font-medium text-gray-700 dark:text-gray-200 sm:mt-px sm:pt-2">
          {{ t('jobs.forms.add.taskIdentifier') }}
        </label>
        <div class="mt-1 sm:mt-0 sm:col-span-2">
          <input
            id="task-identifier"
            v-model="job.taskIdentifier"
            type="text"
            name="first-name"
            autocomplete="on"
            class="dark:bg-dark-200 dark:text-white max-w-lg block w-full shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:max-w-xs sm:text-sm border-gray-300 dark:border-gray-500 rounded-md"
          >
        </div>
      </div>

      <div class="sm:grid sm:grid-cols-3 sm:gap-4 sm:items-start sm:border-t sm:border-gray-200 sm:dark:border-gray-700 sm:pt-3">
        <label for="payload" class="block text-sm font-medium text-gray-700 dark:text-gray-200 sm:mt-px sm:pt-2">
          {{ t('jobs.forms.add.payload') }}
        </label>
        <div class="mt-1 sm:mt-0 sm:col-span-2">
          <input
            id="payload"
            v-model="job.payload"
            type="text"
            name="payload"
            autocomplete="off"
            class="dark:bg-dark-200 dark:text-white max-w-lg block w-full shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:max-w-xs sm:text-sm border-gray-300 dark:border-gray-500 rounded-md"
          >
        </div>
      </div>

      <div class="sm:grid sm:grid-cols-3 sm:gap-4 sm:items-start sm:border-t sm:border-gray-200 sm:dark:border-gray-700 sm:pt-3">
        <label for="queue-name" class="block text-sm font-medium text-gray-700 dark:text-gray-200 sm:mt-px sm:pt-2">
          {{ t('jobs.forms.add.queueName') }}
        </label>
        <div class="mt-1 sm:mt-0 sm:col-span-2">
          <input
            id="queue-name"
            v-model="job.queueName"
            name="queue-name"
            type="text"
            autocomplete="on"
            class="dark:bg-dark-200 dark:text-white block max-w-lg w-full shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm border-gray-300 dark:border-gray-500 rounded-md"
          >
        </div>
      </div>

      <div class="sm:grid sm:grid-cols-3 sm:gap-4 sm:items-start sm:border-t sm:border-gray-200 sm:dark:border-gray-700 sm:pt-3">
        <label for="run-at" class="block text-sm font-medium text-gray-700 dark:text-gray-200 sm:mt-px sm:pt-0 leading-tight">
          {{ t('jobs.forms.add.runAt') }}<br><span class="text-xs font-light text-gray-500 dark:text-gray-300">({{ t('timezones.name') }} : {{ preferredTimezone }})</span>
        </label>
        <div class="mt-1 sm:mt-0 sm:col-span-2">
          <input
            id="run-at"
            v-model="job.runAt"
            name="queue-name"
            type="datetime-local"
            autocomplete="off"
            class="dark:bg-dark-200 dark:text-white block max-w-lg w-full shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm border-gray-300 dark:border-gray-500 rounded-md"
          >
        </div>
      </div>

      <div class="sm:grid sm:grid-cols-3 sm:gap-4 sm:items-start sm:border-t sm:border-gray-200 sm:dark:border-gray-700 sm:pt-3">
        <label for="max-attempts" class="block text-sm font-medium text-gray-700 dark:text-gray-200 sm:mt-px sm:pt-2">
          {{ t('jobs.forms.add.maxAttempts') }}
        </label>
        <div class="mt-1 sm:mt-0 sm:col-span-2">
          <input
            id="max-attempts"
            v-model="job.maxAttempts"
            type="number"
            name="max-attempts"
            autocomplete="on"
            class="dark:bg-dark-200 dark:text-white block max-w-lg w-full shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm border-gray-300 dark:border-gray-500 rounded-md"
          >
        </div>
      </div>

      <div class="sm:grid sm:grid-cols-3 sm:gap-4 sm:items-start sm:border-t sm:border-gray-200 sm:dark:border-gray-700 sm:pt-3">
        <label for="job-key" class="block text-sm font-medium text-gray-700 dark:text-gray-200 sm:mt-px sm:pt-2">
          {{ t('jobs.forms.add.jobKey') }}
        </label>
        <div class="mt-1 sm:mt-0 sm:col-span-2">
          <input
            id="job-key"
            v-model="job.jobKey"
            type="text"
            name="job-key"
            autocomplete="on"
            class="dark:bg-dark-200 dark:text-white max-w-lg block w-full shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:max-w-xs sm:text-sm border-gray-300 dark:border-gray-500 rounded-md"
          >
        </div>
      </div>

      <div class="sm:grid sm:grid-cols-3 sm:gap-4 sm:items-start sm:border-t sm:border-gray-200 sm:dark:border-gray-700 sm:pt-3">
        <label for="priority" class="block text-sm font-medium text-gray-700 dark:text-gray-200 sm:mt-px sm:pt-2">
          {{ t('jobs.forms.add.priority') }}
        </label>
        <div class="mt-1 sm:mt-0 sm:col-span-2">
          <input
            id="priority"
            v-model="job.priority"
            type="number"
            name="priority"
            autocomplete="on"
            class="dark:bg-dark-200 dark:text-white block max-w-lg w-full shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm border-gray-300 dark:border-gray-500 rounded-md"
          >
        </div>
      </div>

      <div class="sm:grid sm:grid-cols-3 sm:gap-4 sm:items-start sm:border-t sm:border-gray-200 sm:dark:border-gray-700 sm:pt-3">
        <label for="flags" class="block text-sm font-medium text-gray-700 dark:text-gray-200 sm:mt-px sm:pt-2">
          {{ t('jobs.forms.add.flags') }}
        </label>
        <div class="mt-1 sm:mt-0 sm:col-span-2">
          <input
            id="flags"
            v-model="job.flags"
            type="text"
            name="flags"
            autocomplete="postal-code"
            class="dark:bg-dark-200 dark:text-white max-w-lg block w-full shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:max-w-xs sm:text-sm border-gray-300 dark:border-gray-500 rounded-md"
          >
        </div>
      </div>

      <div class="sm:grid sm:grid-cols-3 sm:gap-4 sm:items-start sm:border-t sm:border-gray-200 sm:dark:border-gray-700 sm:pt-3">
        <label for="job-key-mode" class="block text-sm font-medium text-gray-700 dark:text-gray-200 sm:mt-px sm:pt-2">
          {{ t('jobs.forms.add.jobKeyMode') }}
        </label>
        <div class="mt-1 sm:mt-0 sm:col-span-2">
          <input
            id="job-key-mode"
            v-model="job.jobKeyMode"
            type="text"
            name="job-key-mode"
            autocomplete="postal-code"
            class="dark:bg-dark-200 dark:text-white max-w-lg block w-full shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:max-w-xs sm:text-sm border-gray-300 dark:border-gray-500 rounded-md"
          >
        </div>
      </div>
    </div>

    <div class="pt-5">
      <div class="flex justify-end">
        <button type="button" class="bg-white py-2 px-4 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500" @click="$emit('cancel')">
          {{ t('jobs.forms.add.cancel') }}
        </button>
        <button type="submit" class="ml-3 inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
          {{ t('jobs.forms.add.save') }}
        </button>
      </div>
    </div>
  </form>
</template>
