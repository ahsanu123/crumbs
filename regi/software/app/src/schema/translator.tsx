import z from "zod";

export const TranslatorTypeSchema = z.enum([
  "Pair",
  "Pair2",
  "Pair3",
  "Pair4",
  "Pair5",
  "Pair6",
  "Pair7",
  "Pair8",
  "Pair9",
  "Pair10",
  "Pair11",
  "Pair12",
  "Pair13",
  "Pair14",
  "Pair15",
  "Pair16",
  "Formula",
]);

const BitAndDescription = z.object({
  bit_id: z.number().int(),
  description: z.string(),
});

const FormulaSchema = z.object({
  type: z.enum(["Str", "KeyPair"]),
  value: z.string().optional(),
  keypair: z.array(BitAndDescription).optional(),
});

export const TranslatorSchema = z.object({
  translator_id: z.number().int(),
  translator_type: TranslatorTypeSchema,
  formula: z.nullable(FormulaSchema),
  keyPairs: z.nullable(z.array(BitAndDescription)),
});

export type TranslatorType = z.infer<typeof TranslatorSchema>
