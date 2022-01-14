<script lang="ts" setup>
import AddJobForm from '~/components/AddJobForm.vue'

const { t } = useI18n()

const show = ref(false)
</script>

<template>
  <Modal v-model="show">
    <template #title>
      <h1 class="dark:text-white">
        {{ t('jobs.forms.add.title') }}
      </h1>
    </template>
    <AddJobForm @cancel="show = false" />
  </Modal>
  <button class="px-4 py-1 rounded bg-indigo-600 text-white cursor-pointer hover:bg-indigo-700 disabled:cursor-default disabled:bg-gray-600 disabled:opacity-50 flex flex-row content-center justify-center items-center" @click="show = true">
    <carbon-add /> {{ t('jobs.add.button') }}
  </button>
</template>
