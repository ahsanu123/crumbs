import { HTMLFieldProps, useField } from "uniforms";
import { z } from 'zod';
import { bitTypes, RegisterSchema } from "../schema";
import { Button, MenuItem, Stack, TextField } from "@mui/material";
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import { produce } from "immer";

type RegisterType = z.infer<typeof RegisterSchema>;
type RegisterBitsType = RegisterType['bits']
type RegisterFieldProps = HTMLFieldProps<RegisterType[], HTMLDivElement>;


function createData(
  name: string,
  calories: number,
  fat: number,
  carbs: number,
  protein: number,
) {
  return { name, calories, fat, carbs, protein };
}

export default function RegisterField(props: RegisterFieldProps) {
  const [fieldProps, context] = useField(props.name, props);

  const { onChange: onRegisterChange, value: registerValues } = fieldProps;

  const handleOnAddNewRegister = () => {

    const eightEmptyBits: RegisterBitsType = [
      {
        bit_id: 0,
        bit_type: "WriteOnly",
        description: "",
        reset_val: 0,
        bit_ordinal: 0,
      },
      {
        bit_id: 1,
        bit_type: "ReadOnly",
        description: "",
        reset_val: 0,
        bit_ordinal: 0,
      },
      {
        bit_id: 2,
        bit_type: "ReadOnly",
        description: "",
        reset_val: 0,
        bit_ordinal: 0,
      },
      {
        bit_id: 3,
        bit_type: "ReadOnly",
        description: "",
        reset_val: 0,
        bit_ordinal: 0,
      },
      {
        bit_id: 4,
        bit_type: "ReadOnly",
        description: "",
        reset_val: 0,
        bit_ordinal: 0,
      },
      {
        bit_id: 5,
        bit_type: "ReadOnly",
        description: "",
        reset_val: 0,
        bit_ordinal: 0,
      },
      {
        bit_id: 6,
        bit_type: "ReadOnly",
        description: "",
        reset_val: 0,
        bit_ordinal: 0,
      },
      {
        bit_id: 7,
        bit_type: "WriteOnly",
        description: "",
        reset_val: 0,
        bit_ordinal: 0,
      },
    ]

    const emptyRegister: RegisterType = {
      register_id: registerValues?.length ? registerValues.length + 1 : 0,
      name: "",
      address: 0,
      bits: eightEmptyBits,
      description: ""
    }

    if (registerValues === undefined) {
      onRegisterChange([emptyRegister])
    }
    else {
      onRegisterChange([
        ...registerValues,
        emptyRegister
      ])
    }
  }

  const handleOnRegisterNameChange = (regId: number, name: string) => {

    const updatedValue = produce(registerValues ?? [], (draft) => {
      const register = draft.find(reg => reg.register_id === regId);

      if (!register) return;
      register.name = name;
    })

    onRegisterChange(updatedValue)
  }

  const handleOnRegisterDescriptionChange = (regId: number, description: string) => {

    const updatedValue = produce(registerValues ?? [], (draft) => {
      const register = draft.find(reg => reg.register_id === regId);

      if (!register) return;
      register.description = description;
    })

    onRegisterChange(updatedValue)
  }

  const registerTable = (register: RegisterType) => (
    <TableContainer>
      <Table>
        <TableHead>
          <TableRow>
            <TableCell>Reg Num</TableCell>
            {Array.from({ length: 8 }).map((_, index) => index).reverse().map((num) => (
              <TableCell align="center">{num}</TableCell>
            ))}
          </TableRow>
        </TableHead>
        <TableBody>
          <TableRow>
            <TableCell>Reg Type</TableCell>
            {register.bits.slice()
              .sort((a, b) => a.bit_ordinal - b.bit_ordinal)
              .map((bit, index) => (
                <TableCell>
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
                <TableCell>
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

  return (
    <Stack>

      {registerValues?.map((register) => (
        <>
          <TextField
            id="register-name"
            label="Register Name"
            size="medium"
            variant="standard"
            sx={{ marginTop: 10 }}
            fullWidth
            value={register.name}
            onChange={(ev) => handleOnRegisterNameChange(register.register_id, ev.target.value)}
          />

          <TextField
            id="register-description"
            label="Register Description"
            placeholder="Enter Register Description"
            rows={4}
            sx={{ marginTop: 5 }}
            multiline
            fullWidth
            value={register.description}
            onChange={(ev) => handleOnRegisterDescriptionChange(register.register_id, ev.target.value)}
          />

          {registerTable(register)}

        </>
      ))}


      <Button
        variant="contained"
        onClick={() => handleOnAddNewRegister()}
      >
        Add New Register
      </Button>

    </Stack>
  )
}
