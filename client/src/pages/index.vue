<script setup lang="ts">
const { t } = useI18n()
</script>

<template>
  <div class="w-full px-5">
    <div class="w-full my-4 flex flex-row content-center justify-between items-center">
      <div>Pouet</div>
      <AddJobButton />
    </div>
    <JobsTable />
  </div>
</template>
