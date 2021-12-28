import type { AxiosInstance } from 'axios'
import type { MaybeRef } from '@vueuse/core'
import type { GetJobParams, Job } from '~/api/jobs'
import { getJobs } from '~/api/jobs'

export async function useJobs({ client, params: rawParams = {} }: { client?: AxiosInstance; params?: MaybeRef<GetJobParams> } = {}) {
  const loading = ref(false)
  const jobs = ref<Job[]>([])
  const count = ref(0)

  const params = isRef(rawParams) ? rawParams : ref(rawParams)

  async function fetchJobs() {
    loading.value = true
    try {
      const data = await getJobs({ client, params })
      jobs.value = data.jobs
      count.value = data.count
    }
    finally {
      loading.value = false
    }
  }

  watch(params, async() => fetchJobs(), {
    deep: true,
  })

  await fetchJobs()

  const hasPreviousPage = computed(() => (params.value.pagination?.page ?? 1) > 1)
  const hasNextPage = computed(() => (params.value.pagination?.itemsPerPage ?? 20) <= jobs.value.length)

  function next() {
    if (hasNextPage.value) {
      params.value.pagination = {
        ...params.value.pagination,
        page: (params.value.pagination?.page ?? 1) + 1,
      }
    }
  }

  function previous() {
    if (hasPreviousPage.value) {
      params.value.pagination = {
        ...params.value.pagination,
        page: (params.value.pagination?.page ?? 1) - 1,
      }
    }
  }

  return { jobs, count, fetchJobs, params, next, previous, hasNextPage, hasPreviousPage }
}
