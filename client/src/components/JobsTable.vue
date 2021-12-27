<script lang="ts" setup>
import { useJobs } from '~/composables'

const { jobs, count } = await useJobs()
</script>

<template>
  <div>
    <h1>Jobs :</h1>
    <div>{{ jobs }} : {{ count }}</div>
  </div>
</template>
