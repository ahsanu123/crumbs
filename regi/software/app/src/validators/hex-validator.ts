import * as yup from 'yup'

// FIXME: choose better name

export enum ResetValueType {
  Int,
  Hex,
  Binary
}

export interface IResetValue {
  isValid: boolean
  valType: ResetValueType | undefined
  value: string
}

export const resetValCaster = yup.mixed().transform((_, originalVal): IResetValue | undefined => {
  const v = String(originalVal).trim()

  const isHex = /^0x[0-9a-f]+$/i.test(v);
  const isBinary = /^0b[01]+$/i.test(v);
  const isNumber = /^-?\d+(\.\d+)?$/.test(v);

  if (isHex) return {
    isValid: true,
    valType: ResetValueType.Hex,
    value: v
  }

  if (isBinary) return {
    isValid: true,
    valType: ResetValueType.Hex,
    value: v
  }

  if (isNumber) return {
    isValid: true,
    valType: ResetValueType.Hex,
    value: v
  }

  return undefined
})

