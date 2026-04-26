import z from "zod";
import { BitTypeSchema } from "./bits";

export const MockDataSchema = z.object({
  bit_id: z.number().int(),
  value: z.string(),
  type: BitTypeSchema
})

export const MockSchema = z.object({
  mock_id: z.number().int(),
  register_id: z.number().int(),
  datas: z.array(MockDataSchema)
});

export type MockDataType = z.infer<typeof MockDataSchema>
export type MockType = z.infer<typeof MockSchema>

export const emptyMockData: MockType[] = [
  {
    mock_id: 0,
    register_id: 0,
    datas: [{
      bit_id: 0,
      value: "",
      type: "ReadOnly"
    }]
  }
]
