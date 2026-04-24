import z from "zod";
import { BitTypeSchema } from "./bits";

export const MockDataSchema = z.object({
  bit_id: z.string(),
  value: z.string(),
  type: BitTypeSchema
})

export const MockSchema = z.object({
  mock_id: z.string(),
  register_id: z.string(),
  datas: z.array(MockDataSchema)
});

export type MockDataType = z.infer<typeof MockDataSchema>
export type MockType = z.infer<typeof MockSchema>

export const emptyMockData: MockType[] = [
  {
    mock_id: "",
    register_id: "",
    datas: [{
      bit_id: "",
      value: "",
      type: "ReadOnly"
    }]
  }
]
