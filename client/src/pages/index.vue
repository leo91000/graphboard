<script setup lang="ts">
const { t } = useI18n()
</script>

<template>
  <div>
    <AddJobButton />
    <suspense>
      <template #default>
        <JobsTable />
      </template>
      <template #fallback>
        <h1>Loading...</h1>
      </template>
    </suspense>
  </div>
</template>
