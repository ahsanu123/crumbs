import { InterpreterSchema } from "./interpreter";
import { RegisterSchema } from "./register";

export interface RegiSchema {
  name: string,
  version: number,
  description: string,

  registers: RegisterSchema[],
  interpreters: InterpreterSchema[]
}

