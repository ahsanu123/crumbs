import { Button } from "@mui/material";
import { useField } from "uniforms";
import { ListItemField } from "uniforms-mui";
import { RegisterFieldProps } from "../schema/register";
import { useRegisterStore } from "../stores/register-store";
import { useMemo } from "react";
import { CombinedRegisterComponent } from "./CombinedRegisterComponent";

export default function AddRegisterComponent(props: RegisterFieldProps) {
  const [fieldProps, context] = useField(props.name, props);

  const combinedRegisters = useRegisterStore(store => store.combinedRegister)
  const addNewRegister = useRegisterStore(store => store.addNewRegister)
  const addNewCombinedRegister = useRegisterStore(store => store.addNewCombinedRegister)

  const allCombinedRegisterIds = useMemo(() => {
    return combinedRegisters.map((reg) => reg.registers.map((r) => r.register_id).flat()).flat()
  }, [combinedRegisters])

  // const formContext = useForm<SchemaType>()
  // console.log("formContext", formContext)
  // formContext.onChange(key, value)

  const { onChange, value } = fieldProps;

  const handleOnCreateNewRegister = () => {
    const updatedRegisters = addNewRegister()
    onChange(updatedRegisters)
  }

  return (
    <>
      <Button
        variant="contained"
        onClick={() => addNewCombinedRegister()}
      >
        Create Combined Register
      </Button>

      {combinedRegisters.map((combinedRegister) => (
        <CombinedRegisterComponent
          combinedRegister={combinedRegister}
        />
      ))}

      {value?.
        map((reg, index) => {
          if (allCombinedRegisterIds.includes(reg.register_id)) return null

          return (
            <ListItemField
              key={index}
              name={`${props.name}.${index}`}
            />
          )
        })}

      <Button
        variant="contained"
        onClick={() => handleOnCreateNewRegister()}
      >
        Add New Register
      </Button>

    </>
  );
}
