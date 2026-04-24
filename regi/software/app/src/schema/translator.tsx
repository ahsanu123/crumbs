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
export const BitPairDataSchema = z.object({
  description: z.string(),
});
const FormulaStringSchema = z.object({
  type: z.literal("Str"),
  value: z.string(),
});

// Formula::KeyPair(HashMap<i32, BitPairData>)
const FormulaKeyPairSchema = z.object({
  type: z.literal("KeyPair"),
  value: z.record(
    z.string(), // TOML/JSON object keys become string
    BitPairDataSchema
  ),
});
const KeyPairRowSchema = z.object({
  key: z.string(),
  description: z.string(),
});

const FormulaSchema = z.object({
  type: z.enum(["Str", "KeyPair"]),
  value: z.string().optional(),
  keypair: z.array(KeyPairRowSchema).optional(),
});
export const TranslatorSchema = z.object({
  translator_id: z.string(),
  translator_type: TranslatorTypeSchema,
  formula: FormulaSchema,
});
