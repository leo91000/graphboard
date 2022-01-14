import { once } from 'lodash'

export type Timezone =
  '-12:00'
  | '-11:00'
  | '-10:00'
  | '-09:50'
  | '-09:00'
  | '-08:00'
  | '-07:00'
  | '-06:00'
  | '-05:00'
  | '-04:50'
  | '-04:00'
  | '-03:50'
  | '-03:00'
  | '-02:00'
  | '-01:00'
  | '+00:00'
  | '+01:00'
  | '+02:00'
  | '+03:00'
  | '+03:50'
  | '+04:00'
  | '+04:50'
  | '+05:00'
  | '+05:50'
  | '+05:75'
  | '+06:00'
  | '+06:50'
  | '+07:00'
  | '+08:00'
  | '+08:75'
  | '+09:00'
  | '+09:50'
  | '+10:00'
  | '+10:50'
  | '+11:00'
  | '+11:50'
  | '+12:00'
  | '+12:75'
  | '+13:00'
  | '+14:00'

const getSupportedTimezones = once((): Timezone[] => ['-12:00', '-11:00', '-10:00', '-09:50', '-09:00', '-08:00', '-07:00', '-06:00', '-05:00', '-04:50', '-04:00', '-03:50', '-03:00', '-02:00', '-01:00', '+00:00', '+01:00', '+02:00', '+03:00', '+03:50', '+04:00', '+04:50', '+05:00', '+05:50', '+05:75', '+06:00', '+06:50', '+07:00', '+08:00', '+08:75', '+09:00', '+09:50', '+10:00', '+10:50', '+11:00', '+11:50', '+12:00', '+12:75', '+13:00', '+14:00'])

function isTimezoneSupported(v: string): v is Timezone {
  return (getSupportedTimezones() as string[]).includes(v)
}

function getCurrentTimezone(): Timezone {
  const tzOffset = new Date().getTimezoneOffset()
  const absoluteTzOffset = Math.abs(tzOffset)

  const isNegativeTimezone = tzOffset === absoluteTzOffset
  const hourOffset = Math.floor(absoluteTzOffset / 60)
  const minutesOffset = absoluteTzOffset % 60

  const timezone = `${isNegativeTimezone ? '-' : '+'}${hourOffset.toString().padStart(2, '0')}:${minutesOffset.toString().padStart(2, '0')}`

  if (isTimezoneSupported(timezone))
    return timezone

  return '+00:00'
}

const preferredTimezone = useLocalStorage<Timezone>('preferred-timezone', getCurrentTimezone())

export function usePreferredTimezone() {
  return { preferredTimezone, getSupportedTimezones, isTimezoneSupported, getCurrentTimezone }
}
