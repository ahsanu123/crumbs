export interface InterpreterRegisterBitSchema {
  register_id: number,
  bit_ids: number[]
}

export interface InterpreterSchema {
  interpreter_id: number,
  name: string,
  description: string,
  registers: InterpreterRegisterBitSchema[],
  formula: string,
}

