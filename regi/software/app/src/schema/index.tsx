import z from "zod";
import { InterpreterSchema } from "./interpreter";
import { TranslatorSchema } from "./translator";
import RegisterField from "../form-components/RegisterField";
import { LongTextField } from "uniforms-mui";
import 'uniforms-bridge-zod'

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
  reset_val: z.number().int().nonnegative(),
  description: z.string()
});

export const RegisterSchema = z.object({
  register_id: z.number().int(),
  name: z.string(),
  description: z.string(),
  address: z.number().int().nonnegative(),
  bits: z.array(BitSchema),
});

export const SchemaSchema = z.object({
  name: z.string(),
  version: z.number().int(),
  description: z.string(),

  registers: z.array(RegisterSchema).uniforms({ component: RegisterField }),

  interpreters: z.array(InterpreterSchema),
  translator: z.array(TranslatorSchema),
});
