import { HTMLFieldProps, useField } from "uniforms";
import { z } from 'zod';
import { math, toBin, toHex } from '../utility/math'
import StarterKit from "@tiptap/starter-kit";
import { Button, MenuItem, Stack, Tab, Tabs, TextField, Typography } from "@mui/material";
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import { useEffect, useRef, useState } from "react";
import { useImmer } from "use-immer";
import { MenuButtonBold, MenuButtonItalic, MenuControlsContainer, MenuDivider, MenuSelectHeading, RichTextEditor, RichTextEditorRef } from "mui-tiptap";
import { produce } from "immer";
import { IResetValue, resetValCaster, ResetValueType } from "../validators/hex-validator";
import { RegisterSchemaBase } from "../schema/register";
import { BitType, bitTypes } from "../schema/bits";
import { MockType } from "../schema/mock";

type RegisterType = z.infer<typeof RegisterSchemaBase>;
type RegisterFieldProps = HTMLFieldProps<RegisterType, HTMLDivElement>;

type FormulaOrDescription = 'formula' | 'description'

export default function RegisterField(props: RegisterFieldProps) {

  const rteRef = useRef<RichTextEditorRef>(null);

  const [fieldProps, context] = useField(props.name, props);

  const [value, setValue] = useImmer<RegisterType | undefined>(undefined)
  const [selectedTab, setSelectedTab] = useState<FormulaOrDescription>('description');
  const [equationError, setEquationError] = useState<string | undefined>(undefined);

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

  const handleOnSaveRegister = () => {
    const description = rteRef.current?.editor?.getHTML();

    if (value === undefined) return;

    const updatedVal = produce(value, draft => {
      draft.description = description ?? ""
    })

    onRegisterChange(updatedVal)
  }

  const handleOnRegisterTypeChange = (bitId: string, bitType: BitType) => {
    setValue(draft => {
      const bit = draft?.bits.find((bit) => bit.bit_id === bitId)
      if (!bit) return;

      bit.bit_type = bitType
    })
  }

  const handleOnRegisterResValChange = (bitId: string, resVal: string) => {
    setValue(draft => {
      const bit = draft?.bits.find((bit) => bit.bit_id === bitId)
      if (!bit) return;

      const value = resetValCaster.cast(resVal) as IResetValue | undefined

      if (value?.isValid && value.valType === ResetValueType.Hex) {
        const binaryVal = toBin(value.value)
        const hexVal = toHex(value.value)
        console.log("binary value:", binaryVal, " hex value: ", hexVal)
      }
      bit.reset_val = resVal
    })
  }

  const renderMockTableInput = (mock: MockType) => {
    return (
      <TableContainer>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>
                <b>Mock Data {value?.name}</b>
              </TableCell>

              <TableCell>Reset Value</TableCell>
              {mock.datas.slice()
                .map((bit, index) => (
                  <TableCell key={index}>
                    <TextField
                      fullWidth
                      id={`bits-resval-${index}`}
                      label="ResVal"
                      type="text"
                      value={bit.value}
                      onChange={(e) => handleOnRegisterResValChange(bit.bit_id, e.target.value)}
                    />
                  </TableCell>
                ))}
            </TableRow>
          </TableHead>
        </Table>
      </TableContainer>
    )
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
                    type="text"
                    value={bit.reset_val}
                    onChange={(e) => handleOnRegisterResValChange(bit.bit_id, e.target.value)}
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
      id={`register-${value.name.replace(' ', '-')}`}
      sx={{
        marginTop: 5,
        marginLeft: 0,
        border: 'solid 1px gray',
        borderRadius: '10px',
        padding: '25px'
      }}
    >

      <Button
        variant="contained"
        onClick={() => handleOnSaveRegister()}
      >
        Save {value.name}
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
        }}
      />

      {renderRegisterTable(value)}

      <Tabs
        value={selectedTab}
        onChange={(_, value) => setSelectedTab(value as FormulaOrDescription)}
      >
        <Tab
          label="Description"
          value={'description' as FormulaOrDescription}
        />
        <Tab
          label="Formula"
          value={'formula' as FormulaOrDescription}
        />
      </Tabs>

      {selectedTab === 'description' && (
        <RichTextEditor
          extensions={[StarterKit]}
          content={value.description}
          renderControls={() => (
            <MenuControlsContainer>
              <MenuSelectHeading />
              <MenuDivider />
              <MenuButtonBold />
              <MenuButtonItalic />
            </MenuControlsContainer>
          )}
        />
      )}

      {selectedTab === 'formula' && (
        <>
          <Typography>
            Available variable: {' '}
            <code>bit0</code>{', '}
            <code>bit1</code>{', '}
            <code>bit2</code>{', '}
            <code>bit3</code>{', '}
            <code>bit4</code>{', '}
            <code>bit5</code>{', '}
            <code>bit6</code>{', '}
            <code>bit7</code>
          </Typography>

          <TextField
            id="formula-input"
            label="Formula"
            size="medium"
            variant="standard"
            fullWidth
            multiline
            rows={4}
            onBlur={(ev) => console.log(math.evaluate(ev.target.value, {
              bit0: 1,
              bit1: 1,
              bit2: 1,
              bit3: 1,
              bit4: 1,
              bit5: 1,
              bit6: 1,
              bit7: 1,
            }))}
          />
        </>
      )}

    </Stack>
  )
}
