import z from "zod";

export const InterpreterRegisterBitsSchema = z.object({
  register_id: z.number().int(),
  bit_ids: z.array(z.number().int())
})

export const InterpreterSchema = z.object({
  interpreter_id: z.number().int(),
  name: z.string(),
  description: z.string(),
  registers: z.array(InterpreterRegisterBitsSchema),
  translator_id: z.number().int(),
});


export type InterpreterType = z.infer<typeof InterpreterSchema>
export type InterpreterRegisterBitsType = z.infer<typeof InterpreterRegisterBitsSchema>

