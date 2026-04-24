import { create, all } from 'mathjs'

export const math = create(all)

export function toHex(value: string) {
  return math.evaluate(`hex(${value})`)
}

export function toBin(value: string) {
  return math.evaluate(`bin(${value})`)
}

export interface EquationResult {
  isCompiled: boolean,
  errorMessage?: string,
  result?: any
}

export function evaluateEquation(equation: string): EquationResult {
  try {
    const result = math.evaluate(equation);

    return {
      isCompiled: true,
      result
    }

  } catch (error) {
    return {
      isCompiled: true,
      errorMessage: JSON.stringify(error)
    }
  }
}

