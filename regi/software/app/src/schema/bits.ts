export enum BitType {
  ReadOnly = "ReadOnly",
  WriteOnly = "WriteOnly",
  ReadWrite = "ReadWrite"
}

export const bitTypes = [
  BitType.ReadOnly,
  BitType.WriteOnly,
  BitType.ReadWrite
]

export interface BitSchema {
  bit_id: number,
  bit_ordinal: number,
  bit_type: BitType,
  reset_val: string,
  description: string,
}

