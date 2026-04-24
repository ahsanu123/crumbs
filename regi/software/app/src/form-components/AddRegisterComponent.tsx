import { Button } from "@mui/material";
import { useField } from "uniforms";
import { ListItemField } from "uniforms-mui";
import { RegisterFieldProps } from "../schema/register";
import { emptyRegister } from "../empty-objects/register";

export default function AddRegisterComponent(props: RegisterFieldProps) {
  const [fieldProps, context] = useField(props.name, props);

  const { onChange, value } = fieldProps;

  const handleOnAddNewRegister = () => {
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
      {value?.map((_, index) => (
        <ListItemField
          key={index}
          name={`${props.name}.${index}`}
        />
      ))}

      <Button
        variant="contained"
        onClick={() => handleOnAddNewRegister()}
      >
        Add New Register
      </Button>

    </>
  );
}
