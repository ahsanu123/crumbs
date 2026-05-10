export enum InterpreterType {
  Register = "register",
  CombinedRegister = "combinedRegister"
}
export interface InterpreterRegisterBitSchema {
  register_id: number,
  bit_ids: number[]
}

export interface InterpreterSchema {
  interpreter_id: number,
  combined_id?: number,
  type: InterpreterType,
  name: string,
  description: string,
  registers: InterpreterRegisterBitSchema[],
  formula: string,
}

