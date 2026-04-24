import z from "zod";
import { BitSchema } from "./bits";
import RegisterField from "../form-components/RegisterComponent";
import { HTMLFieldProps } from "uniforms";

export const RegisterSchemaBase = z.object({
  register_id: z.string(),
  name: z.string(),
  description: z.string(),
  address: z.number().int().nonnegative(),
  bits: z.array(BitSchema),
  interpreterIds: z.array(z.number().int()),
});

export const RegisterSchema = RegisterSchemaBase.uniforms({ component: RegisterField });

export type RegisterType = z.infer<typeof RegisterSchemaBase>;
export type RegisterBitsType = RegisterType['bits']
export type RegisterFieldProps = HTMLFieldProps<RegisterType[], HTMLDivElement>;
