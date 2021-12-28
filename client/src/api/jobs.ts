import type { AxiosInstance } from 'axios'
import type { MaybeRef } from '@vueuse/core'
import { getApiClient } from '~/api/index'

export interface GetJobParams {
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

export interface Job {
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

export async function getJobs({
  params,
  client = getApiClient(),
}: { params: MaybeRef<GetJobParams>; client?: AxiosInstance }): Promise<GetJobResponse> {
  const { data } = await client.get('/jobs', {
    params: unref(params),
  })
  return data
}

export interface AddJobData {
  taskIdentifier: string
  payload?: unknown
  queueName?: string | null
  runAt?: Date | null
  maxAttempts?: string | null
  jobKey?: string | null
  priority?: string | null
  flags?: string | null
  jobKeyMode?: string | null
}

export async function addJob({ job, client = getApiClient() }: { job: MaybeRef<AddJobData>; client?: AxiosInstance }): Promise<Job> {
  const { data } = await client.post('/jobs', unref(job))
  return data
}
