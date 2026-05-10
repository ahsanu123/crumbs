import { BitSchema, BitType } from "../schema/bits"
import { RegisterSchema } from "../schema/register"

export const eightEmptyBits: BitSchema[] = [
  {
    bit_id: 0,
    bit_type: BitType.ReadOnly,
    description: "",
    reset_val: "0",
    bit_ordinal: 0,
  },
  {
    bit_type: BitType.ReadOnly,
    description: "",
    reset_val: "0",
    bit_ordinal: 1,
    bit_id: 1
  },
  {
    bit_type: BitType.ReadOnly,
    description: "",
    reset_val: "0",
    bit_ordinal: 2,
    bit_id: 2
  },
  {
    bit_type: BitType.ReadOnly,
    description: "",
    reset_val: "0",
    bit_ordinal: 3,
    bit_id: 3
  },
  {
    bit_type: BitType.ReadOnly,
    description: "",
    reset_val: "0",
    bit_ordinal: 4,
    bit_id: 4
  },
  {
    bit_type: BitType.ReadOnly,
    description: "",
    reset_val: "0",
    bit_ordinal: 5,
    bit_id: 5
  },
  {
    bit_type: BitType.ReadOnly,
    description: "",
    reset_val: "0",
    bit_ordinal: 6,
    bit_id: 6
  },
  {
    bit_type: BitType.ReadOnly,
    description: "",
    reset_val: "0",
    bit_ordinal: 7,
    bit_id: 7
  },
]

export const emptyRegister: RegisterSchema = {
  register_id: 0,
  name: "",
  address: "0",
  bits: eightEmptyBits,
  description: "",
  ordinal: 0,
}
