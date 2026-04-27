import z from "zod";

export const MockDataSchema = z.object({
  bit_id: z.number().int(),
  value: z.string(),
})

export const MockSchema = z.object({
  mock_id: z.number().int(),
  register_id: z.number().int(),
  interpreter_id: z.number().int(),
  datas: z.array(MockDataSchema)
});

export type MockDataType = z.infer<typeof MockDataSchema>
export type MockType = z.infer<typeof MockSchema>
