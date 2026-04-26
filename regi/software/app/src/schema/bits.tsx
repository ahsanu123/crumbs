import z from "zod";

export const bitTypes = [
  "ReadOnly",
  "WriteOnly",
  "ReadWrite",
]
export const BitTypeSchema = z.enum([
  "ReadOnly",
  "WriteOnly",
  "ReadWrite",
]);

export const BitSchema = z.object({
  bit_id: z.number().int(),
  bit_ordinal: z.number().int().min(0).max(31),
  bit_type: BitTypeSchema,
  reset_val: z.string(),
  description: z.string()
});

export type BitType = z.infer<typeof BitTypeSchema>;
