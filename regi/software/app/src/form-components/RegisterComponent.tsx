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
import { useMemo, useRef, useState } from "react";
import { MenuButtonBold, MenuButtonItalic, MenuControlsContainer, MenuDivider, MenuSelectHeading, RichTextEditor, RichTextEditorRef } from "mui-tiptap";
import { IResetValue, resetValCaster, ResetValueType } from "../validators/hex-validator";
import { RegisterSchemaBase } from "../schema/register";
import { BitType, bitTypes } from "../schema/bits";
import { MockType } from "../schema/mock";
import { useRegisterStore } from "../stores/register-store";
import { InterpreterComponent } from "./register-component/InterpretersComponent";

type RegisterType = z.infer<typeof RegisterSchemaBase>;
type RegisterFieldProps = HTMLFieldProps<RegisterType, HTMLDivElement>;

type FormulaOrDescription = 'formula' | 'description'

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

    const value = resetValCaster.cast(resVal) as IResetValue | undefined

    if (value?.isValid && value.valType === ResetValueType.Hex) {
      const binaryVal = toBin(value.value)
      const hexVal = toHex(value.value)
      console.log("binary value:", binaryVal, " hex value: ", hexVal)
    }

    setRegisterValue(register.register_id, bitId, resVal)
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

      {register && registerInterpreter && (
        <InterpreterComponent
          register={register}
          interpreters={registerInterpreter}
          onAddInterpreter={() => addNewInterpreter(register.register_id)}
          onRemoveInterpreter={(interpreterId) => removeInterpreter(interpreterId)}
          onUpdateInterpreterRegisterBit={(interpreterId, registerId, bitId) => updateInterpreterBits(interpreterId, registerId, bitId)}
          onChangeInterpreterName={(interpreterId, name) => updateInterpreterNameOrDescription(interpreterId, 'name', name)}
          onSaveInterpreterDescription={(interpreterId, description) => updateInterpreterNameOrDescription(interpreterId, 'description', description)}
        />
      )}

    </Stack>
  )
}


//      <Tabs
//        value={selectedTab}
//        onChange={(_, value) => setSelectedTab(value as FormulaOrDescription)}
//      >
//        <Tab
//          label="Description"
//          value={'description' as FormulaOrDescription}
//        />
//        <Tab
//          label="Formula"
//          value={'formula' as FormulaOrDescription}
//        />
//      </Tabs>
//
//      {selectedTab === 'description' && (
//        <RichTextEditor
//          extensions={[StarterKit]}
//          content={value.description}
//          renderControls={() => (
//            <MenuControlsContainer>
//              <MenuSelectHeading />
//              <MenuDivider />
//              <MenuButtonBold />
//              <MenuButtonItalic />
//            </MenuControlsContainer>
//          )}
//        />
//      )}
//
//      {selectedTab === 'formula' && (
//        <>
//          <Typography>
//            Available variable: {' '}
//            <code>bit0</code>{', '}
//            <code>bit1</code>{', '}
//            <code>bit2</code>{', '}
//            <code>bit3</code>{', '}
//            <code>bit4</code>{', '}
//            <code>bit5</code>{', '}
//            <code>bit6</code>{', '}
//            <code>bit7</code>
//          </Typography>
//
//          <TextField
//            id="formula-input"
//            label="Formula"
//            size="medium"
//            variant="standard"
//            fullWidth
//            multiline
//            rows={4}
//            onBlur={(ev) => console.log(math.evaluate(ev.target.value, {
//              bit0: 1,
//              bit1: 1,
//              bit2: 1,
//              bit3: 1,
//              bit4: 1,
//              bit5: 1,
//              bit6: 1,
//              bit7: 1,
//            }))}
//          />
//        </>
//      )}
