import { HTMLFieldProps, useField } from "uniforms";
import { IoSave } from "react-icons/io5";
import { z } from 'zod';
import { Button, MenuItem, Stack, TextField } from "@mui/material";
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import { useMemo, useRef } from "react";
import { RichTextEditorRef } from "mui-tiptap";
import { RegisterSchemaBase } from "../schema/register";
import { BitType, bitTypes } from "../schema/bits";
import { useRegisterStore } from "../stores/register-store";
import { InterpreterComponent } from "./register-component/InterpretersComponent";

type RegisterType = z.infer<typeof RegisterSchemaBase>;
type RegisterFieldProps = HTMLFieldProps<RegisterType, HTMLDivElement>;

export default function RegisterField(props: RegisterFieldProps) {

  const rteRef = useRef<RichTextEditorRef>(null);

  const [fieldProps, context] = useField(props.name, props);

  const vals = useRegisterStore(store => store.registers)

  const setValue = useRegisterStore(store => store.updateRegister)
  const setRegisterType = useRegisterStore(store => store.updateRegisterBitType)
  const setRegisterValue = useRegisterStore(store => store.updateRegisterBitResValue)

  const interpreters = useRegisterStore(store => store.interpreters)
  const addNewInterpreter = useRegisterStore(store => store.addNewInterpreter)
  const removeInterpreter = useRegisterStore(store => store.removeInterpreter)
  const updateInterpreterBits = useRegisterStore(store => store.updateInterpreterBits)
  const updateInterpreterNameOrDescription = useRegisterStore(store => store.updateInterpreterNameOrDescription)
  const updateInterpreterType = useRegisterStore(store => store.updateInterpreterType)
  const updateInterpreterFormula = useRegisterStore(store => store.updateInterpreterFormula)

  const { onChange: onRegisterChange, value: register } = fieldProps;

  const registerInterpreter = useMemo(() => {
    if (!register) return undefined
    return interpreters.filter(int => int.registers.map(reg => reg.register_id).includes(register.register_id))
  }, [interpreters, register])

  const value = useMemo(() => {
    return vals.find(reg => reg.register_id === register?.register_id)
  }, [vals])

  const handleOnRegisterNameChange = (name: string) => {
    if (!register) return;
    setValue(register.register_id, 'name', name)
  }

  const handleOnRegisterAddressChange = (address: string) => {
    if (!register) return;
    setValue(register.register_id, 'address', address)
  }

  const handleOnSaveRegister = () => {
    const description = rteRef.current?.editor?.getHTML();

    if (!value || !register) return;

    const updatedVal = setValue(register.register_id, 'description', description ?? "")
    if (!updatedVal) return

    onRegisterChange(updatedVal)
  }

  const handleOnRegisterTypeChange = (bitId: number, bitType: BitType) => {
    if (!register) return;
    setRegisterType(register.register_id, bitId, bitType)
  }

  const handleOnRegisterResValChange = (bitId: number, resVal: string) => {
    if (!register) return;

    setRegisterValue(register.register_id, bitId, resVal)
  }

  const renderRegisterTable = (register: RegisterType) => (
    <TableContainer>
      <Table>
        <TableHead>
          <TableRow>
            <TableCell>
              <b>Reg Num</b>
            </TableCell>
            {Array.from({ length: 8 }).map((_, index) => index).reverse().map((num) => (
              <TableCell
                align="center"
                key={num}
              >
                <b>
                  {num}
                </b>
              </TableCell>
            ))}
          </TableRow>
        </TableHead>
        <TableBody>
          <TableRow>
            <TableCell>Reg Type</TableCell>
            {register.bits.slice()
              .sort((a, b) => a.bit_ordinal - b.bit_ordinal)
              .map((bit, index) => (
                <TableCell key={index}>
                  <TextField
                    select
                    id={`bits-select-${index}`}
                    label="Type"
                    value={bit.bit_type}
                    fullWidth
                    onChange={(e) => handleOnRegisterTypeChange(bit.bit_id, e.target.value as BitType)}
                  >
                    {bitTypes.map((bitType, index) => (
                      <MenuItem
                        key={index}
                        value={bitType}
                      >
                        {bitType}
                      </MenuItem>
                    ))}
                  </TextField>
                </TableCell>
              ))}
          </TableRow>

          <TableRow>
            <TableCell>Reset Value</TableCell>
            {register.bits.slice()
              .sort((a, b) => a.bit_ordinal - b.bit_ordinal)
              .map((bit, index) => (
                <TableCell key={index}>
                  <TextField
                    fullWidth
                    id={`bits-resval-${index}`}
                    label="ResVal"
                    type="number"
                    value={bit.reset_val}
                    onChange={(e) => handleOnRegisterResValChange(bit.bit_id, e.target.value)}
                    slotProps={{
                      htmlInput: { min: 0, max: 1, step: 1 }
                    }}
                  />
                </TableCell>
              ))}
          </TableRow>

        </TableBody>
      </Table>
    </TableContainer>
  )

  if (value === undefined) return null

  return (
    <Stack
      id={`register-${value.name?.replace(' ', '-')}`}
      sx={{
        marginTop: 5,
        marginLeft: 0,
        border: 'solid 1px gray',
        borderRadius: '10px',
        padding: '25px',
        width: "100%"
      }}
    >

      <Button
        variant="contained"
        onClick={() => handleOnSaveRegister()}
        sx={{
          maxWidth: "500px",
        }}
        startIcon={<IoSave />}
      >
        Save
      </Button>

      <TextField
        id="register-name"
        label="Register Name"
        size="medium"
        variant="standard"
        fullWidth
        value={value.name}
        onChange={(ev) => handleOnRegisterNameChange(ev.target.value)}
        sx={{
          marginTop: 5,
          maxWidth: "50%"
        }}
      />

      <TextField
        id="register-address"
        label="Register Address"
        size="medium"
        variant="standard"
        fullWidth
        value={value.address}
        onChange={(ev) => handleOnRegisterAddressChange(ev.target.value)}
        sx={{
          marginTop: 5,
          maxWidth: "50%"
        }}
      />

      {renderRegisterTable(value)}

      {register && registerInterpreter && (
        <InterpreterComponent
          register={register}
          interpreters={registerInterpreter}
          onAddInterpreter={() => addNewInterpreter(register.register_id)}
          onRemoveInterpreter={(interpreterId) => removeInterpreter(interpreterId)}
          onUpdateInterpreterRegisterBit={(interpreterId, registerId, bitId) => updateInterpreterBits(interpreterId, registerId, bitId)}
          onChangeInterpreterName={(interpreterId, name) => updateInterpreterNameOrDescription(interpreterId, 'name', name)}
          onSaveInterpreterDescription={(interpreterId, description) => updateInterpreterNameOrDescription(interpreterId, 'description', description)}
          onUpdateInterpreterType={(interpreterId, type) => updateInterpreterType(interpreterId, type)}
          onUpdateInterpreterFormula={(interpreterId, formula) => updateInterpreterFormula(interpreterId, formula)}
        />
      )}

    </Stack>
  )
}
