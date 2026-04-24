import { Button } from "@mui/material";
import { HTMLFieldProps, useField } from "uniforms";
import z from "zod";
import { RegisterSchemaBase } from "../schema";
import { ListItemField } from "uniforms-mui";

type RegisterType = z.infer<typeof RegisterSchemaBase>;
type RegisterBitsType = RegisterType['bits']
type RegisterFieldProps = HTMLFieldProps<RegisterType[], HTMLDivElement>;

export default function AddRegisterComponent(props: RegisterFieldProps) {
  const [fieldProps, context] = useField(props.name, props);

  const { onChange, value } = fieldProps;

  const handleOnAddNewRegister = () => {

    const eightEmptyBits: RegisterBitsType = [
      {
        bit_id: "0",
        bit_type: "WriteOnly",
        description: "",
        reset_val: 0,
        bit_ordinal: 0,
      },
      {
        bit_id: "1",
        bit_type: "ReadOnly",
        description: "",
        reset_val: 0,
        bit_ordinal: 0,
      },
      {
        bit_id: "2",
        bit_type: "ReadOnly",
        description: "",
        reset_val: 0,
        bit_ordinal: 0,
      },
      {
        bit_id: "3",
        bit_type: "ReadOnly",
        description: "",
        reset_val: 0,
        bit_ordinal: 0,
      },
      {
        bit_id: "4",
        bit_type: "ReadOnly",
        description: "",
        reset_val: 0,
        bit_ordinal: 0,
      },
      {
        bit_id: "5",
        bit_type: "ReadOnly",
        description: "",
        reset_val: 0,
        bit_ordinal: 0,
      },
      {
        bit_id: "6",
        bit_type: "ReadOnly",
        description: "",
        reset_val: 0,
        bit_ordinal: 0,
      },
      {
        bit_id: "7",
        bit_type: "WriteOnly",
        description: "",
        reset_val: 0,
        bit_ordinal: 0,
      },
    ]

    const emptyRegister: RegisterType = {
      register_id: "0",
      name: "",
      address: 0,
      bits: eightEmptyBits,
      description: ""
    }

    if (value === undefined) {
      onChange([emptyRegister])
    }
    else {
      onChange([
        ...value,
        emptyRegister
      ])
    }
  }

  return (
    <>
      <Button
        variant="contained"
        onClick={() => handleOnAddNewRegister()}
      >
        Add New Register
      </Button>


      {value?.map((_, index) => (
        <ListItemField
          key={index}
          name={`${props.name}.${index}`}
        />
      ))}

    </>
  );
}
