import { RegisterBitsType, RegisterType } from "../schema/register"

export const eightEmptyBits: RegisterBitsType = [
  {
    bit_id: 0,
    bit_type: "WriteOnly",
    description: "",
    reset_val: "0",
    bit_ordinal: 0,
  },
  {
    bit_type: "ReadOnly",
    description: "",
    reset_val: "0",
    bit_ordinal: 0,
    bit_id: 1
  },
  {
    bit_type: "ReadOnly",
    description: "",
    reset_val: "0",
    bit_ordinal: 0,
    bit_id: 2
  },
  {
    bit_type: "ReadOnly",
    description: "",
    reset_val: "0",
    bit_ordinal: 0,
    bit_id: 3
  },
  {
    bit_type: "ReadOnly",
    description: "",
    reset_val: "0",
    bit_ordinal: 0,
    bit_id: 4
  },
  {
    bit_type: "ReadOnly",
    description: "",
    reset_val: "0",
    bit_ordinal: 0,
    bit_id: 5
  },
  {
    bit_type: "ReadOnly",
    description: "",
    reset_val: "0",
    bit_ordinal: 0,
    bit_id: 6
  },
  {
    bit_type: "WriteOnly",
    description: "",
    reset_val: "0",
    bit_ordinal: 0,
    bit_id: 7
  },
]

export const emptyRegister: RegisterType = {
  register_id: 0,
  name: "",
  address: 0,
  bits: eightEmptyBits,
  description: "",
  interpreterIds: [],
  ordinal: 0
}
