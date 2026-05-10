export interface MockDataSchema {
  bit_id: number,
  value: string,
}

export interface MockSchema {
  mock_id: number,
  register_id: number,
  interpreter_id: number,
  data: MockDataSchema[]
}

