import z from "zod";
import { InterpreterSchema } from "./interpreter";
import { TranslatorSchema } from "./translator";
import AddRegisterComponent from "../form-components/AddRegisterComponent";
import { RegisterSchema, RegisterSchemaBase } from "./register";
import 'uniforms-bridge-zod'

export const SchemaBase = z.object({
  name: z.string(),
  version: z.number().int(),
  description: z.string(),

  registers: z.array(RegisterSchemaBase),

  interpreters: z.array(InterpreterSchema),
  translator: z.array(TranslatorSchema),
});

export const Schema = z.object({
  name: z.string(),
  version: z.number().int(),
  description: z.string(),

  registers: z.array(RegisterSchema).uniforms({ component: AddRegisterComponent }),

  interpreters: z.array(InterpreterSchema),
  translator: z.array(TranslatorSchema),
});

export type SchemaType = z.infer<typeof Schema>;
export type SchemaTypeBase = z.infer<typeof SchemaBase>;
export type SchemaKeys = keyof SchemaType;
