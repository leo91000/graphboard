<script lang="ts" setup>
import AddJobForm from '~/components/AddJobForm.vue'

const { t } = useI18n()

const show = ref(false)
</script>

<template>
  <Modal v-model="show">
    <template #title>
      <h1>{{ t('jobs.forms.add.title') }}</h1>
    </template>
    <AddJobForm @cancel="show = false" />
  </Modal>
  <button class="btn" @click="show = true">
    <carbon-add /> {{ t('jobs.add.button') }}
  </button>
</template>
