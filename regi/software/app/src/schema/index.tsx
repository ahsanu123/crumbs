import z from "zod";
import { InterpreterSchema } from "./interpreter";
import { TranslatorSchema } from "./translator";
import RegisterField from "../form-components/RegisterField";
import { LongTextField } from "uniforms-mui";
import 'uniforms-bridge-zod'
import AddRegisterComponent from "../form-components/AddRegisterComponent";

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
  bit_id: z.string(),
  bit_ordinal: z.number().int().min(0).max(31),
  bit_type: BitTypeSchema,
  reset_val: z.number().int().nonnegative(),
  description: z.string()
});

export const RegisterSchemaBase = z.object({
  register_id: z.string(),
  name: z.string(),
  description: z.string(),
  address: z.number().int().nonnegative(),
  bits: z.array(BitSchema),
});

export const RegisterSchema = RegisterSchemaBase.uniforms({ component: RegisterField });

export const SchemaSchema = z.object({
  name: z.string(),
  version: z.number().int(),
  description: z.string(),

  registers: z.array(RegisterSchema).uniforms({ component: AddRegisterComponent }),

  interpreters: z.array(InterpreterSchema),
  translator: z.array(TranslatorSchema),
});
