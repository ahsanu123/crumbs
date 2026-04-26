import z from "zod";

export const CombinedRegisterId = z.object({
  register_id: z.number().int(),
  ordinal: z.number().int()
})

export const CombinedRegisterData = z.object({
  combined_id: z.number().int(),
  registers: z.array(CombinedRegisterId)
});

export const CombinedRegistersSchema = z.array(CombinedRegisterData)

export type CombinedRegistersType = z.infer<typeof CombinedRegisterData>
