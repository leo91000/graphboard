import { once } from 'lodash'
import axios from 'axios'

export const getApiClient = once(() => axios.create({
  baseURL: '/api',
  timeout: 2000,
}))
