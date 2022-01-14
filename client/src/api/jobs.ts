import type { AxiosInstance } from 'axios'
import type { MaybeRef } from '@vueuse/core'
import { getApiClient } from '~/api/index'
import type { Unparsed, UnparsedKeys } from '~/api/utils/parser'
import { parse } from '~/api/utils/parser'

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
  runAt: Date
  attempts: number
  maxAttempts: number
  lastError: string | null
  createdAt: Date
  updatedAt: Date
  key: string | null
  lockedAt: Date | null
  lockedBy: Date | null
  revision: number
  flags: unknown
}

type GetJobResponse = { jobs: Job[]; count: number }

const getJobParseKeys: UnparsedKeys<Job>[] = ['runAt', 'createdAt', 'updatedAt', 'lockedAt']
export async function getJobs({
  params,
  client = getApiClient(),
}: { params: MaybeRef<GetJobParams>; client?: AxiosInstance }): Promise<GetJobResponse> {
  const { data } = await client.get<{ jobs: Unparsed<Job>[]; count: number }>('/jobs', {
    params: unref(params),
  })
  return {
    ...data,
    jobs: data.jobs.map(job => parse(job, getJobParseKeys)),
  }
}

export interface AddJobData {
  taskIdentifier: string
  payload?: unknown
  queueName?: string | null
  runAt?: Date | null
  maxAttempts?: number | null
  jobKey?: string | null
  priority?: number | null
  flags?: string | null
  jobKeyMode?: string | null
}

export async function addJob({ job, client = getApiClient() }: { job: MaybeRef<AddJobData>; client?: AxiosInstance }): Promise<Job> {
  const { data } = await client.post('/jobs', unref(job))
  return data
}
