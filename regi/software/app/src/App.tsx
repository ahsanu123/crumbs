import { AutoField, AutoForm, ListField, ListItemField, LongTextField } from 'uniforms-mui';
import { ZodBridge } from 'uniforms-bridge-zod';
import { z } from 'zod';
import { useField } from 'uniforms';

export const BitTypeSchema = z.enum([
  "ReadOnly",
  "WriteOnly",
  "ReadWrite",
]);

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

/* -------------------------
   CORE TYPES
------------------------- */

export const BitSchema = z.object({
  bit_id: z.number().int(),
  bit_ordinal: z.number().int().min(0).max(31),
  bit_type: BitTypeSchema,
  reset_val: z.number().int().nonnegative(),
  description: z.string().describe("textarea"),
});

export const RegisterSchema = z.object({
  register_id: z.number().int(),
  name: z.string(),
  description: z.string().uniforms({ component: LongTextField }),
  address: z.number().int().nonnegative(),
  bits: z.array(BitSchema),
});

export const InterpreterSchema = z.object({
  interpreter_id: z.number().int(),
  name: z.string(),
  description: z.string(),
  register_ids: z.array(z.number().int()),
  translator_id: z.number().int(),
});

export const BitPairDataSchema = z.object({
  description: z.string(),
});

/* -------------------------
   FORMULA UNION
------------------------- */

// Formula::Str(String)
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

// export const FormulaSchema = z.union([
//   FormulaStringSchema,
//   FormulaKeyPairSchema,
// ]);
const KeyPairRowSchema = z.object({
  key: z.string(),
  description: z.string(),
});

const FormulaSchema = z.object({
  type: z.enum(["Str", "KeyPair"]),
  value: z.string().optional(),
  keypair: z.array(KeyPairRowSchema).optional(),
});

/* -------------------------
   TRANSLATOR
------------------------- */

export const TranslatorSchema = z.object({
  translator_id: z.number().int(),
  translator_type: TranslatorTypeSchema,
  formula: FormulaSchema,
});

/* -------------------------
   ROOT SCHEMA
------------------------- */

export const SchemaSchema = z.object({
  name: z.string(),
  version: z.number().int(),
  description: z.string(),

  registers: z.array(RegisterSchema),

  interpreters: z.array(InterpreterSchema),
  translator: z.array(TranslatorSchema),
});


const schema = new ZodBridge({ schema: SchemaSchema });

function RegisterRow(props: any) {
  const [field] = useField(props.name, props);

  const register = field.value || {};

  return (
    <>
      <div style={{ padding: 8, background: "#eee", marginBottom: 8 }}>
        Current Register:
        {register.name} @ {register.address}
      </div>

      <AutoField name={`${props.name}.register_id`} />
      <AutoField name={`${props.name}.name`} />
      <AutoField name={`${props.name}.address`} />
      <AutoField name={`${props.name}.description`} />
    </>
  );
}

export default function App() {
  return (
    <AutoForm
      schema={schema}
      onSubmit={(model: any) => window.alert(JSON.stringify(model))}
    />
  );
}
