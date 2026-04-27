import z from "zod";

export const interpreterTypes = [
  "BitCondition",
  "Formula",
]

export const InterpreterTypeSchema = z.enum([
  "BitCondition",
  "Formula",
]);

const BitStateCondition = z.object({
  value: z.number().int(),
  stateDescription: z.string()
})

const BitConditionSchema = z.object({
  bit_id: z.number().int(),
  name: z.string(),
  stateConditions: z.array(BitStateCondition)
});

export const InterpreterRegisterBitsSchema = z.object({
  register_id: z.number().int(),
  bit_ids: z.array(z.number().int())
})

export const InterpreterSchema = z.object({
  interpreter_id: z.number().int(),
  name: z.string(),
  description: z.string(),
  type: InterpreterTypeSchema,
  registers: z.array(InterpreterRegisterBitsSchema),
  formula: z.optional(z.string()),
  bitConditions: z.optional(z.array(BitConditionSchema)),
});


export type InterpreterType = z.infer<typeof InterpreterSchema>
export type InterpreterRegisterBitsType = z.infer<typeof InterpreterRegisterBitsSchema>
export type InterpreterTypeEnum = z.infer<typeof InterpreterTypeSchema>
export type BitConditionType = z.infer<typeof BitConditionSchema>
