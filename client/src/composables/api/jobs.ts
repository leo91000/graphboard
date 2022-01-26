import type { AxiosInstance } from 'axios'
import type { MaybeRef } from '@vueuse/core'
import type { GetJobParams, Job } from '~/api/jobs'
import { getJobs } from '~/api/jobs'

export function useJobs({ client, params: rawParams = {} }: { client?: AxiosInstance; params?: MaybeRef<GetJobParams> } = {}) {
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
    immediate: true,
  })

  const page = computed({
    get: () => params.value.pagination?.page ?? 1,
    set: (page) => {
      params.value.pagination = {
        ...params.value.pagination,
        page,
      }
    },
  })

  const itemsPerPage = computed({
    get: () => params.value.pagination?.itemsPerPage ?? 20,
    set: (itemsPerPage) => {
      params.value.pagination = {
        ...params.value.pagination,
        itemsPerPage,
      }
    },
  })

  const hasPreviousPage = computed(() => page.value > 1)
  const hasNextPage = computed(() => Math.floor(count.value / itemsPerPage.value) + 1 > page.value)

  function next() {
    if (hasNextPage.value)
      page.value++
  }

  function previous() {
    if (hasPreviousPage.value)
      page.value--
  }

  return { jobs, count, fetchJobs, params, page, itemsPerPage, next, previous, hasNextPage, hasPreviousPage, loading }
}
