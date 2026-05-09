export interface CombinedRegisterId {
  register_id: number,
  ordinal: number
}

export interface CombinedRegisterSchema {
  combined_id: number,
  name: string,
  registers: CombinedRegisterId[],
}
