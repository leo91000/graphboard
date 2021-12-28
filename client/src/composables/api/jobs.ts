import type { AxiosInstance } from 'axios'
import type { MaybeRef } from '@vueuse/core'
import { getApiClient } from '~/composables'

interface GetJobParams {
  pagination?: {
    itemsPerPage?: number
    page?: number
  }
  order?: {
    field?: 'taskIdentifier'
    direction?: 'asc' | 'desc'
  }
  filters?: {
    taskIdentifier?: string
  }
}

interface Job {
  id: number
  queueName: string | null
  taskIdentifier: string
  payload: unknown
  priority: number
  run_at: string
  attempts: number
  maxAttempts: number
  lastError: string | null
  createdAt: string
  updatedAt: string
  key: string | null
  lockedAt: string | null
  lockedBy: string | null
  revision: number
  flags: unknown
}

type GetJobResponse = { jobs: Job[]; count: number }

export async function getJobs({ client = getApiClient(), params }: { client?: AxiosInstance; params: MaybeRef<GetJobParams> }): Promise<GetJobResponse> {
  const { data } = await client.get('/jobs', {
    params: unref(params),
  })
  return data
}

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

  return { jobs, count, fetchJobs, params, next, previous }
}
