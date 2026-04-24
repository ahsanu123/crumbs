import { HTMLFieldProps, useField } from "uniforms";
import { z } from 'zod';
import { bitTypes, RegisterSchema, RegisterSchemaBase } from "../schema";
import { Button, MenuItem, Stack, TextField } from "@mui/material";
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import { useEffect } from "react";
import { useImmer } from "use-immer";

type RegisterType = z.infer<typeof RegisterSchemaBase>;
type RegisterBitsType = RegisterType['bits']
type RegisterFieldProps = HTMLFieldProps<RegisterType, HTMLDivElement>;


export default function RegisterField(props: RegisterFieldProps) {
  const [fieldProps, context] = useField(props.name, props);

  const [value, setValue] = useImmer<RegisterType | undefined>(undefined)

  const { onChange: onRegisterChange, value: registerValues } = fieldProps;

  useEffect(() => {
    if (registerValues !== undefined)
      setValue(registerValues)
  }, [])

  const handleOnRegisterNameChange = (name: string) => {
    setValue((draft) => {
      if (draft === undefined) return;
      draft.name = name;
    })
  }

  const handleOnRegisterDescriptionChange = (description: string) => {
    setValue((draft) => {
      if (draft === undefined) return;
      draft.description = description;
    })
  }

  const handleOnSaveRegister = () => {
    onRegisterChange(value)
  }

  const renderRegisterTable = (register: RegisterType) => (
    <TableContainer>
      <Table>
        <TableHead>
          <TableRow>
            <TableCell>Reg Num</TableCell>
            {Array.from({ length: 8 }).map((_, index) => index).reverse().map((num) => (
              <TableCell
                align="center"
                key={num}
              >
                {num}
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
                    value={bit.reset_val}
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
    <Stack>

      <Button
        variant="contained"
        onClick={() => handleOnSaveRegister()}
      >
        Save
      </Button>

      <TextField
        id="register-name"
        label="Register Name"
        size="medium"
        variant="standard"
        sx={{ marginTop: 10 }}
        fullWidth
        value={value.name}
        onChange={(ev) => handleOnRegisterNameChange(ev.target.value)}
      />

      <TextField
        id="register-description"
        label="Register Description"
        placeholder="Enter Register Description"
        rows={4}
        sx={{ marginTop: 5 }}
        multiline
        fullWidth
        value={value.description}
        onChange={(ev) => handleOnRegisterDescriptionChange(ev.target.value)}
      />

      {renderRegisterTable(value)}

    </Stack>
  )
}
