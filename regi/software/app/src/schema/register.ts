import { BitSchema } from "./bits";

export interface RegisterSchema {
  register_id: number,
  ordinal: number,
  name: string,
  description: string,
  address: string,
  bits: BitSchema[],
}
